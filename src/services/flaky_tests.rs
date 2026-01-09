use crate::client::{ClientInner, ResponseExt};
use crate::types::{FlakyTest, FlakyTestsListOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct FlakyTestsService {
    inner: Arc<ClientInner>,
}

impl FlakyTestsService {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn list(
        &self,
        org: &str,
        suite_slug: &str,
    ) -> Result<Vec<FlakyTest>, Box<dyn std::error::Error>> {
        self.list_with_options(org, suite_slug, None).await
    }

    pub async fn list_with_options(
        &self,
        org: &str,
        suite_slug: &str,
        opts: Option<FlakyTestsListOptions>,
    ) -> Result<Vec<FlakyTest>, Box<dyn std::error::Error>> {
        let mut url_path = format!(
            "v2/analytics/organizations/{}/suites/{}/flaky-tests",
            org, suite_slug
        );

        if let Some(options) = opts {
            let mut params = Vec::new();
            if let Some(page) = options.page {
                params.push(format!("page={}", page));
            }
            if let Some(per_page) = options.per_page {
                params.push(format!("per_page={}", per_page));
            }
            if !params.is_empty() {
                url_path = format!("{}?{}", url_path, params.join("&"));
            }
        }

        let request = self
            .inner
            .new_request(reqwest::Method::GET, &url_path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json::<Vec<FlakyTest>>().await?)
    }
}
