use super::common::setup_mock_server;
use rust_buildkite::{Annotation, AnnotationCreate, Client, Timestamp};
use time::OffsetDateTime;
use wiremock::Mock;
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_annotations_list_by_build() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/pipelines/sup-keith/builds/awesome-build/annotations"))
        .respond_with(super::common::json_response(r#"[{
            "id": "de0d4ab5-6360-467a-a34b-e5ef5db5320d",
            "context": "default",
            "style": "info",
            "body_html": "<h1>My Markdown Heading</h1>\n<img src=\"artifact://indy.png\" alt=\"Belongs in a museum\" height=250 />",
            "created_at": "2019-04-09T18:07:15.775Z",
            "updated_at": "2019-08-06T20:58:49.396Z"
        },
        {
            "id": "5b3ceff6-78cb-4fe9-88ae-51be5f145977",
            "context": "coverage",
            "style": "info",
            "body_html": "Read the <a href=\"artifact://coverage/index.html\">uploaded coverage report</a>",
            "created_at": "2019-04-09T18:07:16.320Z",
            "updated_at": "2019-04-09T18:07:16.320Z"
        }]"#))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let annotations = client
        .annotations
        .list_by_build("my-great-org", "sup-keith", "awesome-build")
        .await
        .unwrap();

    assert_eq!(annotations.len(), 2);

    let annotations_created_at_1 = Timestamp(
        OffsetDateTime::parse(
            "2019-04-09T18:07:15.775Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap(),
    );
    let annotations_updated_at_1 = Timestamp(
        OffsetDateTime::parse(
            "2019-08-06T20:58:49.396Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap(),
    );

    let want_first = Annotation {
        id: Some("de0d4ab5-6360-467a-a34b-e5ef5db5320d".to_string()),
        context: Some("default".to_string()),
        style: Some("info".to_string()),
        body_html: Some("<h1>My Markdown Heading</h1>\n<img src=\"artifact://indy.png\" alt=\"Belongs in a museum\" height=250 />".to_string()),
        created_at: Some(annotations_created_at_1),
        updated_at: Some(annotations_updated_at_1),
    };

    let annotations_created_at_2 = Timestamp(
        OffsetDateTime::parse(
            "2019-04-09T18:07:16.320Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap(),
    );
    let annotations_updated_at_2 = Timestamp(
        OffsetDateTime::parse(
            "2019-04-09T18:07:16.320Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap(),
    );

    let want_second = Annotation {
        id: Some("5b3ceff6-78cb-4fe9-88ae-51be5f145977".to_string()),
        context: Some("coverage".to_string()),
        style: Some("info".to_string()),
        body_html: Some(
            "Read the <a href=\"artifact://coverage/index.html\">uploaded coverage report</a>"
                .to_string(),
        ),
        created_at: Some(annotations_created_at_2),
        updated_at: Some(annotations_updated_at_2),
    };

    assert_eq!(annotations.first().unwrap(), &want_first);
    assert_eq!(annotations.get(1).unwrap(), &want_second);
}

#[tokio::test]
async fn test_annotations_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/my-great-org/pipelines/my-great-pipeline/builds/10/annotations",
        ))
        .respond_with(super::common::json_response(
            r#"{
            "id": "68aef727-f754-48e1-aad8-5f5da8a9960c",
            "context": "default",
            "style": "info",
            "body_html": "<h1>My Markdown Heading</h1>\n<p>An example annotation!</p>",
            "created_at": "2023-08-21T08:50:05.824Z",
            "updated_at": "2023-08-21T08:50:05.824Z"
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = Client::builder("test-token")
        .http_client(super::common::mock_client())
        .base_url(format!("{}/", mock_server.uri()))
        .build();

    let create_annotation = AnnotationCreate {
        style: Some("info".to_string()),
        context: Some("default".to_string()),
        body: Some("<h1>My Markdown Heading</h1>\n<p>An example annotation!</p>".to_string()),
        append: Some(false),
    };

    let annotation = client
        .annotations
        .create("my-great-org", "my-great-pipeline", "10", create_annotation)
        .await
        .unwrap();

    let annotation_created_at = Timestamp(
        OffsetDateTime::parse(
            "2023-08-21T08:50:05.824Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap(),
    );
    let annotation_updated_at = Timestamp(
        OffsetDateTime::parse(
            "2023-08-21T08:50:05.824Z",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap(),
    );

    let want = Annotation {
        id: Some("68aef727-f754-48e1-aad8-5f5da8a9960c".to_string()),
        context: Some("default".to_string()),
        style: Some("info".to_string()),
        body_html: Some("<h1>My Markdown Heading</h1>\n<p>An example annotation!</p>".to_string()),
        created_at: Some(annotation_created_at),
        updated_at: Some(annotation_updated_at),
    };

    assert_eq!(annotation, want);
}
