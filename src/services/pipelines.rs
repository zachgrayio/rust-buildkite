use crate::client::{ClientInner, ResponseExt};
use crate::types::{CreatePipeline, Pipeline, PipelineListOptions, UpdatePipeline};
use std::sync::Arc;

#[derive(Clone)]
pub struct PipelinesService {
    client: Arc<ClientInner>,
}

impl PipelinesService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// List pipelines
    pub async fn list(&self, org: &str) -> Result<Vec<Pipeline>, Box<dyn std::error::Error>> {
        self.list_with_options(org, None).await
    }

    /// List pipelines with options
    pub async fn list_with_options(
        &self,
        org: &str,
        opts: Option<PipelineListOptions>,
    ) -> Result<Vec<Pipeline>, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines", org);
        let mut request = self.client.new_request(reqwest::Method::GET, &url).await?;

        if let Some(ref options) = opts {
            if let Some(page) = options.page {
                request = request.query(&[("page", page.to_string())]);
            }
            if let Some(per_page) = options.per_page {
                request = request.query(&[("per_page", per_page.to_string())]);
            }
            if let Some(ref name) = options.name {
                request = request.query(&[("name", name)]);
            }
            if let Some(ref repository) = options.repository {
                request = request.query(&[("repository", repository)]);
            }
        }

        let response = request.send().await?.buildkite_error_for_status().await?;
        let pipelines = response.json().await?;
        Ok(pipelines)
    }

    /// Get a pipeline
    pub async fn get(
        &self,
        org: &str,
        pipeline: &str,
    ) -> Result<Pipeline, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines/{}", org, pipeline);
        let pipeline = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?
            .json()
            .await?;
        Ok(pipeline)
    }

    /// Create a pipeline
    pub async fn create(
        &self,
        org: &str,
        pipeline: CreatePipeline,
    ) -> Result<Pipeline, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines", org);
        let pipeline = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&pipeline)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?
            .json()
            .await?;
        Ok(pipeline)
    }

    /// Delete a pipeline
    pub async fn delete(
        &self,
        org: &str,
        pipeline: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines/{}", org, pipeline);
        self.client
            .new_request(reqwest::Method::DELETE, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;
        Ok(())
    }

    /// Update a pipeline
    pub async fn update(
        &self,
        org: &str,
        slug: &str,
        update: UpdatePipeline,
    ) -> Result<Pipeline, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines/{}", org, slug);
        let pipeline = self
            .client
            .new_request(reqwest::Method::PATCH, &url)
            .await?
            .json(&update)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?
            .json()
            .await?;
        Ok(pipeline)
    }

    /// Add a webhook to a pipeline
    pub async fn add_webhook(
        &self,
        org: &str,
        slug: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines/{}/webhook", org, slug);
        self.client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;
        Ok(())
    }

    /// Archive a pipeline
    pub async fn archive(&self, org: &str, slug: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines/{}/archive", org, slug);
        self.client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;
        Ok(())
    }

    /// Unarchive a pipeline
    pub async fn unarchive(&self, org: &str, slug: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/pipelines/{}/unarchive", org, slug);
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
