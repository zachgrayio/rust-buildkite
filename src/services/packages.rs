use crate::client::{ClientInner, ResponseExt};
use crate::types::{Package, PackagePresignedUpload};
use futures::StreamExt;
use reqwest::Body;
use reqwest::multipart::{Form, Part};
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Clone)]
pub struct PackagesService {
    client: Arc<ClientInner>,
}

impl PackagesService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    pub async fn get(
        &self,
        org: &str,
        registry_slug: &str,
        package_id: &str,
    ) -> Result<Package, Box<dyn std::error::Error>> {
        let path = format!(
            "v2/packages/organizations/{}/registries/{}/packages/{}",
            org, registry_slug, package_id
        );
        let request = self.client.new_request(reqwest::Method::GET, &path).await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json().await?)
    }

    pub async fn delete(
        &self,
        org: &str,
        registry_slug: &str,
        package_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!(
            "v2/packages/organizations/{}/registries/{}/packages/{}",
            org, registry_slug, package_id
        );
        let request = self
            .client
            .new_request(reqwest::Method::DELETE, &path)
            .await?;
        request.send().await?.buildkite_error_for_status().await?;
        Ok(())
    }

    pub async fn request_presigned_upload(
        &self,
        org: &str,
        registry_slug: &str,
    ) -> Result<PackagePresignedUpload, Box<dyn std::error::Error + Send + Sync>> {
        let path = format!(
            "v2/packages/organizations/{}/registries/{}/packages/upload",
            org, registry_slug
        );
        let request = self
            .client
            .new_request(reqwest::Method::POST, &path)
            .await?;
        let response = request.send().await?.buildkite_error_for_status().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(
                format!("Failed to request presigned upload: {} - {}", status, body).into(),
            );
        }

        let presigned: PackagePresignedUpload = response.json().await?;
        Ok(presigned)
    }

    pub async fn perform_upload(
        &self,
        presigned: &PackagePresignedUpload,
        file: File,
        filename: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let file_len = file.metadata().await?.len();
        let stream =
            FramedRead::new(file, BytesCodec::new()).map(|r| r.map(|bytes| bytes.freeze()));
        let body = Body::wrap_stream(stream);

        let mut form = Form::new();
        for (key, value) in &presigned.form.data {
            form = form.text(key.clone(), value.clone());
        }

        let key = presigned
            .form
            .data
            .get("key")
            .ok_or("Missing 'key' in presigned upload form data")?
            .replace("${filename}", filename);

        let file_part = Part::stream_with_length(body, file_len)
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")?;
        form = form.part(presigned.form.file_input.clone(), file_part);

        let http_client = reqwest::Client::new();
        let response = http_client
            .post(&presigned.form.url)
            .multipart(form)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("S3 rejected upload with status {}: {}", status, body).into());
        }

        let upload_url = format!("{}/{}", presigned.form.url.trim_end_matches('/'), key);
        Ok(upload_url)
    }

    pub async fn finalize_upload(
        &self,
        org: &str,
        registry_slug: &str,
        s3_url: &str,
    ) -> Result<Package, Box<dyn std::error::Error + Send + Sync>> {
        let path = format!(
            "v2/packages/organizations/{}/registries/{}/packages",
            org, registry_slug
        );

        let form = Form::new().text("package_url", s3_url.to_string());

        let response = self
            .client
            .new_request(reqwest::Method::POST, &path)
            .await?
            .multipart(form)
            .send()
            .await?
            .buildkite_error_for_status()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Failed to finalize package: {} - {}", status, body).into());
        }

        let package: Package = response.json().await?;
        Ok(package)
    }

    pub async fn create(
        &self,
        org: &str,
        registry_slug: &str,
        file: File,
        filename: &str,
    ) -> Result<Package, Box<dyn std::error::Error + Send + Sync>> {
        let presigned = self.request_presigned_upload(org, registry_slug).await?;
        let s3_url = self.perform_upload(&presigned, file, filename).await?;
        let package = self.finalize_upload(org, registry_slug, &s3_url).await?;
        Ok(package)
    }

    pub async fn create_from_file(
        &self,
        org: &str,
        registry_slug: &str,
        file_path: &Path,
    ) -> Result<Package, Box<dyn std::error::Error + Send + Sync>> {
        let file = File::open(file_path).await?;
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid filename")?;

        self.create(org, registry_slug, file, filename).await
    }
}
