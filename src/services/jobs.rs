use crate::client::{ClientInner, ResponseExt};
use crate::types::{Job, JobEnvs, JobLog, JobUnblockOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct JobsService {
    client: Arc<ClientInner>,
}

impl JobsService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// Unblock a job
    pub async fn unblock_job(
        &self,
        org: &str,
        pipeline: &str,
        build_number: &str,
        job_id: &str,
        opts: Option<JobUnblockOptions>,
    ) -> Result<Job, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/jobs/{}/unblock",
            org, pipeline, build_number, job_id
        );

        let body = opts.unwrap_or_default();

        let response = self
            .client
            .new_request(reqwest::Method::PUT, &url)
            .await?
            .json(&body)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let job = response.json().await?;
        Ok(job)
    }

    /// Retry a job
    pub async fn retry_job(
        &self,
        org: &str,
        pipeline: &str,
        build_number: &str,
        job_id: &str,
    ) -> Result<Job, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/jobs/{}/retry",
            org, pipeline, build_number, job_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::PUT, &url)
            .await?
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let job = response.json().await?;
        Ok(job)
    }

    /// Get a job's log output
    pub async fn get_job_log(
        &self,
        org: &str,
        pipeline: &str,
        build_number: &str,
        job_id: &str,
    ) -> Result<JobLog, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/jobs/{}/log",
            org, pipeline, build_number, job_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .header("Accept", "application/json")
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let log = response.json().await?;
        Ok(log)
    }

    /// Get a job's environment variables
    pub async fn get_job_environment_variables(
        &self,
        org: &str,
        pipeline: &str,
        build_number: &str,
        job_id: &str,
    ) -> Result<JobEnvs, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/jobs/{}/env",
            org, pipeline, build_number, job_id
        );

        let response = self
            .client
            .new_request(reqwest::Method::GET, &url)
            .await?
            .header("Accept", "application/json")
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let envs = response.json().await?;
        Ok(envs)
    }
}
