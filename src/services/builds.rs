use crate::client::{ClientInner, ResponseExt};
use crate::types::{Build, BuildGetOptions, BuildsListOptions, CreateBuild};
use std::sync::Arc;

#[derive(Clone)]
pub struct BuildsService {
    client: Arc<ClientInner>,
}

impl BuildsService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// Get a build
    pub async fn get(
        &self,
        org: &str,
        pipeline: &str,
        build_number: &str,
    ) -> Result<Build, Box<dyn std::error::Error>> {
        self.get_with_options(org, pipeline, build_number, None)
            .await
    }

    /// Get a build with options
    pub async fn get_with_options(
        &self,
        org: &str,
        pipeline: &str,
        build_number: &str,
        opts: Option<BuildGetOptions>,
    ) -> Result<Build, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}",
            org, pipeline, build_number
        );

        let mut request = self.client.new_request(reqwest::Method::GET, &url).await?;

        if let Some(ref options) = opts {
            if let Some(include_retried) = options.include_retried_jobs {
                request = request.query(&[("include_retried_jobs", include_retried.to_string())]);
            }
            if let Some(include_test_engine) = options.include_test_engine {
                request =
                    request.query(&[("include_test_engine", include_test_engine.to_string())]);
            }
        }

        let response = request.send().await?.buildkite_error_for_status().await?;
        let build = response.json().await?;
        Ok(build)
    }

    /// Cancel a build
    pub async fn cancel(
        &self,
        org: &str,
        pipeline: &str,
        build_number: &str,
    ) -> Result<Build, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/cancel",
            org, pipeline, build_number
        );

        let response = self
            .client
            .new_request(reqwest::Method::PUT, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let build = response.json().await?;
        Ok(build)
    }

    /// List all builds
    pub async fn list(&self) -> Result<Vec<Build>, Box<dyn std::error::Error>> {
        self.list_with_options(None).await
    }

    /// List all builds with options
    pub async fn list_with_options(
        &self,
        opts: Option<BuildsListOptions>,
    ) -> Result<Vec<Build>, Box<dyn std::error::Error>> {
        let url = "v2/builds";

        let mut request = self.client.new_request(reqwest::Method::GET, url).await?;

        request = Self::apply_list_options(request, &opts);

        let response = request.send().await?.buildkite_error_for_status().await?;
        let builds = response.json().await?;
        Ok(builds)
    }

    /// List builds by organization
    pub async fn list_by_org(&self, org: &str) -> Result<Vec<Build>, Box<dyn std::error::Error>> {
        self.list_by_org_with_options(org, None).await
    }

    /// List builds by organization with options
    pub async fn list_by_org_with_options(
        &self,
        org: &str,
        opts: Option<BuildsListOptions>,
    ) -> Result<Vec<Build>, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/builds", org);

        let mut request = self.client.new_request(reqwest::Method::GET, &url).await?;

        request = Self::apply_list_options(request, &opts);

        let response = request.send().await?.buildkite_error_for_status().await?;
        let builds = response.json().await?;
        Ok(builds)
    }

    /// List builds by pipeline
    pub async fn list_by_pipeline(
        &self,
        org: &str,
        pipeline: &str,
    ) -> Result<Vec<Build>, Box<dyn std::error::Error>> {
        self.list_by_pipeline_with_options(org, pipeline, None)
            .await
    }

    /// List builds by pipeline with options
    pub async fn list_by_pipeline_with_options(
        &self,
        org: &str,
        pipeline: &str,
        opts: Option<BuildsListOptions>,
    ) -> Result<Vec<Build>, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines/{}/builds", org, pipeline);

        let mut request = self.client.new_request(reqwest::Method::GET, &url).await?;

        request = Self::apply_list_options(request, &opts);

        let response = request.send().await?.buildkite_error_for_status().await?;
        let builds = response.json().await?;
        Ok(builds)
    }

    fn apply_list_options(
        mut request: reqwest::RequestBuilder,
        opts: &Option<BuildsListOptions>,
    ) -> reqwest::RequestBuilder {
        if let Some(options) = opts {
            if let Some(page) = options.page {
                request = request.query(&[("page", page.to_string())]);
            }
            if let Some(per_page) = options.per_page {
                request = request.query(&[("per_page", per_page.to_string())]);
            }
            if let Some(ref creator) = options.creator {
                request = request.query(&[("creator", creator)]);
            }
            if let Some(ref created_from) = options.created_from {
                let formatted = created_from
                    .0
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap();
                request = request.query(&[("created_from", formatted)]);
            }
            if let Some(ref created_to) = options.created_to {
                let formatted = created_to
                    .0
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap();
                request = request.query(&[("created_to", formatted)]);
            }
            if let Some(ref finished_from) = options.finished_from {
                let formatted = finished_from
                    .0
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap();
                request = request.query(&[("finished_from", formatted)]);
            }
            if let Some(ref state) = options.state {
                for s in state {
                    request = request.query(&[("state[]", s)]);
                }
            }
            if let Some(ref branch) = options.branch {
                for b in branch {
                    request = request.query(&[("branch[]", b)]);
                }
            }
            if let Some(ref commit) = options.commit {
                request = request.query(&[("commit", commit)]);
            }
            if let Some(include_retried) = options.include_retried_jobs {
                request = request.query(&[("include_retried_jobs", include_retried.to_string())]);
            }
            if let Some(exclude_pipeline) = options.exclude_pipeline {
                request = request.query(&[("exclude_pipeline", exclude_pipeline.to_string())]);
            }
            if let Some(exclude_jobs) = options.exclude_jobs {
                request = request.query(&[("exclude_jobs", exclude_jobs.to_string())]);
            }
            // Add meta_data filters in the format meta_data[key]=value
            if let Some(ref meta_data) = options.meta_data {
                for (key, value) in meta_data {
                    request = request.query(&[(format!("meta_data[{}]", key), value)]);
                }
            }
        }
        request
    }

    /// Create a build
    pub async fn create(
        &self,
        org: &str,
        pipeline: &str,
        build: CreateBuild,
    ) -> Result<Build, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines/{}/builds", org, pipeline);

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&build)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let build = response.json().await?;
        Ok(build)
    }

    /// Rebuild a build
    pub async fn rebuild(
        &self,
        org: &str,
        pipeline: &str,
        build_number: &str,
    ) -> Result<Build, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/rebuild",
            org, pipeline, build_number
        );

        let response = self
            .client
            .new_request(reqwest::Method::PUT, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let build = response.json().await?;
        Ok(build)
    }
}
