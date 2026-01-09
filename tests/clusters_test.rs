mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use rust_buildkite::{ClusterCreate, ClusterUpdate};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_clusters_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/clusters"))
        .respond_with(json_response(r##"[
            {
                "id": "528000d8-4ee1-4479-8af1-032b143185f0",
                "graphql_id": "Q2x1c3Rlci0tLTUyODAwMGQ4LTRlZTEtNDQ3OS04YWYxLTAzMmIxNDMxODVmMA==",
                "name": "Development Cluster",
                "description": "A cluster for development pipelines",
                "emoji": ":toolbox:",
                "color": "#A9CCE3",
                "maintainers": {
                    "users": [],
                    "teams": []
                },
                "url": "https://api.buildkite.com/v2/organizations/my-great-org/clusters/528000d8-4ee1-4479-8af1-032b143185f0",
                "web_url": "https://buildkite.com/organizations/my-great-org/clusters/528000d8-4ee1-4479-8af1-032b143185f0",
                "queues_url": "https://api.buildkite.com/v2/organizations/my-great-org/clusters/528000d8-4ee1-4479-8af1-032b143185f0/queues",
                "created_at": "2023-09-01T04:27:11.392Z",
                "created_by": {
                    "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                    "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                    "name": "Joe Smith",
                    "email": "jsmith@example.com",
                    "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                    "created_at": "2023-02-20T03:00:05.824Z"
                }
            },
            {
                "id": "3edcecdb-5191-44f1-a5ae-370083c8f92e",
                "graphql_id": "Q2x1c3Rlci0tLTNlZGNlY2RiLTUxOTEtNDRmMS1hNWFlLTM3MDA4M2M4ZjkyZQ==",
                "name": "Production Cluster",
                "description": "A cluster for production pipelines",
                "emoji": ":toolbox:",
                "color": "#B9E3A9",
                "maintainers": {
                    "users": [],
                    "teams": []
                },
                "url": "https://api.buildkite.com/v2/organizations/my-great-org/clusters/3edcecdb-5191-44f1-a5ae-370083c8f92e",
                "web_url": "https://buildkite.com/organizations/my-great-org/clusters/3edcecdb-5191-44f1-a5ae-370083c8f92e",
                "queues_url": "https://api.buildkite.com/v2/organizations/my-great-org/clusters/3edcecdb-5191-44f1-a5ae-370083c8f92e/queues",
                "created_at": "2023-09-04T04:25:55.751Z",
                "created_by": {
                    "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                    "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                    "name": "Joe Smith",
                    "email": "jsmith@example.com",
                    "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                    "created_at": "2023-02-20T03:00:05.824Z"
                }
            }
        ]"##))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let clusters = client.clusters.list("my-great-org").await.unwrap();

    assert_eq!(clusters.len(), 2);
    let first = clusters.first().unwrap();
    assert_eq!(
        first.id,
        Some("528000d8-4ee1-4479-8af1-032b143185f0".to_string())
    );
    assert_eq!(
        first.graphql_id,
        Some("Q2x1c3Rlci0tLTUyODAwMGQ4LTRlZTEtNDQ3OS04YWYxLTAzMmIxNDMxODVmMA==".to_string())
    );
    assert_eq!(first.name, Some("Development Cluster".to_string()));
    assert_eq!(first.emoji, Some(":toolbox:".to_string()));
    assert_eq!(first.color, Some("#A9CCE3".to_string()));
    assert!(first.created_by.is_some());

    let creator = first.created_by.as_ref().unwrap();
    assert_eq!(
        creator.id,
        Some("7da07e25-0383-4aff-a7cf-14d1a9aa098f".to_string())
    );
    assert_eq!(creator.name, Some("Joe Smith".to_string()));

    let second = clusters.get(1).unwrap();
    assert_eq!(
        second.id,
        Some("3edcecdb-5191-44f1-a5ae-370083c8f92e".to_string())
    );
    assert_eq!(second.name, Some("Production Cluster".to_string()));
}

#[tokio::test]
async fn test_clusters_get() {
    let mock_server = setup_mock_server().await;

    let response_json = r##"{
        "id": "123",
        "graphql_id": "Q2x1c3Rlci0tLTEyMw==",
        "name": "Development Cluster",
        "description": "A cluster for development work",
        "emoji": ":toolbox:",
        "color": "#A9CCE3",
        "url": "https://api.buildkite.com/v2/organizations/my-great-org/clusters/123",
        "web_url": "https://buildkite.com/organizations/my-great-org/clusters/123",
        "queues_url": "https://api.buildkite.com/v2/organizations/my-great-org/clusters/123/queues",
        "created_at": "2023-08-01T12:00:00.000Z",
        "created_by": {
            "id": "user-123",
            "name": "Test User",
            "email": "test@example.com"
        }
    }"##;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/clusters/123"))
        .respond_with(json_response(response_json))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let cluster = client.clusters.get("my-great-org", "123").await.unwrap();

    assert_eq!(cluster.id, Some("123".to_string()));
    assert_eq!(cluster.graphql_id, Some("Q2x1c3Rlci0tLTEyMw==".to_string()));
    assert_eq!(cluster.name, Some("Development Cluster".to_string()));
    assert_eq!(
        cluster.description,
        Some("A cluster for development work".to_string())
    );
    assert_eq!(cluster.emoji, Some(":toolbox:".to_string()));
    assert_eq!(cluster.color, Some("#A9CCE3".to_string()));
    assert!(cluster.created_by.is_some());
    let creator = cluster.created_by.unwrap();
    assert_eq!(creator.id, Some("user-123".to_string()));
    assert_eq!(creator.name, Some("Test User".to_string()));
}

#[tokio::test]
async fn test_clusters_create() {
    let mock_server = setup_mock_server().await;

    let response_json = r##"{
        "id": "123",
        "name": "Development Cluster",
        "description": "A cluster for development work",
        "emoji": ":toolbox:",
        "color": "#A9CCE3"
    }"##;

    Mock::given(method("POST"))
        .and(path("/v2/organizations/my-great-org/clusters"))
        .respond_with(json_response(response_json))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let create_cluster = ClusterCreate {
        name: "Development Cluster".to_string(),
        description: Some("A cluster for development work".to_string()),
        emoji: Some(":toolbox:".to_string()),
        color: Some("#A9CCE3".to_string()),
        maintainers: None,
    };

    let cluster = client
        .clusters
        .create("my-great-org", create_cluster)
        .await
        .unwrap();

    assert_eq!(cluster.id, Some("123".to_string()));
    assert_eq!(cluster.name, Some("Development Cluster".to_string()));
    assert_eq!(
        cluster.description,
        Some("A cluster for development work".to_string())
    );
    assert_eq!(cluster.emoji, Some(":toolbox:".to_string()));
    assert_eq!(cluster.color, Some("#A9CCE3".to_string()));
}

#[tokio::test]
async fn test_clusters_update() {
    let mock_server = setup_mock_server().await;

    let response_json =
        r#"{"id": "123", "name": "Updated Cluster", "description": "Updated description"}"#;

    Mock::given(method("PATCH"))
        .and(path("/v2/organizations/my-great-org/clusters/123"))
        .respond_with(json_response(response_json))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let update = ClusterUpdate {
        name: Some("Updated Cluster".to_string()),
        description: Some("Updated description".to_string()),
        emoji: None,
        color: None,
        default_queue_id: None,
    };

    let cluster = client
        .clusters
        .update("my-great-org", "123", update)
        .await
        .unwrap();

    assert_eq!(cluster.id, Some("123".to_string()));
    assert_eq!(cluster.name, Some("Updated Cluster".to_string()));
    assert_eq!(cluster.description, Some("Updated description".to_string()));
}

#[tokio::test]
async fn test_clusters_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/organizations/my-great-org/clusters/123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client.clusters.delete("my-great-org", "123").await;

    assert!(result.is_ok());
}
