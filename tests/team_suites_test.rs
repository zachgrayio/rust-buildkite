mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_team_suites_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/testorg/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/suites",
        ))
        .respond_with(json_response(
            r#"[{
                "suite_id": "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
                "suite_url": "https://api.buildkite.com/v2/analytics/organizations/testorg/suites/suite-dreams",
                "access_level": ["read"],
                "created_at": "2023-08-10T05:24:08.651Z"
            },
            {
                "suite_id": "4569",
                "suite_url": "https://api.buildkite.com/v2/analytics/organizations/testorg/suites/suite-and-sour",
                "access_level": ["read", "edit"],
                "created_at": "2023-08-10T05:24:08.663Z"
            }]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let suites = client
        .team_suites
        .list("testorg", "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038")
        .await
        .unwrap();

    assert_eq!(suites.len(), 2);
    let first_suite = suites.first().expect("first suite");
    let second_suite = suites.get(1).expect("second suite");
    assert_eq!(
        first_suite.suite_id,
        Some("1239d7f9-394a-4d99-badf-7c3d8577a8ff".to_string())
    );
    assert_eq!(
        first_suite.suite_url,
        Some(
            "https://api.buildkite.com/v2/analytics/organizations/testorg/suites/suite-dreams"
                .to_string()
        )
    );
    assert_eq!(first_suite.access_level, Some(vec!["read".to_string()]));
    assert!(first_suite.created_at.is_some());

    assert_eq!(second_suite.suite_id, Some("4569".to_string()));
    assert_eq!(
        second_suite.access_level,
        Some(vec!["read".to_string(), "edit".to_string()])
    );
}

#[tokio::test]
async fn test_team_suites_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/organizations/testorg/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/suites/1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        ))
        .respond_with(json_response(
            r#"{
                "suite_id": "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
                "suite_url": "https://api.buildkite.com/v2/analytics/organizations/testorg/suites/suite-dreams",
                "access_level": ["read"],
                "created_at": "2023-08-10T05:24:08.651Z"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let suite = client
        .team_suites
        .get(
            "testorg",
            "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038",
            "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        )
        .await
        .unwrap();

    assert_eq!(
        suite.suite_id,
        Some("1239d7f9-394a-4d99-badf-7c3d8577a8ff".to_string())
    );
    assert_eq!(
        suite.suite_url,
        Some(
            "https://api.buildkite.com/v2/analytics/organizations/testorg/suites/suite-dreams"
                .to_string()
        )
    );
    assert_eq!(suite.access_level, Some(vec!["read".to_string()]));
    assert!(suite.created_at.is_some());
}

#[tokio::test]
async fn test_team_suites_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path(
            "/v2/organizations/testorg/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/suite",
        ))
        .respond_with(json_response(
            r#"{
                "suite_id": "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
                "suite_url": "https://api.buildkite.com/v2/analytics/organizations/testorg/suites/suite-dreams",
                "access_level": ["read", "edit"],
                "created_at": "2023-08-10T05:24:08.651Z"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let suite = client
        .team_suites
        .create(
            "testorg",
            "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038",
            "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
            "read,edit",
        )
        .await
        .unwrap();

    assert_eq!(
        suite.suite_id,
        Some("1239d7f9-394a-4d99-badf-7c3d8577a8ff".to_string())
    );
    assert_eq!(
        suite.access_level,
        Some(vec!["read".to_string(), "edit".to_string()])
    );
}

#[tokio::test]
async fn test_team_suites_update() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path(
            "/v2/organizations/testorg/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/suites/1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        ))
        .respond_with(json_response(
            r#"{
                "suite_id": "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
                "suite_url": "https://api.buildkite.com/v2/analytics/organizations/testorg/suites/suite-dreams",
                "access_level": ["read", "edit"],
                "created_at": "2023-08-10T05:24:08.651Z"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let suite = client
        .team_suites
        .update(
            "testorg",
            "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038",
            "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
            "read,edit",
        )
        .await
        .unwrap();

    assert_eq!(
        suite.access_level,
        Some(vec!["read".to_string(), "edit".to_string()])
    );
}

#[tokio::test]
async fn test_team_suites_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/v2/organizations/testorg/teams/c6fa9b07-efeb-4aea-b5ad-c4aa01e91038/suites/1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        ))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client
        .team_suites
        .delete(
            "testorg",
            "c6fa9b07-efeb-4aea-b5ad-c4aa01e91038",
            "1239d7f9-394a-4d99-badf-7c3d8577a8ff",
        )
        .await;

    assert!(result.is_ok());
}
