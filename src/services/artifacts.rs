use crate::client::{ClientInner, ResponseExt};
use crate::types::{Artifact, ArtifactListOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct ArtifactsService {
    inner: Arc<ClientInner>,
}

impl ArtifactsService {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn list_by_build(
        &self,
        org: &str,
        pipeline: &str,
        build: &str,
    ) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
        self.list_by_build_with_options(org, pipeline, build, None)
            .await
    }

    pub async fn list_by_build_with_options(
        &self,
        org: &str,
        pipeline: &str,
        build: &str,
        opts: Option<ArtifactListOptions>,
    ) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
        let mut url_path = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/artifacts",
            org, pipeline, build
        );

        if let Some(options) = opts {
            let mut params = Vec::new();
            if let Some(page) = options.page {
                params.push(format!("page={}", page));
            }
            if let Some(per_page) = options.per_page {
                params.push(format!("per_page={}", per_page));
            }
            if !params.is_empty() {
                url_path = format!("{}?{}", url_path, params.join("&"));
            }
        }

        let request = self
            .inner
            .new_request(reqwest::Method::GET, &url_path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json::<Vec<Artifact>>().await?)
    }

    pub async fn get(
        &self,
        org: &str,
        pipeline: &str,
        build: &str,
        job: &str,
        id: &str,
    ) -> Result<Artifact, Box<dyn std::error::Error>> {
        let url_path = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/jobs/{}/artifacts/{}",
            org, pipeline, build, job, id
        );
        let request = self
            .inner
            .new_request(reqwest::Method::GET, &url_path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json::<Artifact>().await?)
    }

    /// List artifacts for a specific job
    pub async fn list_by_job(
        &self,
        org: &str,
        pipeline: &str,
        build: &str,
        job: &str,
    ) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
        self.list_by_job_with_options(org, pipeline, build, job, None)
            .await
    }

    /// List artifacts for a specific job with options
    pub async fn list_by_job_with_options(
        &self,
        org: &str,
        pipeline: &str,
        build: &str,
        job: &str,
        opts: Option<ArtifactListOptions>,
    ) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
        let mut url_path = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/jobs/{}/artifacts",
            org, pipeline, build, job
        );

        if let Some(options) = opts {
            let mut params = Vec::new();
            if let Some(page) = options.page {
                params.push(format!("page={}", page));
            }
            if let Some(per_page) = options.per_page {
                params.push(format!("per_page={}", per_page));
            }
            if !params.is_empty() {
                url_path = format!("{}?{}", url_path, params.join("&"));
            }
        }

        let request = self
            .inner
            .new_request(reqwest::Method::GET, &url_path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json::<Vec<Artifact>>().await?)
    }

    /// DownloadArtifactByURL downloads an artifact from the specified URL.
    pub async fn download_artifact_by_url(
        &self,
        url: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let response = self
            .inner
            .http_client
            .get(url)
            .bearer_auth(&self.inner.token)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
