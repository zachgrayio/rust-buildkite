use crate::client::{ClientInner, ResponseExt};
use crate::types::{Annotation, AnnotationCreate, AnnotationListOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct AnnotationsService {
    client: Arc<ClientInner>,
}

impl AnnotationsService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// List annotations for a build
    pub async fn list_by_build(
        &self,
        org: &str,
        pipeline: &str,
        build: &str,
    ) -> Result<Vec<Annotation>, Box<dyn std::error::Error>> {
        self.list_by_build_with_options(org, pipeline, build, None)
            .await
    }

    /// List annotations for a build with options
    pub async fn list_by_build_with_options(
        &self,
        org: &str,
        pipeline: &str,
        build: &str,
        opts: Option<AnnotationListOptions>,
    ) -> Result<Vec<Annotation>, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/annotations",
            org, pipeline, build
        );

        let mut request = self.client.new_request(reqwest::Method::GET, &url).await?;

        if let Some(options) = opts {
            if let Some(page) = options.page {
                request = request.query(&[("page", page.to_string())]);
            }
            if let Some(per_page) = options.per_page {
                request = request.query(&[("per_page", per_page.to_string())]);
            }
        }

        let response = request.send().await?.buildkite_error_for_status().await?;
        let annotations = response.json().await?;
        Ok(annotations)
    }

    /// Create an annotation
    pub async fn create(
        &self,
        org: &str,
        pipeline: &str,
        build: &str,
        annotation: AnnotationCreate,
    ) -> Result<Annotation, Box<dyn std::error::Error>> {
        let url = format!(
            "v2/organizations/{}/pipelines/{}/builds/{}/annotations",
            org, pipeline, build
        );

        let response = self
            .client
            .new_request(reqwest::Method::POST, &url)
            .await?
            .json(&annotation)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        let annotation = response.json().await?;
        Ok(annotation)
    }
}
