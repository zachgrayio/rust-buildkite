use super::common::{buildkite_client, json_response, setup_mock_server};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_team_pipelines_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/my-org/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/pipelines",
        ))
        .respond_with(json_response(
            r#"[{
                "access_level": "manage_build_and_read",
                "created_at": "2023-08-10T05:24:08.651Z",
                "pipeline_id": "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
                "pipeline_url": "https://api.buildkite.com/v2/organizations/my-org/pipelines/pipeline-1"
            },
            {
                "access_level": "manage_build_and_read",
                "created_at": "2023-08-10T05:24:08.663Z",
                "pipeline_id": "4569ddb1-1697-4fad-a46b-372f7318432d",
                "pipeline_url": "https://api.buildkite.com/v2/organizations/my-org/pipelines/pipeline-2"
            }]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let pipelines = client
        .team_pipelines
        .list("my-org", "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038")
        .await
        .unwrap();

    assert_eq!(pipelines.len(), 2);
    let first_pipeline = pipelines.first().expect("first pipeline");
    assert_eq!(
        first_pipeline.pipeline_id,
        Some("1239d7f9-394a-4d99-badf-7c3d8577a8ff".to_string())
    );
    assert_eq!(
        first_pipeline.pipeline_url,
        Some("https://api.buildkite.com/v2/organizations/my-org/pipelines/pipeline-1".to_string())
    );
    assert_eq!(
        first_pipeline.access_level,
        Some("manage_build_and_read".to_string())
    );
    assert!(first_pipeline.created_at.is_some());
}

#[tokio::test]
async fn test_team_pipelines_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/my-org/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/pipelines/1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        ))
        .respond_with(json_response(
            r#"{
                "access_level": "manage_build_and_read",
                "created_at": "2023-08-10T05:24:08.651Z",
                "pipeline_id": "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
                "pipeline_url": "https://api.buildkite.com/v2/organizations/my-org/pipelines/pipeline-1"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let pipeline = client
        .team_pipelines
        .get(
            "my-org",
            "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038",
            "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        )
        .await
        .unwrap();

    assert_eq!(
        pipeline.pipeline_id,
        Some("1239d7f9-394a-4d99-badf-7c3d8577a8ff".to_string())
    );
    assert_eq!(
        pipeline.pipeline_url,
        Some("https://api.buildkite.com/v2/organizations/my-org/pipelines/pipeline-1".to_string())
    );
    assert_eq!(
        pipeline.access_level,
        Some("manage_build_and_read".to_string())
    );
    assert!(pipeline.created_at.is_some());
}

#[tokio::test]
async fn test_team_pipelines_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/my-org/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/pipelines",
        ))
        .respond_with(json_response(
            r#"{
                "access_level": "manage_build_and_read",
                "created_at": "2023-08-10T05:24:08.651Z",
                "pipeline_id": "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
                "pipeline_url": "https://api.buildkite.com/v2/organizations/my-org/pipelines/pipeline-1"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let pipeline = client
        .team_pipelines
        .create(
            "my-org",
            "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038",
            "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
            "manage_build_and_read",
        )
        .await
        .unwrap();

    assert_eq!(
        pipeline.pipeline_id,
        Some("1239d7f9-394a-4d99-badf-7c3d8577a8ff".to_string())
    );
    assert_eq!(
        pipeline.access_level,
        Some("manage_build_and_read".to_string())
    );
}

#[tokio::test]
async fn test_team_pipelines_update() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path(
            "/v2/organizations/my-org/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/pipelines/1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        ))
        .respond_with(json_response(
            r#"{
                "access_level": "build_and_read",
                "created_at": "2023-08-10T05:24:08.651Z",
                "pipeline_id": "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
                "pipeline_url": "https://api.buildkite.com/v2/organizations/my-org/pipelines/pipeline-1"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let pipeline = client
        .team_pipelines
        .update(
            "my-org",
            "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038",
            "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
            "build_and_read",
        )
        .await
        .unwrap();

    assert_eq!(pipeline.access_level, Some("build_and_read".to_string()));
}

#[tokio::test]
async fn test_team_pipelines_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/v2/organizations/my-org/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/pipelines/1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client
        .team_pipelines
        .delete(
            "my-org",
            "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038",
            "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        )
        .await;

    assert!(result.is_ok());
}
