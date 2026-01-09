mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_package_registries_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/packages/organizations/my-org/registries"))
        .respond_with(json_response(r#"[
            {
                "id": "0191e23a-4bc6-7683-bfa4-5f73bc9b7c44",
                "graphql_id": "UGFja2FnZXNSZWdpc3RyeS0tLTAxOTFlMjNhLTRiYzYtNzY4My1iZmE0LTVmNzNiYzliN2M0NA==",
                "slug": "my-ruby-gems",
                "url": "https://api.buildkite.com/v2/packages/organizations/my-org/registries/my-ruby-gems",
                "web_url": "https://buildkite.com/organizations/my-org/packages/my-ruby-gems",
                "name": "my-ruby-gems",
                "ecosystem": "rubygems",
                "description": "My RubyGems registry",
                "emoji": ":rubygems:",
                "color": null,
                "public": false,
                "oidc_policy": null
            },
            {
                "id": "0191df45-aae9-78b2-8db4-65f0ce3f0d0a",
                "graphql_id": "UGFja2FnZXNSZWdpc3RyeS0tLTAxOTFkZjQ1LWFhZTktNzhiMi04ZGI0LTY1ZjBjZTNmMGQwYQ==",
                "slug": "my-docker",
                "url": "https://api.buildkite.com/v2/packages/organizations/my-org/registries/my-docker",
                "web_url": "https://buildkite.com/organizations/my-org/packages/my-docker",
                "name": "my-docker",
                "ecosystem": "docker",
                "description": "My Docker registry",
                "emoji": ":docker:",
                "color": null,
                "public": false,
                "oidc_policy": null
            }
        ]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let registries = client.package_registries.list("my-org").await.unwrap();

    assert_eq!(registries.len(), 2);
    let first = registries.first().unwrap();
    assert_eq!(
        first.id,
        Some("0191e23a-4bc6-7683-bfa4-5f73bc9b7c44".to_string())
    );
    assert_eq!(first.slug, Some("my-ruby-gems".to_string()));
    assert_eq!(first.ecosystem, Some("rubygems".to_string()));
    let second = registries.get(1).unwrap();
    assert_eq!(
        second.id,
        Some("0191df45-aae9-78b2-8db4-65f0ce3f0d0a".to_string())
    );
    assert_eq!(second.slug, Some("my-docker".to_string()));
    assert_eq!(second.ecosystem, Some("docker".to_string()));
}

#[tokio::test]
async fn test_package_registries_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/packages/organizations/my-org/registries/my-ruby-gems"))
        .respond_with(json_response(r#"{
            "id": "0191e23a-4bc6-7683-bfa4-5f73bc9b7c44",
            "graphql_id": "UGFja2FnZXNSZWdpc3RyeS0tLTAxOTFlMjNhLTRiYzYtNzY4My1iZmE0LTVmNzNiYzliN2M0NA==",
            "slug": "my-ruby-gems",
            "url": "https://api.buildkite.com/v2/packages/organizations/my-org/registries/my-ruby-gems",
            "web_url": "https://buildkite.com/organizations/my-org/packages/my-ruby-gems",
            "name": "my-ruby-gems",
            "ecosystem": "rubygems",
            "description": "My RubyGems registry",
            "emoji": ":rubygems:",
            "color": null,
            "public": false,
            "oidc_policy": null
        }"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let registry = client
        .package_registries
        .get("my-org", "my-ruby-gems")
        .await
        .unwrap();

    assert_eq!(
        registry.id,
        Some("0191e23a-4bc6-7683-bfa4-5f73bc9b7c44".to_string())
    );
    assert_eq!(registry.slug, Some("my-ruby-gems".to_string()));
    assert_eq!(registry.name, Some("my-ruby-gems".to_string()));
    assert_eq!(registry.ecosystem, Some("rubygems".to_string()));
    assert_eq!(
        registry.description,
        Some("My RubyGems registry".to_string())
    );
}

#[tokio::test]
async fn test_package_registries_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/packages/organizations/my-org/registries"))
        .respond_with(json_response(r#"{
            "id": "0191e23a-4bc6-7683-bfa4-5f73bc9b7c44",
            "graphql_id": "UGFja2FnZXNSZWdpc3RyeS0tLTAxOTFlMjNhLTRiYzYtNzY4My1iZmE0LTVmNzNiYzliN2M0NA==",
            "slug": "my-new-registry",
            "url": "https://api.buildkite.com/v2/packages/organizations/my-org/registries/my-new-registry",
            "web_url": "https://buildkite.com/organizations/my-org/packages/my-new-registry",
            "name": "my-new-registry",
            "ecosystem": "rubygems",
            "description": "A new RubyGems registry",
            "emoji": ":rubygems:",
            "color": null,
            "public": false,
            "oidc_policy": null
        }"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let input = rust_buildkite::CreatePackageRegistryInput {
        name: Some("my-new-registry".to_string()),
        ecosystem: Some("rubygems".to_string()),
        description: Some("A new RubyGems registry".to_string()),
        ..Default::default()
    };

    let registry = client
        .package_registries
        .create("my-org", input)
        .await
        .unwrap();

    assert_eq!(registry.name, Some("my-new-registry".to_string()));
    assert_eq!(registry.ecosystem, Some("rubygems".to_string()));
    assert_eq!(
        registry.description,
        Some("A new RubyGems registry".to_string())
    );
}

#[tokio::test]
async fn test_package_registries_update() {
    let mock_server = setup_mock_server().await;

    let update_response = r##"{
        "id": "0191e23a-4bc6-7683-bfa4-5f73bc9b7c44",
        "graphql_id": "UGFja2FnZXNSZWdpc3RyeS0tLTAxOTFlMjNhLTRiYzYtNzY4My1iZmE0LTVmNzNiYzliN2M0NA==",
        "slug": "my-ruby-gems",
        "url": "https://api.buildkite.com/v2/packages/organizations/my-org/registries/my-ruby-gems",
        "web_url": "https://buildkite.com/organizations/my-org/packages/my-ruby-gems",
        "name": "my-ruby-gems",
        "ecosystem": "rubygems",
        "description": "Updated description",
        "emoji": ":gem:",
        "color": "#FF0000",
        "public": false,
        "oidc_policy": null
    }"##;

    Mock::given(method("PATCH"))
        .and(path(
            "/v2/packages/organizations/my-org/registries/my-ruby-gems",
        ))
        .respond_with(json_response(update_response))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let input = rust_buildkite::UpdatePackageRegistryInput {
        description: Some("Updated description".to_string()),
        emoji: Some(":gem:".to_string()),
        color: Some("#FF0000".to_string()),
        ..Default::default()
    };

    let registry = client
        .package_registries
        .update("my-org", "my-ruby-gems", input)
        .await
        .unwrap();

    assert_eq!(
        registry.description,
        Some("Updated description".to_string())
    );
    assert_eq!(registry.emoji, Some(":gem:".to_string()));
    assert_eq!(registry.color, Some("#FF0000".to_string()));
}

#[tokio::test]
async fn test_package_registries_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/v2/packages/organizations/my-org/registries/my-ruby-gems",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client
        .package_registries
        .delete("my-org", "my-ruby-gems")
        .await;

    assert!(result.is_ok());
}
