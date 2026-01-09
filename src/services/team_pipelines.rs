use crate::client::{ClientInner, ResponseExt};
use crate::types::TeamPipeline;
use std::sync::Arc;

#[derive(Clone)]
pub struct TeamPipelinesService {
    client: Arc<ClientInner>,
}

impl TeamPipelinesService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// List team pipelines
    pub async fn list(
        &self,
        org: &str,
        team_id: &str,
    ) -> Result<Vec<TeamPipeline>, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams/{}/pipelines", org, team_id);

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let pipelines = response.json().await?;
        Ok(pipelines)
    }

    /// Get a team pipeline
    pub async fn get(
        &self,
        org: &str,
        team_id: &str,
        pipeline_id: &str,
    ) -> Result<TeamPipeline, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/teams/{}/pipelines/{}",
            org, team_id, pipeline_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let pipeline = response.json().await?;
        Ok(pipeline)
    }

    /// Create a team pipeline association
    pub async fn create(
        &self,
        org: &str,
        team_id: &str,
        pipeline_id: &str,
        access_level: &str,
    ) -> Result<TeamPipeline, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams/{}/pipelines", org, team_id);

        #[derive(serde::Serialize)]
        struct CreateBody {
            pipeline_id: String,
            access_level: String,
        }

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&CreateBody {
                pipeline_id: pipeline_id.to_string(),
                access_level: access_level.to_string(),
            })
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let pipeline = response.json().await?;
        Ok(pipeline)
    }

    /// Update a team pipeline association
    pub async fn update(
        &self,
        org: &str,
        team_id: &str,
        pipeline_id: &str,
        access_level: &str,
    ) -> Result<TeamPipeline, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/teams/{}/pipelines/{}",
            org, team_id, pipeline_id
        );

        #[derive(serde::Serialize)]
        struct UpdateBody {
            access_level: String,
        }

        let response = self
            .client
            .new_request(reqwest::Method::PATCH, &url)
            .await?
            .json(&UpdateBody {
                access_level: access_level.to_string(),
            })
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let pipeline = response.json().await?;
        Ok(pipeline)
    }

    /// Delete a team pipeline association
    pub async fn delete(
        &self,
        org: &str,
        team_id: &str,
        pipeline_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/teams/{}/pipelines/{}",
            org, team_id, pipeline_id
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
