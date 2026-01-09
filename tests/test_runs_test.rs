mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use rust_buildkite::FailedExecutionsOptions;
use wiremock::Mock;
use wiremock::matchers::{method, path, query_param};

#[tokio::test]
async fn test_test_runs_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/analytics/organizations/my-org/suites/my-suite/runs"))
        .respond_with(json_response(
            r#"[
            {
                "id": "run-123",
                "url": "https://api.buildkite.com/v2/analytics/organizations/my-org/suites/my-suite/runs/run-123",
                "web_url": "https://buildkite.com/organizations/my-org/analytics/suites/my-suite/runs/run-123",
                "branch": "main",
                "commit_sha": "abc123",
                "created_at": "2023-01-01T12:00:00.000Z",
                "state": "finished",
                "result": "passed",
                "build_id": "build-456"
            },
            {
                "id": "run-124",
                "branch": "feature",
                "state": "running"
            }
        ]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let runs = client.test_runs.list("my-org", "my-suite").await.unwrap();

    assert_eq!(runs.len(), 2);
    let first = runs.first().unwrap();
    assert_eq!(first.id, Some("run-123".to_string()));
    assert_eq!(first.branch, Some("main".to_string()));
    assert_eq!(first.state, Some("finished".to_string()));
    assert_eq!(first.result, Some("passed".to_string()));
    assert_eq!(first.build_id, Some("build-456".to_string()));
    let second = runs.get(1).unwrap();
    assert_eq!(second.id, Some("run-124".to_string()));
    assert_eq!(second.state, Some("running".to_string()));
}

#[tokio::test]
async fn test_test_runs_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/analytics/organizations/my-org/suites/my-suite/runs/run-123",
        ))
        .respond_with(json_response(
            r#"{
            "id": "run-123",
            "url": "https://api.buildkite.com/v2/analytics/organizations/my-org/suites/my-suite/runs/run-123",
            "web_url": "https://buildkite.com/organizations/my-org/analytics/suites/my-suite/runs/run-123",
            "branch": "main",
            "commit_sha": "abc123def456",
            "created_at": "2023-01-01T12:00:00.000Z",
            "state": "finished",
            "result": "passed",
            "build_id": "build-789"
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let run = client
        .test_runs
        .get("my-org", "my-suite", "run-123")
        .await
        .unwrap();

    assert_eq!(run.id, Some("run-123".to_string()));
    assert_eq!(run.branch, Some("main".to_string()));
    assert_eq!(run.commit_sha, Some("abc123def456".to_string()));
    assert_eq!(run.state, Some("finished".to_string()));
    assert_eq!(run.result, Some("passed".to_string()));
    assert_eq!(run.build_id, Some("build-789".to_string()));
}

#[tokio::test]
async fn test_test_runs_get_failed_executions() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/analytics/organizations/my-org/suites/my-suite/runs/run-123/failed_executions",
        ))
        .respond_with(json_response(
            r#"[
            {
                "execution_id": "exec-1",
                "run_id": "run-123",
                "test_id": "test-456",
                "run_name": "Run #1",
                "commit_sha": "abc123",
                "branch": "main",
                "failure_reason": "Assertion failed",
                "duration": 1.5,
                "location": "tests/example_test.rb:10",
                "test_name": "test_something"
            }
        ]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let executions = client
        .test_runs
        .get_failed_executions("my-org", "my-suite", "run-123", None)
        .await
        .unwrap();

    assert_eq!(executions.len(), 1);
    let exec = executions.first().unwrap();
    assert_eq!(exec.execution_id, Some("exec-1".to_string()));
    assert_eq!(exec.run_id, Some("run-123".to_string()));
    assert_eq!(exec.test_id, Some("test-456".to_string()));
    assert_eq!(exec.failure_reason, Some("Assertion failed".to_string()));
    assert_eq!(exec.duration, Some(1.5));
}

#[tokio::test]
async fn test_test_runs_get_failed_executions_with_expanded() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/analytics/organizations/my-org/suites/my-suite/runs/run-123/failed_executions",
        ))
        .and(query_param("include_failure_expanded", "true"))
        .respond_with(json_response(
            r#"[
            {
                "execution_id": "exec-1",
                "run_id": "run-123",
                "test_id": "test-456",
                "failure_reason": "Assertion failed",
                "failure_expanded": [
                    {
                        "backtrace": ["line1", "line2"],
                        "expanded": ["expanded1", "expanded2"]
                    }
                ]
            }
        ]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let opts = FailedExecutionsOptions {
        include_failure_expanded: Some(true),
    };
    let executions = client
        .test_runs
        .get_failed_executions("my-org", "my-suite", "run-123", Some(opts))
        .await
        .unwrap();

    assert_eq!(executions.len(), 1);
    let exec = executions.first().unwrap();
    assert!(exec.failure_expanded.is_some());
    let expanded = exec.failure_expanded.as_ref().unwrap();
    assert_eq!(expanded.len(), 1);
    let first = expanded.first().unwrap();
    assert!(first.backtrace.is_some());
    let backtrace = first.backtrace.as_ref().unwrap();
    assert_eq!(backtrace.len(), 2);
}
