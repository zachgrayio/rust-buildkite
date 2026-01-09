mod common;

use common::{buildkite_client, setup_mock_server};
use rust_buildkite::{ClusterQueueCreate, ClusterQueuePause, ClusterQueueUpdate};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_list_cluster_queues() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/queues",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": "queue-1",
                "key": "default",
                "description": "Default queue"
            },
            {
                "id": "queue-2",
                "key": "high-priority",
                "description": "High priority queue"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let queues = client
        .cluster_queues
        .list("test-org", "cluster-123")
        .await
        .unwrap();

    assert_eq!(queues.len(), 2);
    assert_eq!(queues.first().unwrap().id, Some("queue-1".to_string()));
    assert_eq!(queues.first().unwrap().key, Some("default".to_string()));
    assert_eq!(queues.get(1).unwrap().id, Some("queue-2".to_string()));
    assert_eq!(
        queues.get(1).unwrap().key,
        Some("high-priority".to_string())
    );
}

#[tokio::test]
async fn test_get_cluster_queue() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/queues/queue-456",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "queue-456",
            "key": "default",
            "description": "Default queue",
            "dispatch_paused": false
        })))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let queue = client
        .cluster_queues
        .get("test-org", "cluster-123", "queue-456")
        .await
        .unwrap();

    assert_eq!(queue.id, Some("queue-456".to_string()));
    assert_eq!(queue.key, Some("default".to_string()));
    assert_eq!(queue.dispatch_paused, Some(false));
}

#[tokio::test]
async fn test_create_cluster_queue() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/queues",
        ))
        .and(body_json(serde_json::json!({
            "key": "new-queue",
            "description": "A new queue"
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "queue-789",
            "key": "new-queue",
            "description": "A new queue",
            "dispatch_paused": false
        })))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let create = ClusterQueueCreate {
        key: Some("new-queue".to_string()),
        description: Some("A new queue".to_string()),
        retry_agent_affinity: None,
    };

    let queue = client
        .cluster_queues
        .create("test-org", "cluster-123", create)
        .await
        .unwrap();

    assert_eq!(queue.id, Some("queue-789".to_string()));
    assert_eq!(queue.key, Some("new-queue".to_string()));
}

#[tokio::test]
async fn test_update_cluster_queue() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/queues/queue-456",
        ))
        .and(body_json(serde_json::json!({
            "description": "Updated description"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "queue-456",
            "key": "default",
            "description": "Updated description"
        })))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let update = ClusterQueueUpdate {
        description: Some("Updated description".to_string()),
        retry_agent_affinity: None,
    };

    let queue = client
        .cluster_queues
        .update("test-org", "cluster-123", "queue-456", update)
        .await
        .unwrap();

    assert_eq!(queue.description, Some("Updated description".to_string()));
}

#[tokio::test]
async fn test_delete_cluster_queue() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/queues/queue-456",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client
        .cluster_queues
        .delete("test-org", "cluster-123", "queue-456")
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pause_cluster_queue() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/queues/queue-456/pause_dispatch",
        ))
        .and(body_json(serde_json::json!({
            "dispatch_paused_note": "Maintenance window"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "queue-456",
            "key": "default",
            "dispatch_paused": true,
            "dispatch_paused_note": "Maintenance window"
        })))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let pause = ClusterQueuePause {
        note: Some("Maintenance window".to_string()),
    };

    let queue = client
        .cluster_queues
        .pause("test-org", "cluster-123", "queue-456", pause)
        .await
        .unwrap();

    assert_eq!(queue.dispatch_paused, Some(true));
    assert_eq!(
        queue.dispatch_paused_note,
        Some("Maintenance window".to_string())
    );
}

#[tokio::test]
async fn test_resume_cluster_queue() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/test-org/clusters/cluster-123/queues/queue-456/resume_dispatch",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client
        .cluster_queues
        .resume("test-org", "cluster-123", "queue-456")
        .await;

    assert!(result.is_ok());
}
