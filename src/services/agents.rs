use crate::client::{ClientInner, ResponseExt};
use crate::types::{Agent, AgentListOptions, AgentPauseOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct AgentsService {
    client: Arc<ClientInner>,
}

impl AgentsService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// List agents for an organization
    pub async fn list(&self, org: &str) -> Result<Vec<Agent>, Box<dyn std::error::Error>> {
        self.list_with_options(org, None).await
    }

    /// List agents for an organization with optional filter options
    pub async fn list_with_options(
        &self,
        org: &str,
        opts: Option<AgentListOptions>,
    ) -> Result<Vec<Agent>, Box<dyn std::error::Error>> {
        let mut url = format!("v2/organizations/{}/agents", org);

        if let Some(options) = opts {
            let mut params = Vec::new();
            if let Some(name) = &options.name {
                params.push(format!("name={}", name));
            }
            if let Some(hostname) = &options.hostname {
                params.push(format!("hostname={}", hostname));
            }
            if let Some(version) = &options.version {
                params.push(format!("version={}", version));
            }
            if let Some(page) = options.page {
                params.push(format!("page={}", page));
            }
            if let Some(per_page) = options.per_page {
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

        let agents = response.json().await?;
        Ok(agents)
    }

    /// Create an agent
    pub async fn create(
        &self,
        org: &str,
        agent: Agent,
    ) -> Result<Agent, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/agents", org);
        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&agent)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;
        let agent = response.json().await?;
        Ok(agent)
    }

    /// Get an agent
    pub async fn get(
        &self,
        org: &str,
        agent_id: &str,
    ) -> Result<Agent, Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/agents/{}", org, agent_id);

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let agent = response.json().await?;
        Ok(agent)
    }

    /// Stop an agent
    pub async fn stop(
        &self,
        org: &str,
        agent_id: &str,
        force: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/agents/{}/stop", org, agent_id);

        #[derive(serde::Serialize)]
        struct StopBody {
            force: bool,
        }

        self.client
            .new_request(reqwest::Method::PUT, &url)
            .await?
            .json(&StopBody { force })
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        Ok(())
    }

    /// Pause an agent
    pub async fn pause(
        &self,
        org: &str,
        agent_id: &str,
        opts: Option<AgentPauseOptions>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/agents/{}/pause", org, agent_id);

        let body = opts.unwrap_or_default();

        self.client
            .new_request(reqwest::Method::PUT, &url)
            .await?
            .json(&body)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        Ok(())
    }

    /// Resume an agent
    pub async fn resume(
        &self,
        org: &str,
        agent_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/agents/{}/resume", org, agent_id);

        self.client
            .new_request(reqwest::Method::PUT, &url)
            .await?
            .json(&serde_json::json!({}))
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        Ok(())
    }

    /// Delete an agent
    pub async fn delete(
        &self,
        org: &str,
        agent_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("v2/organizations/{}/agents/{}", org, agent_id);

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
