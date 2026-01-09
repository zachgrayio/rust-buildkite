use crate::client::{ClientInner, ResponseExt};
use crate::types::{
    CreatePackageRegistryInput, PackageRegistry, RegistryPackages, RegistryPackagesOptions,
    UpdatePackageRegistryInput,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct PackageRegistriesService {
    client: Arc<ClientInner>,
}

impl PackageRegistriesService {
    pub(crate) fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        org: &str,
    ) -> Result<Vec<PackageRegistry>, Box<dyn std::error::Error>> {
        let path = format!("v2/packages/organizations/{}/registries", org);
        let request = self.client.new_request(reqwest::Method::GET, &path).await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json().await?)
    }

    pub async fn get(
        &self,
        org: &str,
        registry_slug: &str,
    ) -> Result<PackageRegistry, Box<dyn std::error::Error>> {
        let path = format!(
            "v2/packages/organizations/{}/registries/{}",
            org, registry_slug
        );
        let request = self.client.new_request(reqwest::Method::GET, &path).await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json().await?)
    }

    /// Create a package registry
    pub async fn create(
        &self,
        org: &str,
        input: CreatePackageRegistryInput,
    ) -> Result<PackageRegistry, Box<dyn std::error::Error>> {
        let path = format!("v2/packages/organizations/{}/registries", org);
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

    /// Update a package registry
    pub async fn update(
        &self,
        org: &str,
        registry_slug: &str,
        input: UpdatePackageRegistryInput,
    ) -> Result<PackageRegistry, Box<dyn std::error::Error>> {
        let path = format!(
            "v2/packages/organizations/{}/registries/{}",
            org, registry_slug
        );
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

    /// Delete a package registry
    pub async fn delete(
        &self,
        org: &str,
        registry_slug: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!(
            "v2/packages/organizations/{}/registries/{}",
            org, registry_slug
        );
        let request = self
            .client
            .new_request(reqwest::Method::DELETE, &path)
            .await?;
        request.send().await?.buildkite_error_for_status().await?;
        Ok(())
    }

    /// List packages in a registry
    pub async fn list_packages(
        &self,
        org: &str,
        registry_slug: &str,
    ) -> Result<RegistryPackages, Box<dyn std::error::Error>> {
        self.list_packages_with_options(org, registry_slug, None)
            .await
    }

    /// List packages in a registry with options
    pub async fn list_packages_with_options(
        &self,
        org: &str,
        registry_slug: &str,
        opts: Option<RegistryPackagesOptions>,
    ) -> Result<RegistryPackages, Box<dyn std::error::Error>> {
        let mut path = format!(
            "v2/packages/organizations/{}/registries/{}/packages",
            org, registry_slug
        );

        if let Some(options) = opts {
            let mut params = Vec::new();
            if let Some(before) = options.before {
                params.push(format!("before={}", before));
            }
            if let Some(after) = options.after {
                params.push(format!("after={}", after));
            }
            if let Some(per_page) = options.per_page {
                params.push(format!("per_page={}", per_page));
            }
            if !params.is_empty() {
                path = format!("{}?{}", path, params.join("&"));
            }
        }

        let request = self.client.new_request(reqwest::Method::GET, &path).await?;
        let response = request.send().await?.buildkite_error_for_status().await?;
        Ok(response.json().await?)
    }
}
