use crate::client::{ClientInner, ResponseExt};
use crate::types::Test;
use std::sync::Arc;

#[derive(Clone)]
pub struct TestsService {
    inner: Arc<ClientInner>,
}

impl TestsService {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn get(
        &self,
        org: &str,
        suite_slug: &str,
        test_id: &str,
    ) -> Result<Test, Box<dyn std::error::Error>> {
        let url_path = format!(
            "v2/analytics/organizations/{}/suites/{}/tests/{}",
            org, suite_slug, test_id
        );
        let request = self
            .inner
            .new_request(reqwest::Method::GET, &url_path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json::<Test>().await?)
    }
}
