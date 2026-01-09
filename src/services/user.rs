use crate::client::{ClientInner, ResponseExt};
use crate::types::User;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    client: Arc<ClientInner>,
}

impl UserService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// Get the current user (associated with the access token)
    pub async fn current_user(&self) -> Result<User, Box<dyn std::error::Error>> {
        let response = self
            .client
            .new_request(reqwest::Method::GET, "v2/user")
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let user = response.json().await?;
        Ok(user)
    }
}
