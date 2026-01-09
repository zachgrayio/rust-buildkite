use crate::services::*;
use std::sync::Arc;
use url::Url;

const DEFAULT_BASE_URL: &str = "https://api.buildkite.com/";
const USER_AGENT: &str = concat!("rust-buildkite/", env!("CARGO_PKG_VERSION"));

/// Extension trait for reqwest::Response to add Buildkite-specific error handling
#[async_trait::async_trait]
pub trait ResponseExt {
    /// Check response for Buildkite API errors and convert them to BuildkiteError
    async fn buildkite_error_for_status(
        self,
    ) -> Result<reqwest::Response, crate::error::BuildkiteError>;
}

#[async_trait::async_trait]
impl ResponseExt for reqwest::Response {
    async fn buildkite_error_for_status(
        self,
    ) -> Result<reqwest::Response, crate::error::BuildkiteError> {
        let status = self.status();

        if status.is_success() {
            return Ok(self);
        }

        let url = self.url().to_string();
        let status_code = status.as_u16();
        let body_bytes = self.bytes().await?;
        let raw_body = body_bytes.to_vec();

        let message = if let Ok(error_response) =
            serde_json::from_slice::<crate::error::ErrorResponse>(&body_bytes)
        {
            error_response.message
        } else {
            String::from_utf8(raw_body.clone()).unwrap_or_else(|_| format!("HTTP {}", status_code))
        };

        Err(crate::error::BuildkiteError {
            status_code: Some(status_code),
            message,
            url: Some(url),
            method: None,
            raw_body: Some(raw_body),
        })
    }
}

/// Response wrapper with pagination information
#[derive(Debug, Clone)]
pub struct Response<T> {
    pub data: T,
    pub next_page: Option<u32>,
    pub prev_page: Option<u32>,
    pub first_page: Option<u32>,
    pub last_page: Option<u32>,
}

impl<T> Response<T> {
    /// Create a response with just data (no pagination)
    pub fn new(data: T) -> Self {
        Self {
            data,
            next_page: None,
            prev_page: None,
            first_page: None,
            last_page: None,
        }
    }

    /// Parse Link header for pagination
    pub fn with_pagination(data: T, link_header: Option<&str>) -> Self {
        let (first_page, prev_page, next_page, last_page) = link_header
            .map(Self::parse_link_header)
            .unwrap_or((None, None, None, None));

        Self {
            data,
            next_page,
            prev_page,
            first_page,
            last_page,
        }
    }

    fn parse_link_header(
        header_value: &str,
    ) -> (Option<u32>, Option<u32>, Option<u32>, Option<u32>) {
        let mut first_page = None;
        let mut prev_page = None;
        let mut next_page = None;
        let mut last_page = None;

        for link in header_value.split(',') {
            let parts: Vec<&str> = link.split(';').map(|s| s.trim()).collect();
            if parts.len() < 2 {
                continue;
            }

            let Some(url_part) = parts.first() else {
                continue;
            };
            let url_part = url_part.trim_start_matches('<').trim_end_matches('>');

            if let Ok(url) = Url::parse(url_part)
                && let Some(page_str) = url.query_pairs().find(|(k, _)| k == "page").map(|(_, v)| v)
                && let Ok(page) = page_str.parse::<u32>()
            {
                for part in parts.get(1..).unwrap_or(&[]) {
                    match part.trim() {
                        "rel=\"first\"" => first_page = Some(page),
                        "rel=\"prev\"" => prev_page = Some(page),
                        "rel=\"next\"" => next_page = Some(page),
                        "rel=\"last\"" => last_page = Some(page),
                        _ => {}
                    }
                }
            }
        }

        (first_page, prev_page, next_page, last_page)
    }
}

pub(crate) struct ClientInner {
    pub http_client: reqwest::Client,
    pub base_url: String,
    pub token: String,
    pub user_agent: String,
    pub http_debug: bool,
}

impl ClientInner {
    fn resolve_url(&self, rel_path: &str) -> Result<Url, url::ParseError> {
        let rel = Url::parse(rel_path);

        if let Ok(url) = &rel
            && (url.scheme() == "http" || url.scheme() == "https")
        {
            return Ok(url.clone());
        }

        let rel_parsed = match rel {
            Ok(url) => url,
            Err(url::ParseError::RelativeUrlWithoutBase) => {
                if rel_path.starts_with("://") || rel_path.contains("://") {
                    return Err(url::ParseError::RelativeUrlWithoutBase);
                }
                let dummy = format!("http://dummy/{}", rel_path.trim_start_matches('/'));
                Url::parse(&dummy)?
            }
            Err(e) => return Err(e),
        };

        let mut result = Url::parse(&self.base_url)?;

        let base_path = result.path().trim_end_matches('/');
        let clean_rel = rel_parsed.path().trim_start_matches('/');

        let new_path = if base_path.is_empty() {
            format!("/{}", clean_rel)
        } else {
            format!("{}/{}", base_path, clean_rel)
        };

        result.set_path(&new_path);

        if let Some(query) = rel_parsed.query() {
            result.set_query(Some(query));
        }
        if let Some(fragment) = rel_parsed.fragment() {
            result.set_fragment(Some(fragment));
        }

        Ok(result)
    }

    pub async fn new_request(
        &self,
        method: reqwest::Method,
        url_path: &str,
    ) -> Result<reqwest::RequestBuilder, crate::error::BuildkiteError> {
        let url = self.resolve_url(url_path)?;

        if self.http_debug {
            eprintln!("DEBUG request: {} {}", method, url);
        }

        let request = self
            .http_client
            .request(method, url)
            .bearer_auth(&self.token)
            .header("User-Agent", &self.user_agent);
        Ok(request)
    }
}

/// Buildkite API client
#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
    pub access_tokens: AccessTokensService,
    pub agents: AgentsService,
    pub annotations: AnnotationsService,
    pub artifacts: ArtifactsService,
    pub builds: BuildsService,
    pub clusters: ClustersService,
    pub cluster_tokens: ClusterTokensService,
    pub cluster_queues: ClusterQueuesService,
    pub flaky_tests: FlakyTestsService,
    pub jobs: JobsService,
    pub organizations: OrganizationsService,
    pub package_registries: PackageRegistriesService,
    pub packages: PackagesService,
    pub pipeline_templates: PipelineTemplatesService,
    pub pipelines: PipelinesService,
    pub rate_limit: RateLimitService,
    pub team_members: TeamMemberService,
    pub team_pipelines: TeamPipelinesService,
    pub team_suites: TeamSuitesService,
    pub teams: TeamsService,
    pub test_runs: TestRunsService,
    pub test_suites: TestSuitesService,
    pub tests: TestsService,
    pub user: UserService,
}

impl Client {
    /// Create a new client builder with the given token
    #[must_use]
    pub fn builder(token: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(token)
    }

    #[doc(hidden)]
    pub fn test_resolve_url(&self, rel_path: &str) -> Result<Url, url::ParseError> {
        self.inner.resolve_url(rel_path)
    }
}

/// Builder for creating a Buildkite client
pub struct ClientBuilder {
    token: String,
    base_url: Option<String>,
    http_client: Option<reqwest::Client>,
    user_agent: Option<String>,
    http_debug: bool,
}

impl ClientBuilder {
    /// Create a new client builder with the given token
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: None,
            http_client: None,
            user_agent: None,
            http_debug: false,
        }
    }

    /// Set a custom base URL (useful for testing)
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set a custom HTTP client
    #[must_use]
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Set a custom User-Agent header
    #[must_use]
    pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Some(agent.into());
        self
    }

    /// Enable HTTP debug logging
    #[must_use]
    pub fn http_debug(mut self, debug: bool) -> Self {
        self.http_debug = debug;
        self
    }

    /// Build the client
    #[must_use]
    pub fn build(self) -> Client {
        let http_client = self.http_client.unwrap_or_default();
        let mut base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());

        if !base_url.ends_with('/') {
            base_url.push('/');
        }

        let token = self.token;
        let user_agent = self.user_agent.unwrap_or_else(|| USER_AGENT.to_string());
        let http_debug = self.http_debug;

        let inner = Arc::new(ClientInner {
            http_client,
            base_url,
            token,
            user_agent,
            http_debug,
        });

        Client {
            inner: inner.clone(),
            access_tokens: AccessTokensService::new(inner.clone()),
            agents: AgentsService::new(inner.clone()),
            annotations: AnnotationsService::new(inner.clone()),
            artifacts: ArtifactsService::new(inner.clone()),
            builds: BuildsService::new(inner.clone()),
            clusters: ClustersService::new(inner.clone()),
            cluster_tokens: ClusterTokensService::new(inner.clone()),
            cluster_queues: ClusterQueuesService::new(inner.clone()),
            flaky_tests: FlakyTestsService::new(inner.clone()),
            jobs: JobsService::new(inner.clone()),
            organizations: OrganizationsService::new(inner.clone()),
            package_registries: PackageRegistriesService::new(inner.clone()),
            packages: PackagesService::new(inner.clone()),
            pipeline_templates: PipelineTemplatesService::new(inner.clone()),
            pipelines: PipelinesService::new(inner.clone()),
            rate_limit: RateLimitService::new(inner.clone()),
            team_members: TeamMemberService::new(inner.clone()),
            team_pipelines: TeamPipelinesService::new(inner.clone()),
            team_suites: TeamSuitesService::new(inner.clone()),
            teams: TeamsService::new(inner.clone()),
            test_runs: TestRunsService::new(inner.clone()),
            test_suites: TestSuitesService::new(inner.clone()),
            tests: TestsService::new(inner.clone()),
            user: UserService::new(inner),
        }
    }
}
