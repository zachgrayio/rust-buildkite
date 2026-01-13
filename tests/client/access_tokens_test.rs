use super::common::{buildkite_client, json_response, setup_mock_server};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_access_tokens_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/access-token"))
        .respond_with(json_response(
            r#"{
            "uuid": "token-uuid-123",
            "scopes": ["read_agents", "write_pipelines"],
            "description": "My API Token",
            "created_at": "2023-01-01T12:00:00.000Z",
            "user": {
                "name": "Test User",
                "email": "test@example.com"
            }
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let token = client.access_tokens.get().await.unwrap();

    assert_eq!(token.uuid, Some("token-uuid-123".to_string()));
    assert!(token.scopes.is_some());
    let scopes = token.scopes.unwrap();
    assert_eq!(scopes.len(), 2);
    assert!(scopes.contains(&"read_agents".to_string()));
    assert!(scopes.contains(&"write_pipelines".to_string()));
    assert_eq!(token.description, Some("My API Token".to_string()));
    assert!(token.user.is_some());
    let user = token.user.unwrap();
    assert_eq!(user.name, Some("Test User".to_string()));
    assert_eq!(user.email, Some("test@example.com".to_string()));
}

#[tokio::test]
async fn test_access_tokens_revoke() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/access-token"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client.access_tokens.revoke().await;

    assert!(result.is_ok());
}
