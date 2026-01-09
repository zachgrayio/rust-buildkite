mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use wiremock::Mock;
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_organizations_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations"))
        .respond_with(json_response(
            r#"[
            {
                "id": "org-123",
                "graphql_id": "T3JnYW5pemF0aW9uLS0tb3JnLTEyMw==",
                "url": "https://api.buildkite.com/v2/organizations/my-org",
                "web_url": "https://buildkite.com/my-org",
                "name": "My Organization",
                "slug": "my-org",
                "pipelines_url": "https://api.buildkite.com/v2/organizations/my-org/pipelines",
                "agents_url": "https://api.buildkite.com/v2/organizations/my-org/agents",
                "created_at": "2023-01-01T12:00:00.000Z"
            },
            {
                "id": "org-456",
                "name": "Another Org",
                "slug": "another-org"
            }
        ]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let orgs = client.organizations.list().await.unwrap();

    assert_eq!(orgs.len(), 2);
    let first = orgs.first().unwrap();
    assert_eq!(first.id, Some("org-123".to_string()));
    assert_eq!(first.name, Some("My Organization".to_string()));
    assert_eq!(first.slug, Some("my-org".to_string()));
    let second = orgs.get(1).unwrap();
    assert_eq!(second.id, Some("org-456".to_string()));
    assert_eq!(second.slug, Some("another-org".to_string()));
}

#[tokio::test]
async fn test_organizations_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-org"))
        .respond_with(json_response(
            r#"{
            "id": "org-123",
            "graphql_id": "T3JnYW5pemF0aW9uLS0tb3JnLTEyMw==",
            "url": "https://api.buildkite.com/v2/organizations/my-org",
            "web_url": "https://buildkite.com/my-org",
            "name": "My Organization",
            "slug": "my-org",
            "pipelines_url": "https://api.buildkite.com/v2/organizations/my-org/pipelines",
            "agents_url": "https://api.buildkite.com/v2/organizations/my-org/agents",
            "created_at": "2023-01-01T12:00:00.000Z"
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let org = client.organizations.get("my-org").await.unwrap();

    assert_eq!(org.id, Some("org-123".to_string()));
    assert_eq!(
        org.graphql_id,
        Some("T3JnYW5pemF0aW9uLS0tb3JnLTEyMw==".to_string())
    );
    assert_eq!(org.name, Some("My Organization".to_string()));
    assert_eq!(org.slug, Some("my-org".to_string()));
}
