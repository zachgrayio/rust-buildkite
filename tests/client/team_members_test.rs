use super::common::{buildkite_client, json_response, setup_mock_server};
use rust_buildkite::CreateTeamMember;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_team_members_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/teams/123/members"))
        .respond_with(json_response(r#"[{"user_id":"123"}]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let members = client
        .team_members
        .list("my-great-org", "123")
        .await
        .unwrap();

    assert_eq!(members.len(), 1);
    let first_member = members.first().expect("first member");
    assert_eq!(first_member.user_id, Some("123".to_string()));
}

#[tokio::test]
async fn test_team_members_list_paginated() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/teams/123/members"))
        .respond_with(json_response(r#"[{"user_id":"123"},{"user_id":"456"}]"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let members = client
        .team_members
        .list("my-great-org", "123")
        .await
        .unwrap();

    assert_eq!(members.len(), 2);
    assert_eq!(
        members.first().expect("first").user_id,
        Some("123".to_string())
    );
    assert_eq!(
        members.get(1).expect("second").user_id,
        Some("456".to_string())
    );
}

#[tokio::test]
async fn test_team_members_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/teams/123/members/456"))
        .respond_with(json_response(r#"{"user_id":"456"}"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let member = client
        .team_members
        .get("my-great-org", "123", "456")
        .await
        .unwrap();

    assert_eq!(member.user_id, Some("456".to_string()));
}

#[tokio::test]
async fn test_team_members_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/organizations/my-great-org/teams/123/members"))
        .respond_with(json_response(r#"{"user_id":"456"}"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let input = CreateTeamMember {
        user_id: "456".to_string(),
        role: None,
    };

    let member = client
        .team_members
        .create("my-great-org", "123", &input)
        .await
        .unwrap();

    assert_eq!(member.user_id, Some("456".to_string()));
}

#[tokio::test]
async fn test_team_members_update() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path("/v2/organizations/my-great-org/teams/123/members/456"))
        .respond_with(json_response(r#"{"user_id":"456"}"#))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let member = client
        .team_members
        .update("my-great-org", "123", "456", "maintainer")
        .await
        .unwrap();

    assert_eq!(member.user_id, Some("456".to_string()));
}

#[tokio::test]
async fn test_team_members_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/organizations/my-great-org/teams/123/members/456"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client
        .team_members
        .delete("my-great-org", "123", "456")
        .await;

    assert!(result.is_ok());
}
