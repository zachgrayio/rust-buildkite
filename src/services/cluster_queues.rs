use crate::client::{ClientInner, ResponseExt};
use crate::types::{
    ClusterQueue, ClusterQueueCreate, ClusterQueuePause, ClusterQueueUpdate,
    ClusterQueuesListOptions,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ClusterQueuesService {
    client: Arc<ClientInner>,
}

impl ClusterQueuesService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        org: &str,
        cluster_id: &str,
    ) -> Result<Vec<ClusterQueue>, Box<dyn std::error::Error>> {
        self.list_with_options(org, cluster_id, None).await
    }

    pub async fn list_with_options(
        &self,
        org: &str,
        cluster_id: &str,
        options: Option<ClusterQueuesListOptions>,
    ) -> Result<Vec<ClusterQueue>, Box<dyn std::error::Error>> {
        let mut url = format!("v2/organizations/{}/clusters/{}/queues", org, cluster_id);

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

        let queues = response.json().await?;
        Ok(queues)
    }

    pub async fn get(
        &self,
        org: &str,
        cluster_id: &str,
        queue_id: &str,
    ) -> Result<ClusterQueue, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/clusters/{}/queues/{}",
            org, cluster_id, queue_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let queue = response.json().await?;
        Ok(queue)
    }

    pub async fn create(
        &self,
        org: &str,
        cluster_id: &str,
        queue: ClusterQueueCreate,
    ) -> Result<ClusterQueue, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/clusters/{}/queues", org, cluster_id);

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&queue)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let queue = response.json().await?;
        Ok(queue)
    }

    pub async fn update(
        &self,
        org: &str,
        cluster_id: &str,
        queue_id: &str,
        update: ClusterQueueUpdate,
    ) -> Result<ClusterQueue, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/clusters/{}/queues/{}",
            org, cluster_id, queue_id
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

        let queue = response.json().await?;
        Ok(queue)
    }

    pub async fn delete(
        &self,
        org: &str,
        cluster_id: &str,
        queue_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/clusters/{}/queues/{}",
            org, cluster_id, queue_id
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

    pub async fn pause(
        &self,
        org: &str,
        cluster_id: &str,
        queue_id: &str,
        pause: ClusterQueuePause,
    ) -> Result<ClusterQueue, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/clusters/{}/queues/{}/pause_dispatch",
            org, cluster_id, queue_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&pause)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let queue = response.json().await?;
        Ok(queue)
    }

    pub async fn resume(
        &self,
        org: &str,
        cluster_id: &str,
        queue_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/clusters/{}/queues/{}/resume_dispatch",
            org, cluster_id, queue_id
        );

        self.client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        Ok(())
    }
}
