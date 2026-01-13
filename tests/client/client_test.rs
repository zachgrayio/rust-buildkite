use super::common::{buildkite_client, json_response, setup_mock_server};
use rust_buildkite::Client;
use wiremock::Mock;
use wiremock::matchers::{header, method, path};

#[tokio::test]
async fn test_bearer_auth_header() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/test-org/pipelines"))
        .and(header("Authorization", "Bearer test-token"))
        .respond_with(json_response("[]"))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client.pipelines.list("test-org").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_custom_user_agent() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/test-org/pipelines"))
        .and(header("User-Agent", "my-custom-agent/1.0"))
        .respond_with(json_response("[]"))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .base_url(format!("{}/", mock_server.uri()))
        .user_agent("my-custom-agent/1.0")
        .build();

    let result = client.pipelines.list("test-org").await;
    assert!(result.is_ok());
}

#[test]
fn test_response_pagination_link_parsing() {
    use rust_buildkite::Response;

    let link_header = r#"<https://api.buildkite.com/?page=1>; rel="first", <https://api.buildkite.com/?page=2>; rel="prev", <https://api.buildkite.com/?page=4>; rel="next", <https://api.buildkite.com/?page=5>; rel="last""#;

    let response = Response::with_pagination(vec![1, 2, 3], Some(link_header));

    assert_eq!(response.first_page, Some(1));
    assert_eq!(response.prev_page, Some(2));
    assert_eq!(response.next_page, Some(4));
    assert_eq!(response.last_page, Some(5));
    assert_eq!(response.data, vec![1, 2, 3]);
}

#[test]
fn test_response_without_pagination() {
    use rust_buildkite::Response;

    let response = Response::new(vec![1, 2, 3]);

    assert_eq!(response.first_page, None);
    assert_eq!(response.prev_page, None);
    assert_eq!(response.next_page, None);
    assert_eq!(response.last_page, None);
    assert_eq!(response.data, vec![1, 2, 3]);
}

#[test]
fn test_resolve_url_unit_standard_base_url() {
    let client = Client::builder("test-token")
        .base_url("https://api.buildkite.com/")
        .build();

    let url = client
        .test_resolve_url("v2/organizations/myorg/pipelines")
        .unwrap();
    assert_eq!(
        url.as_str(),
        "https://api.buildkite.com/v2/organizations/myorg/pipelines"
    );
}

#[test]
fn test_resolve_url_unit_base_url_no_trailing_slash() {
    let client = Client::builder("test-token")
        .base_url("https://api.buildkite.com")
        .build();

    let url = client
        .test_resolve_url("v2/organizations/myorg/pipelines")
        .unwrap();
    assert_eq!(
        url.as_str(),
        "https://api.buildkite.com/v2/organizations/myorg/pipelines"
    );
}

#[test]
fn test_resolve_url_unit_with_path_prefix() {
    let client = Client::builder("test-token")
        .base_url("https://proxy.example.com/api/")
        .build();

    let url = client
        .test_resolve_url("v2/organizations/myorg/pipelines")
        .unwrap();
    assert_eq!(
        url.as_str(),
        "https://proxy.example.com/api/v2/organizations/myorg/pipelines"
    );
}

#[test]
fn test_resolve_url_unit_with_path_prefix_no_trailing_slash() {
    let client = Client::builder("test-token")
        .base_url("https://proxy.example.com/api")
        .build();

    let url = client
        .test_resolve_url("v2/organizations/myorg/pipelines")
        .unwrap();
    assert_eq!(
        url.as_str(),
        "https://proxy.example.com/api/v2/organizations/myorg/pipelines"
    );
}

#[test]
fn test_resolve_url_unit_with_deep_path_prefix() {
    let client = Client::builder("test-token")
        .base_url("https://gateway.example.com/internal/buildkite/")
        .build();

    let url = client
        .test_resolve_url("v2/organizations/myorg/builds")
        .unwrap();
    assert_eq!(
        url.as_str(),
        "https://gateway.example.com/internal/buildkite/v2/organizations/myorg/builds"
    );
}

#[test]
fn test_resolve_url_unit_relative_path_with_leading_slash() {
    let client = Client::builder("test-token")
        .base_url("https://proxy.example.com/api/")
        .build();

    let url = client
        .test_resolve_url("/v2/organizations/myorg/pipelines")
        .unwrap();
    assert_eq!(
        url.as_str(),
        "https://proxy.example.com/api/v2/organizations/myorg/pipelines"
    );
}

#[test]
fn test_resolve_url_unit_empty_relative_path() {
    let client = Client::builder("test-token")
        .base_url("https://proxy.example.com/api/")
        .build();

    let url = client.test_resolve_url("").unwrap();
    assert_eq!(url.as_str(), "https://proxy.example.com/api/");
}

#[test]
fn test_resolve_url_unit_with_query_parameters() {
    let client = Client::builder("test-token")
        .base_url("https://proxy.example.com/api/")
        .build();

    let url = client
        .test_resolve_url("v2/organizations/myorg/pipelines?page=2&per_page=50")
        .unwrap();
    assert_eq!(
        url.as_str(),
        "https://proxy.example.com/api/v2/organizations/myorg/pipelines?page=2&per_page=50"
    );
}

#[test]
fn test_resolve_url_unit_with_fragment() {
    let client = Client::builder("test-token")
        .base_url("https://proxy.example.com/api/")
        .build();

    let url = client
        .test_resolve_url("v2/organizations/myorg/pipelines#section")
        .unwrap();
    assert_eq!(
        url.as_str(),
        "https://proxy.example.com/api/v2/organizations/myorg/pipelines#section"
    );
}

#[test]
fn test_resolve_url_unit_absolute_url_preserved() {
    let client = Client::builder("test-token")
        .base_url("https://proxy.example.com/api/")
        .build();

    let url = client
        .test_resolve_url("https://other.example.com/v2/test")
        .unwrap();
    assert_eq!(url.as_str(), "https://other.example.com/v2/test");
}

#[test]
fn test_resolve_url_unit_invalid_url_error() {
    let client = Client::builder("test-token")
        .base_url("https://proxy.example.com/api/")
        .build();

    let result = client.test_resolve_url("://invalid-url");
    assert!(result.is_err());
}

#[test]
fn test_resolve_url_unit_root_base_url() {
    let client = Client::builder("test-token")
        .base_url("https://proxy.example.com/")
        .build();

    let url = client
        .test_resolve_url("v2/organizations/myorg/pipelines")
        .unwrap();
    assert_eq!(
        url.as_str(),
        "https://proxy.example.com/v2/organizations/myorg/pipelines"
    );
}

#[tokio::test]
async fn test_resolve_url_integration_with_path_prefix() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/api/v2/organizations/myorg/pipelines"))
        .respond_with(json_response("[]"))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .base_url(format!("{}/api/", mock_server.uri()))
        .build();

    let result = client.pipelines.list("myorg").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resolve_url_integration_with_deep_path_prefix() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/internal/buildkite/v2/organizations/myorg/builds"))
        .respond_with(json_response("[]"))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .base_url(format!("{}/internal/buildkite/", mock_server.uri()))
        .build();

    let result = client.builds.list_by_org("myorg").await;
    assert!(result.is_ok());
}
