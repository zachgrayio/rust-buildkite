use super::common::{buildkite_client, json_response, setup_mock_server};
use rust_buildkite::{Author, Build, BuildGetOptions, BuildsListOptions, Client, CreateBuild};
use std::collections::HashMap;
use wiremock::Mock;
use wiremock::matchers::{method, path, query_param};

#[tokio::test]
async fn test_builds_cancel() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PUT"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/sup-keith/builds/1/cancel",
        ))
        .respond_with(super::common::json_response(
            r#"{"id": "1", "state": "cancelled"}"#,
        ))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let build = client
        .builds
        .cancel("my-great-org", "sup-keith", "1")
        .await
        .unwrap();

    assert_eq!(build.id, Some("1".to_string()));
    assert_eq!(build.state, Some("cancelled".to_string()));
}

#[tokio::test]
async fn test_builds_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/builds"))
        .respond_with(super::common::json_response(
            r#"[{"id":"123"},{"id":"1234"}]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let builds = client.builds.list().await.unwrap();

    assert_eq!(builds.len(), 2);
    assert_eq!(builds.first().unwrap().id, Some("123".to_string()));
    assert_eq!(builds.get(1).unwrap().id, Some("1234".to_string()));
}

#[tokio::test]
async fn test_builds_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/sup-keith/builds/123",
        ))
        .respond_with(super::common::json_response(r#"{"id":"123"}"#))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let build = client
        .builds
        .get("my-great-org", "sup-keith", "123")
        .await
        .unwrap();

    assert_eq!(build.id, Some("123".to_string()));
}

#[tokio::test]
async fn test_builds_get_with_group_key() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/sup-keith/builds/123",
        ))
        .respond_with(super::common::json_response(
            r#"{"id":"123", "jobs": [ {"group_key": "job_group" }]}"#,
        ))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let build = client
        .builds
        .get("my-great-org", "sup-keith", "123")
        .await
        .unwrap();

    assert_eq!(build.id, Some("123".to_string()));
    assert_eq!(
        build.jobs.as_ref().unwrap().first().unwrap().group_key,
        Some("job_group".to_string())
    );
}

#[tokio::test]
async fn test_builds_list_by_org() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/builds"))
        .respond_with(super::common::json_response(
            r#"[{"id":"123"},{"id":"1234"}]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let builds = client.builds.list_by_org("my-great-org").await.unwrap();

    assert_eq!(builds.len(), 2);
    assert_eq!(builds.first().unwrap().id, Some("123".to_string()));
    assert_eq!(builds.get(1).unwrap().id, Some("1234".to_string()));
}

#[tokio::test]
async fn test_builds_list_by_pipeline() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/sup-keith/builds",
        ))
        .respond_with(super::common::json_response(
            r#"[{"id":"123"},{"id":"1234"}]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let builds = client
        .builds
        .list_by_pipeline("my-great-org", "sup-keith")
        .await
        .unwrap();

    assert_eq!(builds.len(), 2);
    assert_eq!(builds.first().unwrap().id, Some("123".to_string()));
    assert_eq!(builds.get(1).unwrap().id, Some("1234".to_string()));
}

#[tokio::test]
async fn test_builds_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/sup-keith/builds",
        ))
        .respond_with(json_response(r#"{"id":"123"}"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let create_build = CreateBuild {
        commit: "HEAD".to_string(),
        branch: "main".to_string(),
        message: "Hello, world!".to_string(),
        author: None,
        clean_checkout: None,
        env: None,
        meta_data: None,
        ignore_pipeline_branch_filters: None,
        pull_request_base_branch: None,
        pull_request_id: None,
        pull_request_repository: None,
    };

    let build = client
        .builds
        .create("my-great-org", "sup-keith", create_build)
        .await
        .unwrap();

    assert_eq!(build.id, Some("123".to_string()));
}

#[tokio::test]
async fn test_build_unmarshal_with_author_info() {
    let json = r#"{
        "id": "123",
        "state": "running",
        "blocked": false,
        "message": "Hello, world!",
        "commit": "HEAD",
        "branch": "main",
        "source": "ui",
        "author": {
            "username": "foojim",
            "name": "Uhh, Jim",
            "email": "foojim@example.com"
        }
    }"#;

    let build: Build = serde_json::from_str(json).unwrap();

    assert_eq!(build.id, Some("123".to_string()));
    assert_eq!(build.state, Some("running".to_string()));
    assert_eq!(build.blocked, Some(false));

    match build.author.unwrap() {
        Author::Object(author) => {
            assert_eq!(author.email, Some("foojim@example.com".to_string()));
            assert_eq!(author.name, Some("Uhh, Jim".to_string()));
            assert_eq!(author.username, Some("foojim".to_string()));
        }
        Author::String(_) => panic!("Expected Author::Object"),
    }
}

#[tokio::test]
async fn test_build_unmarshal_with_author_email_string() {
    let json = r#"{
        "id": "123",
        "state": "running",
        "blocked": false,
        "message": "Hello, world!",
        "commit": "HEAD",
        "branch": "main",
        "source": "ui",
        "author": "foojim@example.com"
    }"#;

    let build: Build = serde_json::from_str(json).unwrap();

    assert_eq!(build.id, Some("123".to_string()));

    match build.author.unwrap() {
        Author::String(email) => {
            assert_eq!(email, "foojim@example.com");
        }
        Author::Object(_) => panic!("Expected Author::String"),
    }
}

#[tokio::test]
async fn test_builds_list_by_status() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/builds"))
        .and(query_param("state[]", "running"))
        .respond_with(json_response(r#"[{"id":"123","state":"running"}]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let opts = BuildsListOptions {
        state: Some(vec!["running".to_string()]),
        ..Default::default()
    };

    let builds = client.builds.list_with_options(Some(opts)).await.unwrap();

    assert_eq!(builds.len(), 1);
    assert_eq!(builds.first().unwrap().state, Some("running".to_string()));
}

#[tokio::test]
async fn test_builds_list_by_multiple_status() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/builds"))
        .and(query_param("state[]", "running"))
        .respond_with(json_response(
            r#"[{"id":"123","state":"running"},{"id":"124","state":"scheduled"}]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let opts = BuildsListOptions {
        state: Some(vec!["running".to_string(), "scheduled".to_string()]),
        ..Default::default()
    };

    let builds = client.builds.list_with_options(Some(opts)).await.unwrap();

    assert_eq!(builds.len(), 2);
}

#[tokio::test]
async fn test_builds_list_by_branch() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/builds"))
        .and(query_param("branch[]", "main"))
        .respond_with(json_response(r#"[{"id":"123","branch":"main"}]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let opts = BuildsListOptions {
        branch: Some(vec!["main".to_string()]),
        ..Default::default()
    };

    let builds = client
        .builds
        .list_by_org_with_options("my-great-org", Some(opts))
        .await
        .unwrap();

    assert_eq!(builds.len(), 1);
    assert_eq!(builds.first().unwrap().branch, Some("main".to_string()));
}

#[tokio::test]
async fn test_builds_list_exclude_jobs() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/builds"))
        .and(query_param("exclude_jobs", "true"))
        .respond_with(json_response(r#"[{"id":"123"}]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let opts = BuildsListOptions {
        exclude_jobs: Some(true),
        ..Default::default()
    };

    let builds = client.builds.list_with_options(Some(opts)).await.unwrap();

    assert_eq!(builds.len(), 1);
    assert!(builds.first().unwrap().jobs.is_none());
}

#[tokio::test]
async fn test_builds_get_include_test_engine() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/sup-keith/builds/123",
        ))
        .and(query_param("include_test_engine", "true"))
        .respond_with(json_response(
            r#"{
            "id": "123",
            "test_engine": {
                "runs": [{"id": "run-123", "suite": {"id": "suite-123", "slug": "my-suite"}}]
            }
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let opts = BuildGetOptions {
        include_test_engine: Some(true),
        include_retried_jobs: None,
    };

    let build = client
        .builds
        .get_with_options("my-great-org", "sup-keith", "123", Some(opts))
        .await
        .unwrap();

    assert_eq!(build.id, Some("123".to_string()));
    assert!(build.test_engine.is_some());
    let test_engine = build.test_engine.unwrap();
    assert!(test_engine.runs.is_some());
    let runs = test_engine.runs.unwrap();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs.first().unwrap().id, Some("run-123".to_string()));
}

#[tokio::test]
async fn test_builds_rebuild() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PUT"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/sup-keith/builds/123/rebuild",
        ))
        .respond_with(json_response(
            r#"{"id":"124","rebuilt_from":{"id":"123","number":1}}"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let build = client
        .builds
        .rebuild("my-great-org", "sup-keith", "123")
        .await
        .unwrap();

    assert_eq!(build.id, Some("124".to_string()));
    assert!(build.rebuilt_from.is_some());
    let rebuilt_from = build.rebuilt_from.unwrap();
    assert_eq!(rebuilt_from.id, Some("123".to_string()));
}

#[tokio::test]
async fn test_build_unmarshal_with_test_engine() {
    let json = r#"{
        "id": "123",
        "test_engine": {
            "runs": [
                {
                    "id": "run-123",
                    "suite": {
                        "id": "suite-123",
                        "slug": "my-suite"
                    }
                }
            ]
        }
    }"#;

    let build: Build = serde_json::from_str(json).unwrap();

    assert_eq!(build.id, Some("123".to_string()));
    assert!(build.test_engine.is_some());
    let test_engine = build.test_engine.unwrap();
    assert!(test_engine.runs.is_some());
    let runs = test_engine.runs.unwrap();
    assert_eq!(runs.len(), 1);
    let first_run = runs.first().unwrap();
    assert_eq!(first_run.id, Some("run-123".to_string()));
    assert!(first_run.suite.is_some());
    let suite = first_run.suite.as_ref().unwrap();
    assert_eq!(suite.slug, Some("my-suite".to_string()));
}

#[tokio::test]
async fn test_builds_list_with_meta_data() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/builds"))
        .and(query_param("meta_data[env]", "production"))
        .respond_with(json_response(
            r#"[{"id":"123","meta_data":{"env":"production"}}]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let mut meta_data = HashMap::new();
    meta_data.insert("env".to_string(), "production".to_string());

    let opts = BuildsListOptions {
        meta_data: Some(meta_data),
        ..Default::default()
    };

    let builds = client.builds.list_with_options(Some(opts)).await.unwrap();

    assert_eq!(builds.len(), 1);
    assert_eq!(builds.first().unwrap().id, Some("123".to_string()));
}

#[tokio::test]
async fn test_builds_list_with_exclude_pipeline() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/builds"))
        .and(query_param("exclude_pipeline", "true"))
        .respond_with(json_response(r#"[{"id":"123"},{"id":"124"}]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let opts = BuildsListOptions {
        exclude_pipeline: Some(true),
        ..Default::default()
    };

    let builds = client
        .builds
        .list_by_org_with_options("my-great-org", Some(opts))
        .await
        .unwrap();

    assert_eq!(builds.len(), 2);
}
