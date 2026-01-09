use crate::client::{ClientInner, ResponseExt};
use crate::types::RateLimit;
use std::sync::Arc;

#[derive(Clone)]
pub struct RateLimitService {
    inner: Arc<ClientInner>,
}

impl RateLimitService {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn get(&self, org: &str) -> Result<RateLimit, Box<dyn std::error::Error>> {
        let url_path = format!("v2/organizations/{}/rate_limit", org);
        let request = self
            .inner
            .new_request(reqwest::Method::GET, &url_path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json::<RateLimit>().await?)
    }
}
