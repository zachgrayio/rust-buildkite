use crate::client::{ClientInner, ResponseExt};
use crate::types::AccessToken;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccessTokensService {
    client: Arc<ClientInner>,
}

impl AccessTokensService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// Get the current access token details
    pub async fn get(&self) -> Result<AccessToken, Box<dyn std::error::Error>> {
        let response = self
            .client
            .new_request(reqwest::Method::GET, "v2/access-token")
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let token = response.json().await?;
        Ok(token)
    }

    /// Revoke the current access token
    pub async fn revoke(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .new_request(reqwest::Method::DELETE, "v2/access-token")
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        Ok(())
    }
}
