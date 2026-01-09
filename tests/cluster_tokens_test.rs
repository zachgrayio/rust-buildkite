mod common;

use common::{buildkite_client, setup_mock_server};
use rust_buildkite::ClusterTokenCreateUpdate;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_list_cluster_tokens() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/tokens",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": "token-1",
                "description": "Agent token 1"
            },
            {
                "id": "token-2",
                "description": "Agent token 2"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let tokens = client
        .cluster_tokens
        .list("test-org", "cluster-123")
        .await
        .unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens.first().unwrap().id, Some("token-1".to_string()));
    assert_eq!(tokens.get(1).unwrap().id, Some("token-2".to_string()));
}

#[tokio::test]
async fn test_get_cluster_token() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/tokens/token-456",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "token-456",
            "description": "Agent token",
            "allowed_ip_addresses": "0.0.0.0/0"
        })))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let token = client
        .cluster_tokens
        .get("test-org", "cluster-123", "token-456")
        .await
        .unwrap();

    assert_eq!(token.id, Some("token-456".to_string()));
    assert_eq!(token.description, Some("Agent token".to_string()));
}

#[tokio::test]
async fn test_create_cluster_token() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/tokens",
        ))
        .and(body_json(serde_json::json!({
            "description": "New agent token"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "token-789",
            "description": "New agent token",
            "token": "secret-token-value"
        })))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let create = ClusterTokenCreateUpdate {
        description: Some("New agent token".to_string()),
        allowed_ip_addresses: None,
    };

    let token = client
        .cluster_tokens
        .create("test-org", "cluster-123", create)
        .await
        .unwrap();

    assert_eq!(token.id, Some("token-789".to_string()));
    assert_eq!(token.token, Some("secret-token-value".to_string()));
}

#[tokio::test]
async fn test_update_cluster_token() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/tokens/token-456",
        ))
        .and(body_json(serde_json::json!({
            "description": "Updated description"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "token-456",
            "description": "Updated description"
        })))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let update = ClusterTokenCreateUpdate {
        description: Some("Updated description".to_string()),
        allowed_ip_addresses: None,
    };

    let token = client
        .cluster_tokens
        .update("test-org", "cluster-123", "token-456", update)
        .await
        .unwrap();

    assert_eq!(token.description, Some("Updated description".to_string()));
}

#[tokio::test]
async fn test_delete_cluster_token() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/tokens/token-456",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client
        .cluster_tokens
        .delete("test-org", "cluster-123", "token-456")
        .await;

    assert!(result.is_ok());
}
