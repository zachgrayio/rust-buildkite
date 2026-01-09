use crate::client::{ClientInner, ResponseExt};
use crate::types::{ClusterToken, ClusterTokenCreateUpdate, ClusterTokensListOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct ClusterTokensService {
    client: Arc<ClientInner>,
}

impl ClusterTokensService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        org: &str,
        cluster_id: &str,
    ) -> Result<Vec<ClusterToken>, Box<dyn std::error::Error>> {
        self.list_with_options(org, cluster_id, None).await
    }

    pub async fn list_with_options(
        &self,
        org: &str,
        cluster_id: &str,
        options: Option<ClusterTokensListOptions>,
    ) -> Result<Vec<ClusterToken>, Box<dyn std::error::Error>> {
        let mut url = format!("v2/organizations/{}/clusters/{}/tokens", org, cluster_id);

        if let Some(opts) = options {
            let mut params = Vec::new();
            if let Some(page) = opts.page {
                params.push(format!("page={}", page));
            }
            if let Some(per_page) = opts.per_page {
                params.push(format!("per_page={}", per_page));
            }
            if !params.is_empty() {
                url = format!("{}?{}", url, params.join("&"));
            }
        }

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let tokens = response.json().await?;
        Ok(tokens)
    }

    pub async fn get(
        &self,
        org: &str,
        cluster_id: &str,
        token_id: &str,
    ) -> Result<ClusterToken, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/clusters/{}/tokens/{}",
            org, cluster_id, token_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let token = response.json().await?;
        Ok(token)
    }

    pub async fn create(
        &self,
        org: &str,
        cluster_id: &str,
        token: ClusterTokenCreateUpdate,
    ) -> Result<ClusterToken, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/clusters/{}/tokens", org, cluster_id);

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&token)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let token = response.json().await?;
        Ok(token)
    }

    pub async fn update(
        &self,
        org: &str,
        cluster_id: &str,
        token_id: &str,
        update: ClusterTokenCreateUpdate,
    ) -> Result<ClusterToken, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/clusters/{}/tokens/{}",
            org, cluster_id, token_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::PATCH, &url)
            .await?
            .json(&update)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let token = response.json().await?;
        Ok(token)
    }

    pub async fn delete(
        &self,
        org: &str,
        cluster_id: &str,
        token_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/clusters/{}/tokens/{}",
            org, cluster_id, token_id
        );

        self.client
            .new_request(reqwest::Method::DELETE, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        Ok(())
    }
}
