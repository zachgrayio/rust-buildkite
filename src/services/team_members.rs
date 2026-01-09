use crate::client::{ClientInner, ResponseExt};
use crate::types::{CreateTeamMember, TeamMember};
use std::sync::Arc;

#[derive(Clone)]
pub struct TeamMemberService {
    client: Arc<ClientInner>,
}

impl TeamMemberService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// List team members
    pub async fn list(
        &self,
        org: &str,
        team_id: &str,
    ) -> Result<Vec<TeamMember>, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams/{}/members", org, team_id);

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let members = response.json().await?;
        Ok(members)
    }

    /// Get a team member
    pub async fn get(
        &self,
        org: &str,
        team_id: &str,
        user_id: &str,
    ) -> Result<TeamMember, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/teams/{}/members/{}",
            org, team_id, user_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let member = response.json().await?;
        Ok(member)
    }

    /// Create a team member
    pub async fn create(
        &self,
        org: &str,
        team_id: &str,
        input: &CreateTeamMember,
    ) -> Result<TeamMember, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/teams/{}/members", org, team_id);

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(input)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let member = response.json().await?;
        Ok(member)
    }

    /// Update a team member
    pub async fn update(
        &self,
        org: &str,
        team_id: &str,
        user_id: &str,
        role: &str,
    ) -> Result<TeamMember, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/teams/{}/members/{}",
            org, team_id, user_id
        );

        #[derive(serde::Serialize)]
        struct UpdateBody {
            role: String,
        }

        let response = self
            .client
            .new_request(reqwest::Method::PATCH, &url)
            .await?
            .json(&UpdateBody {
                role: role.to_string(),
            })
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let member = response.json().await?;
        Ok(member)
    }

    /// Delete a team member
    pub async fn delete(
        &self,
        org: &str,
        team_id: &str,
        user_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/teams/{}/members/{}",
            org, team_id, user_id
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
