use crate::client::{ClientInner, ResponseExt};
use crate::types::{Cluster, ClusterCreate, ClusterUpdate, ClustersListOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct ClustersService {
    client: Arc<ClientInner>,
}

impl ClustersService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// List clusters
    pub async fn list(&self, org: &str) -> Result<Vec<Cluster>, Box<dyn std::error::Error>> {
        self.list_with_options(org, None).await
    }

    /// List clusters with options
    pub async fn list_with_options(
        &self,
        org: &str,
        options: Option<ClustersListOptions>,
    ) -> Result<Vec<Cluster>, Box<dyn std::error::Error>> {
        let mut url = format!("v2/organizations/{}/clusters", org);

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

        let clusters = response.json().await?;
        Ok(clusters)
    }

    /// Get a cluster
    pub async fn get(
        &self,
        org: &str,
        cluster_id: &str,
    ) -> Result<Cluster, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/clusters/{}", org, cluster_id);

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let cluster = response.json().await?;
        Ok(cluster)
    }

    /// Create a cluster
    pub async fn create(
        &self,
        org: &str,
        cluster: ClusterCreate,
    ) -> Result<Cluster, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/clusters", org);

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&cluster)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let cluster = response.json().await?;
        Ok(cluster)
    }

    /// Update a cluster
    pub async fn update(
        &self,
        org: &str,
        cluster_id: &str,
        update: ClusterUpdate,
    ) -> Result<Cluster, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/clusters/{}", org, cluster_id);

        let response = self
            .client
            .new_request(reqwest::Method::PATCH, &url)
            .await?
            .json(&update)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let cluster = response.json().await?;
        Ok(cluster)
    }

    /// Delete a cluster
    pub async fn delete(
        &self,
        org: &str,
        cluster_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/clusters/{}", org, cluster_id);

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
