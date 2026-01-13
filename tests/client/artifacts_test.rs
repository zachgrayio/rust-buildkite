use super::common::{buildkite_client, json_response, setup_mock_server};
use wiremock::Mock;
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_artifacts_list_by_build() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/artifacts"))
        .respond_with(json_response(r#"[
            {
                "id": "artifact-123",
                "job_id": "job-456",
                "url": "https://api.buildkite.com/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/jobs/job-456/artifacts/artifact-123",
                "download_url": "https://api.buildkite.com/v2/artifacts/download",
                "state": "finished",
                "path": "build/output.zip",
                "dirname": "build",
                "filename": "output.zip",
                "mime_type": "application/zip",
                "file_size": 1024,
                "sha1sum": "abc123"
            },
            {
                "id": "artifact-124",
                "job_id": "job-456",
                "filename": "logs.txt",
                "file_size": 256
            }
        ]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let artifacts = client
        .artifacts
        .list_by_build("my-great-org", "my-pipeline", "123")
        .await
        .unwrap();

    assert_eq!(artifacts.len(), 2);
    let first = artifacts.first().unwrap();
    assert_eq!(first.id, Some("artifact-123".to_string()));
    assert_eq!(first.job_id, Some("job-456".to_string()));
    assert_eq!(first.filename, Some("output.zip".to_string()));
    assert_eq!(first.file_size, Some(1024));
    assert_eq!(first.path, Some("build/output.zip".to_string()));
    assert_eq!(first.dirname, Some("build".to_string()));
    assert_eq!(first.mime_type, Some("application/zip".to_string()));
    assert_eq!(first.sha1, Some("abc123".to_string()));
    let second = artifacts.get(1).unwrap();
    assert_eq!(second.id, Some("artifact-124".to_string()));
    assert_eq!(second.filename, Some("logs.txt".to_string()));
}

#[tokio::test]
async fn test_artifacts_list_by_job() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/jobs/job-456/artifacts"))
        .respond_with(json_response(r#"[
            {
                "id": "artifact-123",
                "job_id": "job-456",
                "filename": "output.zip",
                "file_size": 1024
            }
        ]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let artifacts = client
        .artifacts
        .list_by_job("my-great-org", "my-pipeline", "123", "job-456")
        .await
        .unwrap();

    assert_eq!(artifacts.len(), 1);
    let first = artifacts.first().unwrap();
    assert_eq!(first.id, Some("artifact-123".to_string()));
    assert_eq!(first.job_id, Some("job-456".to_string()));
    assert_eq!(first.filename, Some("output.zip".to_string()));
    assert_eq!(first.file_size, Some(1024));
}

#[tokio::test]
async fn test_artifacts_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/jobs/job-456/artifacts/artifact-123"))
        .respond_with(json_response(r#"{
            "id": "artifact-123",
            "job_id": "job-456",
            "url": "https://api.buildkite.com/v2/organizations/my-great-org/pipelines/my-pipeline/builds/123/jobs/job-456/artifacts/artifact-123",
            "download_url": "https://api.buildkite.com/v2/artifacts/download",
            "state": "finished",
            "path": "build/output.zip",
            "dirname": "build",
            "filename": "output.zip",
            "mime_type": "application/zip",
            "file_size": 1024,
            "glob_path": "build/*",
            "original_path": "/tmp/build/output.zip",
            "sha1sum": "abc123"
        }"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let artifact = client
        .artifacts
        .get(
            "my-great-org",
            "my-pipeline",
            "123",
            "job-456",
            "artifact-123",
        )
        .await
        .unwrap();

    assert_eq!(artifact.id, Some("artifact-123".to_string()));
    assert_eq!(artifact.job_id, Some("job-456".to_string()));
    assert_eq!(artifact.filename, Some("output.zip".to_string()));
    assert_eq!(artifact.file_size, Some(1024));
    assert_eq!(artifact.state, Some("finished".to_string()));
    assert_eq!(artifact.path, Some("build/output.zip".to_string()));
    assert_eq!(artifact.dirname, Some("build".to_string()));
    assert_eq!(artifact.mime_type, Some("application/zip".to_string()));
    assert_eq!(artifact.glob_path, Some("build/*".to_string()));
    assert_eq!(
        artifact.original_path,
        Some("/tmp/build/output.zip".to_string())
    );
    assert_eq!(artifact.sha1, Some("abc123".to_string()));
}

#[tokio::test]
async fn test_artifacts_download() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/artifacts/download/test-file"))
        .respond_with(
            wiremock::ResponseTemplate::new(200).set_body_bytes(b"Hello, artifact content!"),
        )
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let download_url = format!("{}/artifacts/download/test-file", mock_server.uri());
    let bytes = client
        .artifacts
        .download_artifact_by_url(&download_url)
        .await
        .unwrap();

    assert_eq!(bytes, b"Hello, artifact content!");
}
