use super::common::{buildkite_client, json_response, setup_mock_server};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_agents_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/agents"))
        .respond_with(json_response(
            r#"[
            {
                "id": "agent-123",
                "name": "agent-1",
                "connection_state": "connected",
                "hostname": "host-1.example.com",
                "ip_address": "192.168.1.1",
                "user_agent": "buildkite-agent/3.40.0",
                "version": "3.40.0"
            },
            {
                "id": "agent-124",
                "name": "agent-2",
                "connection_state": "disconnected"
            }
        ]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let agents = client.agents.list("my-great-org").await.unwrap();

    assert_eq!(agents.len(), 2);
    let first = agents.first().unwrap();
    assert_eq!(first.id, Some("agent-123".to_string()));
    assert_eq!(first.name, Some("agent-1".to_string()));
    assert_eq!(first.connection_state, Some("connected".to_string()));
    assert_eq!(first.hostname, Some("host-1.example.com".to_string()));
    let second = agents.get(1).unwrap();
    assert_eq!(second.id, Some("agent-124".to_string()));
    assert_eq!(second.connection_state, Some("disconnected".to_string()));
}

#[tokio::test]
async fn test_agents_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/agents/agent-123"))
        .respond_with(json_response(
            r#"{
            "id": "agent-123",
            "graphql_id": "QWdlbnQtLS1hZ2VudC0xMjM=",
            "name": "my-agent",
            "connection_state": "connected",
            "hostname": "host.example.com",
            "ip_address": "192.168.1.100",
            "user_agent": "buildkite-agent/3.40.0",
            "version": "3.40.0",
            "created_at": "2023-01-01T12:00:00.000Z",
            "priority": 10,
            "meta_data": ["queue=default", "os=linux"]
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let agent = client
        .agents
        .get("my-great-org", "agent-123")
        .await
        .unwrap();

    assert_eq!(agent.id, Some("agent-123".to_string()));
    assert_eq!(
        agent.graphql_id,
        Some("QWdlbnQtLS1hZ2VudC0xMjM=".to_string())
    );
    assert_eq!(agent.name, Some("my-agent".to_string()));
    assert_eq!(agent.connection_state, Some("connected".to_string()));
    assert_eq!(agent.hostname, Some("host.example.com".to_string()));
    assert_eq!(agent.version, Some("3.40.0".to_string()));
    assert_eq!(agent.priority, Some(10));
    assert!(agent.meta_data.is_some());
    let meta_data = agent.meta_data.unwrap();
    assert_eq!(meta_data.len(), 2);
    assert!(meta_data.contains(&"queue=default".to_string()));
}

#[tokio::test]
async fn test_agents_stop() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PUT"))
        .and(path("/v2/organizations/my-great-org/agents/agent-123/stop"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client.agents.stop("my-great-org", "agent-123", false).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agents_pause() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PUT"))
        .and(path(
            "/v2/organizations/my-great-org/agents/agent-123/pause",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client.agents.pause("my-great-org", "agent-123", None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agents_resume() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PUT"))
        .and(path(
            "/v2/organizations/my-great-org/agents/agent-123/resume",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client.agents.resume("my-great-org", "agent-123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agents_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/organizations/my-great-org/agents/agent-123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client.agents.delete("my-great-org", "agent-123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agents_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/organizations/my-great-org/agents"))
        .respond_with(json_response(
            r#"{
            "id": "agent-new",
            "graphql_id": "QWdlbnQtLS1hZ2VudC1uZXc=",
            "name": "new-agent",
            "connection_state": "disconnected",
            "hostname": null,
            "ip_address": null,
            "user_agent": null,
            "version": "3.40.0",
            "created_at": "2023-08-01T12:00:00.000Z"
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let agent = rust_buildkite::Agent {
        name: Some("new-agent".to_string()),
        version: Some("3.40.0".to_string()),
        ..Default::default()
    };

    let result = client.agents.create("my-great-org", agent).await.unwrap();

    assert_eq!(result.id, Some("agent-new".to_string()));
    assert_eq!(result.name, Some("new-agent".to_string()));
    assert_eq!(result.version, Some("3.40.0".to_string()));
}
