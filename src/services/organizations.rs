use crate::client::{ClientInner, ResponseExt};
use crate::types::{Emoji, Organization, OrganizationListOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct OrganizationsService {
    client: Arc<ClientInner>,
}

impl OrganizationsService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// List organizations the current user belongs to
    pub async fn list(&self) -> Result<Vec<Organization>, Box<dyn std::error::Error>> {
        self.list_with_options(None).await
    }

    /// List organizations the current user belongs to with options
    pub async fn list_with_options(
        &self,
        opts: Option<OrganizationListOptions>,
    ) -> Result<Vec<Organization>, Box<dyn std::error::Error>> {
        let mut request = self
            .client
            .new_request(reqwest::Method::GET, "v2/organizations")
            .await?;

        if let Some(options) = opts {
            if let Some(page) = options.page {
                request = request.query(&[("page", page.to_string())]);
            }
            if let Some(per_page) = options.per_page {
                request = request.query(&[("per_page", per_page.to_string())]);
            }
        }

        let response = request.send().await?.buildkite_error_for_status().await?;
        let organizations = response.json().await?;
        Ok(organizations)
    }

    /// Get an organization
    pub async fn get(&self, org: &str) -> Result<Organization, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}", org);
        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let organization = response.json().await?;
        Ok(organization)
    }

    /// List emojis for an organization
    pub async fn list_emojis(&self, org: &str) -> Result<Vec<Emoji>, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/emojis", org);
        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let emojis = response.json().await?;
        Ok(emojis)
    }
}
