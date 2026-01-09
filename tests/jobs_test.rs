mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use wiremock::Mock;
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_jobs_unblock() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PUT"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/jobs/job-456/unblock",
        ))
        .respond_with(json_response(
            r#"{
            "id": "job-456",
            "type": "manual",
            "name": "Deploy to Production",
            "state": "unblocked"
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let job = client
        .jobs
        .unblock_job("my-great-org", "my-pipeline", "123", "job-456", None)
        .await
        .unwrap();

    assert_eq!(job.id, Some("job-456".to_string()));
    assert_eq!(job.state, Some("unblocked".to_string()));
}

#[tokio::test]
async fn test_jobs_retry() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PUT"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/jobs/job-456/retry",
        ))
        .respond_with(json_response(
            r#"{
            "id": "job-789",
            "type": "script",
            "state": "scheduled",
            "retried_in_job_id": null,
            "retry_source": {
                "job_id": "job-456",
                "retry_type": "manual"
            }
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let job = client
        .jobs
        .retry_job("my-great-org", "my-pipeline", "123", "job-456")
        .await
        .unwrap();

    assert_eq!(job.id, Some("job-789".to_string()));
    assert_eq!(job.state, Some("scheduled".to_string()));
    assert!(job.retry_source.is_some());
    let retry_source = job.retry_source.unwrap();
    assert_eq!(retry_source.job_id, Some("job-456".to_string()));
    assert_eq!(retry_source.retry_type, Some("manual".to_string()));
}

#[tokio::test]
async fn test_jobs_get_log() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/jobs/job-456/log"))
        .respond_with(json_response(r#"{
            "url": "https://api.buildkite.com/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/jobs/job-456/log",
            "content": "Hello, world!\nBuild complete.",
            "size": 28,
            "header_times": [1609459200000, 1609459260000]
        }"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let log = client
        .jobs
        .get_job_log("my-great-org", "my-pipeline", "123", "job-456")
        .await
        .unwrap();

    assert!(log.content.is_some());
    assert_eq!(log.content.unwrap(), "Hello, world!\nBuild complete.");
    assert_eq!(log.size, Some(28));
    assert!(log.header_times.is_some());
    let header_times = log.header_times.unwrap();
    assert_eq!(header_times.len(), 2);
}

#[tokio::test]
async fn test_jobs_get_environment_variables() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/jobs/job-456/env",
        ))
        .respond_with(json_response(
            r#"{
            "env": {
                "BUILDKITE_BRANCH": "main",
                "BUILDKITE_COMMIT": "abc123",
                "MY_SECRET": "hunter2"
            }
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let envs = client
        .jobs
        .get_job_environment_variables("my-great-org", "my-pipeline", "123", "job-456")
        .await
        .unwrap();

    assert!(envs.environment_variables.is_some());
    let env_vars = envs.environment_variables.unwrap();
    assert_eq!(env_vars.get("BUILDKITE_BRANCH"), Some(&"main".to_string()));
    assert_eq!(
        env_vars.get("BUILDKITE_COMMIT"),
        Some(&"abc123".to_string())
    );
}
