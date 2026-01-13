use super::common::{buildkite_client, json_response, setup_mock_server};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_pipeline_templates_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipeline-templates"))
        .respond_with(json_response(r#"[
            {
                "uuid": "90333dc7-b86a-4485-98c3-9419a5dbc52e",
                "graphql_id": "UGlwZWxpbmVUZW1wbG5lLS0tOTAzMzNkYzctYjg2YS00NDg1LTk4YzMtOTQxOWE1ZGJjNTJl==",
                "name": "Pipeline Upload Template",
                "description": "Pipeline template with basic YAML pipeline upload",
                "configuration": "steps:\n  - label: \":pipeline:\"\n    command: \"buildkite-agent pipeline upload\"\n",
                "available": true,
                "created_at": "2023-08-11T01:22:05.650Z",
                "created_by": {
                    "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                    "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                    "name": "Joe Smith",
                    "email": "jsmith@example.com",
                    "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                    "created_at": "2023-02-20T03:00:05.824Z"
                },
                "updated_at": "2023-08-11T01:22:05.650Z",
                "updated_by": {
                    "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                    "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                    "name": "Joe Smith",
                    "email": "jsmith@example.com",
                    "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                    "created_at": "2023-02-20T03:00:05.824Z"
                },
                "url": "https://api.buildkite.com/v2/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e",
                "web_url": "https://buildkite.com/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e"
            },
            {
                "uuid": "6a25cc85-9fa2-4a00-b66c-bfe377bc5f78",
                "graphql_id": "UGlwZWxpbmVUZW1wbG5lLS0tNmEyNWNjODUtOWZhMi00YTAwLWI2NmMtYmZlMzc3YmM1Zjc4==",
                "name": "Pipeline-Dev Upload Template",
                "description": "Pipeline template uploading buildkite-dev.yml",
                "configuration": "steps:\n  - label: \":pipeline:\"\n    command: \"buildkite-agent pipeline upload .buildkite/pipeline-dev.yml\"\n",
                "available": true,
                "created_at": "2023-08-11T02:24:33.602Z",
                "created_by": {
                    "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                    "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                    "name": "Joe Smith",
                    "email": "jsmith@example.com",
                    "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                    "created_at": "2023-02-20T03:00:05.824Z"
                },
                "updated_at": "2023-08-11T02:24:33.602Z",
                "updated_by": {
                    "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                    "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                    "name": "Joe Smith",
                    "email": "jsmith@example.com",
                    "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                    "created_at": "2023-02-20T03:00:05.824Z"
                },
                "url": "https://api.buildkite.com/v2/organizations/my-great-org/pipeline-templates/6a25cc85-9fa2-4a00-b66c-bfe377bc5f78",
                "web_url": "https://buildkite.com/organizations/my-great-org/pipeline-templates/6a25cc85-9fa2-4a00-b66c-bfe377bc5f78"
            }
        ]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let templates = client
        .pipeline_templates
        .list("my-great-org")
        .await
        .unwrap();

    assert_eq!(templates.len(), 2);
    let first = templates.first().unwrap();
    assert_eq!(
        first.uuid,
        Some("90333dc7-b86a-4485-98c3-9419a5dbc52e".to_string())
    );
    assert_eq!(first.name, Some("Pipeline Upload Template".to_string()));
    assert_eq!(
        first.graphql_id,
        Some(
            "UGlwZWxpbmVUZW1wbG5lLS0tOTAzMzNkYzctYjg2YS00NDg1LTk4YzMtOTQxOWE1ZGJjNTJl=="
                .to_string()
        )
    );
    assert_eq!(first.available, Some(true));
    assert!(first.created_by.is_some());

    let created_by = first.created_by.as_ref().unwrap();
    assert_eq!(
        created_by.id,
        Some("7da07e25-0383-4aff-a7cf-14d1a9aa098f".to_string())
    );
    assert_eq!(created_by.name, Some("Joe Smith".to_string()));
    assert_eq!(created_by.email, Some("jsmith@example.com".to_string()));

    let second = templates.get(1).unwrap();
    assert_eq!(
        second.uuid,
        Some("6a25cc85-9fa2-4a00-b66c-bfe377bc5f78".to_string())
    );
    assert_eq!(
        second.name,
        Some("Pipeline-Dev Upload Template".to_string())
    );
}

#[tokio::test]
async fn test_pipeline_templates_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e"))
        .respond_with(json_response(r#"{
            "uuid": "90333dc7-b86a-4485-98c3-9419a5dbc52e",
            "graphql_id": "UGlwZWxpbmVUZW1wbG5lLS0tOTAzMzNkYzctYjg2YS00NDg1LTk4YzMtOTQxOWE1ZGJjNTJl==",
            "name": "Pipeline Upload Template",
            "description": "Pipeline template with basic YAML pipeline upload",
            "configuration": "steps:\n  - label: \":pipeline:\"\n    command: \"buildkite-agent pipeline upload\"\n",
            "available": true,
            "created_at": "2023-08-11T01:22:05.650Z",
            "created_by": {
                "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                "name": "Joe Smith",
                "email": "jsmith@example.com",
                "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                "created_at": "2023-02-20T03:00:05.824Z"
            },
            "updated_at": "2023-08-11T01:22:05.650Z",
            "updated_by": {
                "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                "name": "Joe Smith",
                "email": "jsmith@example.com",
                "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                "created_at": "2023-02-20T03:00:05.824Z"
            },
            "url": "https://api.buildkite.com/v2/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e",
            "web_url": "https://buildkite.com/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e"
        }"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let template = client
        .pipeline_templates
        .get("my-great-org", "90333dc7-b86a-4485-98c3-9419a5dbc52e")
        .await
        .unwrap();

    assert_eq!(
        template.uuid,
        Some("90333dc7-b86a-4485-98c3-9419a5dbc52e".to_string())
    );
    assert_eq!(template.name, Some("Pipeline Upload Template".to_string()));
    assert_eq!(
        template.description,
        Some("Pipeline template with basic YAML pipeline upload".to_string())
    );
    assert!(
        template
            .configuration
            .as_ref()
            .unwrap()
            .contains("buildkite-agent pipeline upload")
    );
    assert_eq!(template.url, Some("https://api.buildkite.com/v2/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e".to_string()));
    assert_eq!(template.web_url, Some("https://buildkite.com/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e".to_string()));
}

#[tokio::test]
async fn test_pipeline_templates_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/organizations/my-great-org/pipeline-templates"))
        .respond_with(json_response(r#"{
            "uuid": "90333dc7-b86a-4485-98c3-9419a5dbc52e",
            "graphql_id": "UGlwZWxpbmVUZW1wbG5lLS0tOTAzMzNkYzctYjg2YS00NDg1LTk4YzMtOTQxOWE1ZGJjNTJl==",
            "name": "Production Pipeline Uploader",
            "description": "Production pipeline upload template",
            "configuration": "steps:\n  - label: \":pipeline:\"\n    command: \"buildkite-agent pipeline upload .buildkite/pipeline-production.yml\"\n",
            "available": true,
            "created_at": "2023-08-11T01:22:05.650Z",
            "created_by": {
                "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                "name": "Joe Smith",
                "email": "jsmith@example.com",
                "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                "created_at": "2023-02-20T03:00:05.824Z"
            },
            "updated_at": "2023-08-11T01:22:05.650Z",
            "updated_by": {
                "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                "name": "Joe Smith",
                "email": "jsmith@example.com",
                "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                "created_at": "2023-02-20T03:00:05.824Z"
            },
            "url": "https://api.buildkite.com/v2/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e",
            "web_url": "https://buildkite.com/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e"
        }"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let input = rust_buildkite::PipelineTemplateCreate {
        name: Some("Production Pipeline Uploader".to_string()),
        description: Some("Production pipeline upload template".to_string()),
        configuration: Some("steps:\n  - label: \":pipeline:\"\n    command: \"buildkite-agent pipeline upload .buildkite/pipeline-production.yml\"\n".to_string()),
        available: Some(true),
    };

    let template = client
        .pipeline_templates
        .create("my-great-org", input)
        .await
        .unwrap();

    assert_eq!(
        template.uuid,
        Some("90333dc7-b86a-4485-98c3-9419a5dbc52e".to_string())
    );
    assert_eq!(
        template.name,
        Some("Production Pipeline Uploader".to_string())
    );
}

#[tokio::test]
async fn test_pipeline_templates_update() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path("/v2/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e"))
        .respond_with(json_response(r#"{
            "uuid": "90333dc7-b86a-4485-98c3-9419a5dbc52e",
            "graphql_id": "UGlwZWxpbmVUZW1wbG5lLS0tOTAzMzNkYzctYjg2YS00NDg1LTk4YzMtOTQxOWE1ZGJjNTJl==",
            "name": "Pipeline Upload Template",
            "description": "Updated Pipeline template description",
            "configuration": "steps:\n  - label: \":pipeline:\"\n    command: \"buildkite-agent pipeline upload\"\n",
            "available": true,
            "created_at": "2023-08-11T01:22:05.650Z",
            "created_by": {
                "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                "name": "Joe Smith",
                "email": "jsmith@example.com",
                "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                "created_at": "2023-02-20T03:00:05.824Z"
            },
            "updated_at": "2023-08-11T05:42:12.000Z",
            "updated_by": {
                "id": "7da07e25-0383-4aff-a7cf-14d1a9aa098f",
                "graphql_id": "VXNlci0tLTdkYTA3ZTI1LTAzODMtNGFmZi1hN2NmLTE0ZDFhOWFhMDk4Zg==",
                "name": "Joe Smith",
                "email": "jsmith@example.com",
                "avatar_url": "https://www.gravatar.com/avatar/593nf93m405mf744n3kg9456jjph9grt4",
                "created_at": "2023-02-20T03:00:05.824Z"
            },
            "url": "https://api.buildkite.com/v2/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e",
            "web_url": "https://buildkite.com/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e"
        }"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let input = rust_buildkite::PipelineTemplateUpdate {
        description: Some("Updated Pipeline template description".to_string()),
        ..Default::default()
    };

    let template = client
        .pipeline_templates
        .update(
            "my-great-org",
            "90333dc7-b86a-4485-98c3-9419a5dbc52e",
            input,
        )
        .await
        .unwrap();

    assert_eq!(
        template.description,
        Some("Updated Pipeline template description".to_string())
    );
}

#[tokio::test]
async fn test_pipeline_templates_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/organizations/my-great-org/pipeline-templates/90333dc7-b86a-4485-98c3-9419a5dbc52e"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client
        .pipeline_templates
        .delete("my-great-org", "90333dc7-b86a-4485-98c3-9419a5dbc52e")
        .await;

    assert!(result.is_ok());
}
