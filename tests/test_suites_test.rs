mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use rust_buildkite::{TestSuiteCreate, TestSuiteUpdate};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_test_suites_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/analytics/organizations/my-great-org/suites"))
        .respond_with(json_response(
            r#"[
                {
                    "id": "7c202aaa-3165-4811-9813-173c4c285463",
                    "graphql_id": "N2MyMDJhYWEtMzE2NS00ODExLTk4MTMtMTczYzRjMjg1NDYz=",
                    "slug": "suite-1",
                    "name": "suite-1",
                    "url": "https://api.buildkite.com/v2/analytics/organizations/my-great-org/suites/suite-1",
                    "web_url": "https://buildkite.com/organizations/my-great-org/analytics/suites/suite-1",
                    "default_branch": "main"
                },
                {
                    "id": "38ed1d73-cea9-4aba-b223-def25e66ef51",
                    "graphql_id": "MzhlZDFkNzMtY2VhOS00YWJhLWIyMjMtZGVmMjVlNjZlZjUx=",
                    "slug": "suite-2",
                    "name": "suite-2",
                    "url": "https://api.buildkite.com/v2/analytics/organizations/my-great-org/suites/suite-2",
                    "web_url": "https://buildkite.com/organizations/my-great-org/analytics/suites/suite-2",
                    "default_branch": "main"
                }
            ]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let suites = client.test_suites.list("my-great-org").await.unwrap();

    assert_eq!(suites.len(), 2);
    let first_suite = suites.first().expect("first suite");
    assert_eq!(
        first_suite.id,
        Some("7c202aaa-3165-4811-9813-173c4c285463".to_string())
    );
    assert_eq!(
        first_suite.graphql_id,
        Some("N2MyMDJhYWEtMzE2NS00ODExLTk4MTMtMTczYzRjMjg1NDYz=".to_string())
    );
    assert_eq!(first_suite.slug, Some("suite-1".to_string()));
    assert_eq!(first_suite.name, Some("suite-1".to_string()));
    assert_eq!(
        first_suite.url,
        Some(
            "https://api.buildkite.com/v2/analytics/organizations/my-great-org/suites/suite-1"
                .to_string()
        )
    );
    assert_eq!(
        first_suite.web_url,
        Some(
            "https://buildkite.com/organizations/my-great-org/analytics/suites/suite-1".to_string()
        )
    );
    assert_eq!(first_suite.default_branch, Some("main".to_string()));
}

#[tokio::test]
async fn test_test_suites_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/analytics/organizations/my-great-org/suites/suite-1",
        ))
        .respond_with(json_response(
            r#"{
                "id": "7c202aaa-3165-4811-9813-173c4c285463",
                "graphql_id": "N2MyMDJhYWEtMzE2NS00ODExLTk4MTMtMTczYzRjMjg1NDYz=",
                "slug": "suite-1",
                "name": "suite-1",
                "url": "https://api.buildkite.com/v2/analytics/organizations/my-great-org/suites/suite-1",
                "web_url": "https://buildkite.com/organizations/my-great-org/analytics/suites/suite-1",
                "default_branch": "main"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let suite = client
        .test_suites
        .get("my-great-org", "suite-1")
        .await
        .unwrap();

    assert_eq!(
        suite.id,
        Some("7c202aaa-3165-4811-9813-173c4c285463".to_string())
    );
    assert_eq!(
        suite.graphql_id,
        Some("N2MyMDJhYWEtMzE2NS00ODExLTk4MTMtMTczYzRjMjg1NDYz=".to_string())
    );
    assert_eq!(suite.slug, Some("suite-1".to_string()));
    assert_eq!(suite.name, Some("suite-1".to_string()));
    assert_eq!(
        suite.url,
        Some(
            "https://api.buildkite.com/v2/analytics/organizations/my-great-org/suites/suite-1"
                .to_string()
        )
    );
    assert_eq!(
        suite.web_url,
        Some(
            "https://buildkite.com/organizations/my-great-org/analytics/suites/suite-1".to_string()
        )
    );
    assert_eq!(suite.default_branch, Some("main".to_string()));
}

#[tokio::test]
async fn test_test_suites_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/analytics/organizations/my-great-org/suites"))
        .respond_with(json_response(
            r#"{
                "name": "Suite 3",
                "default_branch": "main",
                "team_ids": ["8369b300-fff0-4ef1-91de-010f72f4458d"]
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let input = TestSuiteCreate {
        name: "Suite 3".to_string(),
        default_branch: Some("main".to_string()),
        show_api_token: None,
        team_uuids: Some(vec!["8369b300-fff0-4ef1-91de-010f72f4458d".to_string()]),
    };

    let suite = client
        .test_suites
        .create("my-great-org", input)
        .await
        .unwrap();

    assert_eq!(suite.name, Some("Suite 3".to_string()));
    assert_eq!(suite.default_branch, Some("main".to_string()));
}

#[tokio::test]
async fn test_test_suites_update() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/analytics/organizations/my-great-org/suites"))
        .respond_with(json_response(
            r#"{
                "name": "Suite 4",
                "default_branch": "main",
                "team_ids": ["818b0849-9718-4898-8de3-42d591a7fe26"],
                "slug": "suite-4"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    Mock::given(method("PATCH"))
        .and(path(
            "/v2/analytics/organizations/my-great-org/suites/suite-4",
        ))
        .respond_with(json_response(
            r#"{
                "name": "Suite 4",
                "default_branch": "develop",
                "team_ids": ["818b0849-9718-4898-8de3-42d591a7fe26"],
                "slug": "suite-4"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let input = TestSuiteCreate {
        name: "Suite 4".to_string(),
        default_branch: Some("main".to_string()),
        show_api_token: None,
        team_uuids: Some(vec!["818b0849-9718-4898-8de3-42d591a7fe26".to_string()]),
    };

    let created_suite = client
        .test_suites
        .create("my-great-org", input)
        .await
        .unwrap();

    let update_input = TestSuiteUpdate {
        name: Some("Suite 4".to_string()),
        default_branch: Some("develop".to_string()),
    };

    let updated_suite = client
        .test_suites
        .update(
            "my-great-org",
            &created_suite.slug.unwrap_or_else(|| "suite-4".to_string()),
            update_input,
        )
        .await
        .unwrap();

    assert_eq!(updated_suite.name, Some("Suite 4".to_string()));
    assert_eq!(updated_suite.slug, Some("suite-4".to_string()));
    assert_eq!(updated_suite.default_branch, Some("develop".to_string()));
}

#[tokio::test]
async fn test_test_suites_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/v2/analytics/organizations/my-great-org/suites/suite-5",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client.test_suites.delete("my-great-org", "suite-5").await;

    assert!(result.is_ok());
}
