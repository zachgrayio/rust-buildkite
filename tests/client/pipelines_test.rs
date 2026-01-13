use super::common::{buildkite_client, json_response, setup_mock_server};
use rust_buildkite::{Client, CreatePipeline, PipelineListOptions, UpdatePipeline};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_pipelines_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipelines"))
        .respond_with(super::common::json_response(
            r#"[{"id":"123"},{"id":"1234"}]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let pipelines = client.pipelines.list("my-great-org").await.unwrap();

    assert_eq!(pipelines.len(), 2);
    assert_eq!(pipelines.first().unwrap().id, Some("123".to_string()));
    assert_eq!(pipelines.get(1).unwrap().id, Some("1234".to_string()));
}

#[tokio::test]
async fn test_pipelines_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/my-great-pipeline-slug",
        ))
        .respond_with(super::common::json_response(
            r#"{"id":"123", "slug":"my-great-pipeline-slug"}"#,
        ))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let pipeline = client
        .pipelines
        .get("my-great-org", "my-great-pipeline-slug")
        .await
        .unwrap();

    assert_eq!(pipeline.id, Some("123".to_string()));
    assert_eq!(pipeline.slug, Some("my-great-pipeline-slug".to_string()));
}

#[tokio::test]
async fn test_pipelines_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/organizations/my-great-org/pipelines"))
        .respond_with(json_response(
            r#"{
            "name":"my-great-pipeline",
            "repository":"my-great-repo",
            "cluster_id":"528000d8-4ee1-4479-8af1-032b143185f0",
            "default_branch":"main",
            "tags": ["well-tested", "great-config"]
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let create_pipeline = CreatePipeline {
        name: "my-great-pipeline".to_string(),
        repository: "my-great-repo".to_string(),
        cluster_id: "528000d8-4ee1-4479-8af1-032b143185f0".to_string(),
        configuration: None,
        steps: None,
        default_branch: Some("main".to_string()),
        default_command_step_timeout: None,
        description: None,
        env: None,
        maximum_command_step_timeout: None,
        provider_settings: None,
        branch_configuration: None,
        skip_queued_branch_builds: None,
        skip_queued_branch_builds_filter: None,
        cancel_running_branch_builds: None,
        cancel_running_branch_builds_filter: None,
        team_uuids: None,
        visibility: None,
        tags: Some(vec!["well-tested".to_string(), "great-config".to_string()]),
    };

    let pipeline = client
        .pipelines
        .create("my-great-org", create_pipeline)
        .await
        .unwrap();

    assert_eq!(pipeline.name, Some("my-great-pipeline".to_string()));
    assert_eq!(pipeline.repository, Some("my-great-repo".to_string()));
    assert_eq!(
        pipeline.cluster_id,
        Some("528000d8-4ee1-4479-8af1-032b143185f0".to_string())
    );
}

#[tokio::test]
async fn test_pipelines_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/my-great-pipeline-slug",
        ))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    client
        .pipelines
        .delete("my-great-org", "my-great-pipeline-slug")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_pipelines_list_by_name() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipelines"))
        .and(query_param("name", "my-pipeline"))
        .respond_with(json_response(r#"[{"id":"123","name":"my-pipeline"}]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let opts = PipelineListOptions {
        name: Some("my-pipeline".to_string()),
        ..Default::default()
    };

    let pipelines = client
        .pipelines
        .list_with_options("my-great-org", Some(opts))
        .await
        .unwrap();

    assert_eq!(pipelines.len(), 1);
    assert_eq!(
        pipelines.first().unwrap().name,
        Some("my-pipeline".to_string())
    );
}

#[tokio::test]
async fn test_pipelines_list_by_repository() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipelines"))
        .and(query_param("repository", "https://github.com/org/repo"))
        .respond_with(json_response(
            r#"[{"id":"123","repository":"https://github.com/org/repo"}]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let opts = PipelineListOptions {
        repository: Some("https://github.com/org/repo".to_string()),
        ..Default::default()
    };

    let pipelines = client
        .pipelines
        .list_with_options("my-great-org", Some(opts))
        .await
        .unwrap();

    assert_eq!(pipelines.len(), 1);
    assert_eq!(
        pipelines.first().unwrap().repository,
        Some("https://github.com/org/repo".to_string())
    );
}

#[tokio::test]
async fn test_pipelines_update() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path("/v2/organizations/my-great-org/pipelines/my-pipeline"))
        .respond_with(json_response(
            r#"{
            "id": "123",
            "name": "updated-pipeline",
            "description": "Updated description"
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let update = UpdatePipeline {
        name: Some("updated-pipeline".to_string()),
        description: Some("Updated description".to_string()),
        ..Default::default()
    };

    let pipeline = client
        .pipelines
        .update("my-great-org", "my-pipeline", update)
        .await
        .unwrap();

    assert_eq!(pipeline.name, Some("updated-pipeline".to_string()));
    assert_eq!(
        pipeline.description,
        Some("Updated description".to_string())
    );
}

#[tokio::test]
async fn test_pipelines_create_by_configuration() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/organizations/my-great-org/pipelines"))
        .respond_with(json_response(
            r#"{
            "id": "123",
            "name": "my-pipeline",
            "configuration": "steps:\n  - command: \"echo hello\""
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let create_pipeline = CreatePipeline {
        name: "my-pipeline".to_string(),
        repository: "git@github.com:org/repo.git".to_string(),
        cluster_id: "cluster-123".to_string(),
        configuration: Some("steps:\n  - command: \"echo hello\"".to_string()),
        steps: None,
        default_branch: None,
        default_command_step_timeout: None,
        description: None,
        env: None,
        maximum_command_step_timeout: None,
        provider_settings: None,
        branch_configuration: None,
        skip_queued_branch_builds: None,
        skip_queued_branch_builds_filter: None,
        cancel_running_branch_builds: None,
        cancel_running_branch_builds_filter: None,
        team_uuids: None,
        visibility: None,
        tags: None,
    };

    let pipeline = client
        .pipelines
        .create("my-great-org", create_pipeline)
        .await
        .unwrap();

    assert_eq!(
        pipeline.configuration,
        Some("steps:\n  - command: \"echo hello\"".to_string())
    );
}

#[tokio::test]
async fn test_pipelines_add_webhook() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/my-pipeline/webhook",
        ))
        .respond_with(ResponseTemplate::new(201))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let result = client
        .pipelines
        .add_webhook("my-great-org", "my-pipeline")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pipelines_archive() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/my-pipeline/archive",
        ))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let result = client
        .pipelines
        .archive("my-great-org", "my-pipeline")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pipelines_unarchive() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/my-pipeline/unarchive",
        ))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let result = client
        .pipelines
        .unarchive("my-great-org", "my-pipeline")
        .await;
    assert!(result.is_ok());
}
