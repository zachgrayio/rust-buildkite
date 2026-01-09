use crate::client::{ClientInner, ResponseExt};
use crate::types::{TestSuite, TestSuiteCreate, TestSuiteListOptions, TestSuiteUpdate};
use std::sync::Arc;

#[derive(Clone)]
pub struct TestSuitesService {
    inner: Arc<ClientInner>,
}

impl TestSuitesService {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn list(&self, org: &str) -> Result<Vec<TestSuite>, Box<dyn std::error::Error>> {
        self.list_with_options(org, None).await
    }

    pub async fn list_with_options(
        &self,
        org: &str,
        opts: Option<TestSuiteListOptions>,
    ) -> Result<Vec<TestSuite>, Box<dyn std::error::Error>> {
        let mut url_path = format!("v2/analytics/organizations/{}/suites", org);

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
        Ok(response.json::<Vec<TestSuite>>().await?)
    }

    pub async fn get(
        &self,
        org: &str,
        suite_slug: &str,
    ) -> Result<TestSuite, Box<dyn std::error::Error>> {
        let url_path = format!("v2/analytics/organizations/{}/suites/{}", org, suite_slug);
        let request = self
            .inner
            .new_request(reqwest::Method::GET, &url_path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json::<TestSuite>().await?)
    }

    pub async fn create(
        &self,
        org: &str,
        suite_create: TestSuiteCreate,
    ) -> Result<TestSuite, Box<dyn std::error::Error>> {
        let url_path = format!("v2/analytics/organizations/{}/suites", org);
        let request = self
            .inner
            .new_request(reqwest::Method::POST, &url_path)
            .await?;
        let response = request
            .json(&suite_create)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;
        Ok(response.json::<TestSuite>().await?)
    }

    pub async fn update(
        &self,
        org: &str,
        suite_slug: &str,
        suite_update: TestSuiteUpdate,
    ) -> Result<TestSuite, Box<dyn std::error::Error>> {
        let url_path = format!("v2/analytics/organizations/{}/suites/{}", org, suite_slug);
        let request = self
            .inner
            .new_request(reqwest::Method::PATCH, &url_path)
            .await?;
        let response = request
            .json(&suite_update)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;
        Ok(response.json::<TestSuite>().await?)
    }

    pub async fn delete(
        &self,
        org: &str,
        suite_slug: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url_path = format!("v2/analytics/organizations/{}/suites/{}", org, suite_slug);
        let request = self
            .inner
            .new_request(reqwest::Method::DELETE, &url_path)
            .await?;
        request.send().await?.buildkite_error_for_status().await?;
        Ok(())
    }
}
