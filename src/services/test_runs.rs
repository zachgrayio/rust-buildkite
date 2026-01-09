use crate::client::{ClientInner, ResponseExt};
use crate::types::{FailedExecution, FailedExecutionsOptions, TestRun, TestRunsListOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct TestRunsService {
    inner: Arc<ClientInner>,
}

impl TestRunsService {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn get(
        &self,
        org: &str,
        suite_slug: &str,
        run_id: &str,
    ) -> Result<TestRun, Box<dyn std::error::Error>> {
        let url_path = format!(
            "v2/analytics/organizations/{}/suites/{}/runs/{}",
            org, suite_slug, run_id
        );
        let request = self
            .inner
            .new_request(reqwest::Method::GET, &url_path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json::<TestRun>().await?)
    }

    pub async fn list(
        &self,
        org: &str,
        suite_slug: &str,
    ) -> Result<Vec<TestRun>, Box<dyn std::error::Error>> {
        self.list_with_options(org, suite_slug, None).await
    }

    pub async fn list_with_options(
        &self,
        org: &str,
        suite_slug: &str,
        opts: Option<TestRunsListOptions>,
    ) -> Result<Vec<TestRun>, Box<dyn std::error::Error>> {
        let mut url_path = format!(
            "v2/analytics/organizations/{}/suites/{}/runs",
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
        Ok(response.json::<Vec<TestRun>>().await?)
    }

    /// GetFailedExecutions returns the failed executions for a test run.
    pub async fn get_failed_executions(
        &self,
        org: &str,
        suite_slug: &str,
        run_id: &str,
        opts: Option<FailedExecutionsOptions>,
    ) -> Result<Vec<FailedExecution>, Box<dyn std::error::Error>> {
        let mut url_path = format!(
            "v2/analytics/organizations/{}/suites/{}/runs/{}/failed_executions",
            org, suite_slug, run_id
        );

        if opts.is_some_and(|o| o.include_failure_expanded == Some(true)) {
            url_path = format!("{}?include_failure_expanded=true", url_path);
        }

        let request = self
            .inner
            .new_request(reqwest::Method::GET, &url_path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json::<Vec<FailedExecution>>().await?)
    }
}
