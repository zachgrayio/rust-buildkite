use crate::client::{ClientInner, ResponseExt};
use crate::types::TeamSuite;
use std::sync::Arc;

#[derive(Clone)]
pub struct TeamSuitesService {
    client: Arc<ClientInner>,
}

impl TeamSuitesService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// List team suites
    pub async fn list(
        &self,
        org: &str,
        team_id: &str,
    ) -> Result<Vec<TeamSuite>, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams/{}/suites", org, team_id);

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let suites = response.json().await?;
        Ok(suites)
    }

    /// Get a team suite
    pub async fn get(
        &self,
        org: &str,
        team_id: &str,
        suite_id: &str,
    ) -> Result<TeamSuite, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/teams/{}/suites/{}",
            org, team_id, suite_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let suite = response.json().await?;
        Ok(suite)
    }

    /// Create a team suite association
    pub async fn create(
        &self,
        org: &str,
        team_id: &str,
        suite_id: &str,
        access_level: &str,
    ) -> Result<TeamSuite, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams/{}/suite", org, team_id);

        #[derive(serde::Serialize)]
        struct CreateBody {
            suite_id: String,
            access_level: String,
        }

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&CreateBody {
                suite_id: suite_id.to_string(),
                access_level: access_level.to_string(),
            })
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let suite = response.json().await?;
        Ok(suite)
    }

    /// Update a team suite association
    pub async fn update(
        &self,
        org: &str,
        team_id: &str,
        suite_id: &str,
        access_level: &str,
    ) -> Result<TeamSuite, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/teams/{}/suites/{}",
            org, team_id, suite_id
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

        let suite = response.json().await?;
        Ok(suite)
    }

    /// Delete a team suite association
    pub async fn delete(
        &self,
        org: &str,
        team_id: &str,
        suite_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/teams/{}/suites/{}",
            org, team_id, suite_id
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
