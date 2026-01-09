use crate::client::{ClientInner, ResponseExt};
use crate::types::{PipelineTemplate, PipelineTemplateCreate, PipelineTemplateUpdate};
use std::sync::Arc;

#[derive(Clone)]
pub struct PipelineTemplatesService {
    client: Arc<ClientInner>,
}

impl PipelineTemplatesService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        org: &str,
    ) -> Result<Vec<PipelineTemplate>, Box<dyn std::error::Error>> {
        let path = format!("v2/organizations/{}/pipeline-templates", org);
        let request = self.client.new_request(reqwest::Method::GET, &path).await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json().await?)
    }

    pub async fn get(
        &self,
        org: &str,
        uuid: &str,
    ) -> Result<PipelineTemplate, Box<dyn std::error::Error>> {
        let path = format!("v2/organizations/{}/pipeline-templates/{}", org, uuid);
        let request = self.client.new_request(reqwest::Method::GET, &path).await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json().await?)
    }

    /// Create a pipeline template
    pub async fn create(
        &self,
        org: &str,
        input: PipelineTemplateCreate,
    ) -> Result<PipelineTemplate, Box<dyn std::error::Error>> {
        let path = format!("v2/organizations/{}/pipeline-templates", org);
        let request = self
            .client
            .new_request(reqwest::Method::POST, &path)
            .await?;
        let response = request
            .json(&input)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;
        Ok(response.json().await?)
    }

    /// Update a pipeline template
    pub async fn update(
        &self,
        org: &str,
        uuid: &str,
        input: PipelineTemplateUpdate,
    ) -> Result<PipelineTemplate, Box<dyn std::error::Error>> {
        let path = format!("v2/organizations/{}/pipeline-templates/{}", org, uuid);
        let request = self
            .client
            .new_request(reqwest::Method::PATCH, &path)
            .await?;
        let response = request
            .json(&input)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;
        Ok(response.json().await?)
    }

    /// Delete a pipeline template
    pub async fn delete(&self, org: &str, uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("v2/organizations/{}/pipeline-templates/{}", org, uuid);
        let request = self
            .client
            .new_request(reqwest::Method::DELETE, &path)
            .await?;
        request.send().await?.buildkite_error_for_status().await?;
        Ok(())
    }
}
