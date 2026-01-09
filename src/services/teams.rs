use crate::client::{ClientInner, ResponseExt};
use crate::types::{CreateTeam, Team, TeamsListOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct TeamsService {
    client: Arc<ClientInner>,
}

impl TeamsService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// List teams for an organization
    pub async fn list(&self, org: &str) -> Result<Vec<Team>, Box<dyn std::error::Error>> {
        self.list_with_options(org, None).await
    }

    /// List teams for an organization with options
    pub async fn list_with_options(
        &self,
        org: &str,
        opts: Option<TeamsListOptions>,
    ) -> Result<Vec<Team>, Box<dyn std::error::Error>> {
        let mut url = format!("v2/organizations/{}/teams", org);

        if let Some(options) = opts {
            let mut params = Vec::new();
            if let Some(page) = options.page {
                params.push(format!("page={}", page));
            }
            if let Some(per_page) = options.per_page {
                params.push(format!("per_page={}", per_page));
            }
            if let Some(user_id) = options.user_id {
                params.push(format!("user_id={}", user_id));
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

        let teams = response.json().await?;
        Ok(teams)
    }

    /// Get a team
    pub async fn get_team(
        &self,
        org: &str,
        team_id: &str,
    ) -> Result<Team, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams/{}", org, team_id);

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let team = response.json().await?;
        Ok(team)
    }

    /// Create a team
    pub async fn create_team(
        &self,
        org: &str,
        team: CreateTeam,
    ) -> Result<Team, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams", org);

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&team)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let team = response.json().await?;
        Ok(team)
    }

    /// Update a team
    pub async fn update_team(
        &self,
        org: &str,
        team_id: &str,
        team: CreateTeam,
    ) -> Result<Team, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams/{}", org, team_id);

        let response = self
            .client
            .new_request(reqwest::Method::PATCH, &url)
            .await?
            .json(&team)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let team = response.json().await?;
        Ok(team)
    }

    /// Delete a team
    pub async fn delete_team(
        &self,
        org: &str,
        team_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams/{}", org, team_id);

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
