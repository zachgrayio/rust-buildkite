use super::common::{buildkite_client, json_response, setup_mock_server};
use rust_buildkite::CreateTeam;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_teams_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/teams"))
        .respond_with(json_response(
            r#"[
            {
                "id": "team-123",
                "name": "Engineering",
                "slug": "engineering",
                "description": "The engineering team",
                "privacy": "visible",
                "default": false
            },
            {
                "id": "team-124",
                "name": "Everyone",
                "slug": "everyone",
                "default": true
            }
        ]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let teams = client.teams.list("my-great-org").await.unwrap();

    assert_eq!(teams.len(), 2);
    let first = teams.first().unwrap();
    assert_eq!(first.id, Some("team-123".to_string()));
    assert_eq!(first.name, Some("Engineering".to_string()));
    assert_eq!(first.slug, Some("engineering".to_string()));
    assert_eq!(first.description, Some("The engineering team".to_string()));
    assert_eq!(first.privacy, Some("visible".to_string()));
    assert_eq!(first.default, Some(false));
    assert_eq!(teams.get(1).unwrap().default, Some(true));
}

#[tokio::test]
async fn test_teams_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/teams/team-123"))
        .respond_with(json_response(
            r#"{
            "id": "team-123",
            "name": "Engineering",
            "slug": "engineering",
            "description": "The engineering team",
            "privacy": "visible",
            "default": false,
            "created_at": "2023-01-01T12:00:00.000Z",
            "created_by": {
                "id": "user-123",
                "name": "Admin User",
                "email": "admin@example.com"
            }
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let team = client
        .teams
        .get_team("my-great-org", "team-123")
        .await
        .unwrap();

    assert_eq!(team.id, Some("team-123".to_string()));
    assert_eq!(team.name, Some("Engineering".to_string()));
    assert!(team.created_by.is_some());
    let creator = team.created_by.unwrap();
    assert_eq!(creator.name, Some("Admin User".to_string()));
}

#[tokio::test]
async fn test_teams_create() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/v2/organizations/my-great-org/teams"))
        .respond_with(json_response(
            r#"{
            "id": "team-new",
            "name": "New Team",
            "slug": "new-team",
            "description": "A new team",
            "privacy": "visible",
            "default": false
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let create_team = CreateTeam {
        name: Some("New Team".to_string()),
        description: Some("A new team".to_string()),
        privacy: Some("visible".to_string()),
        is_default_team: Some(false),
        default_member_role: None,
        members_can_create_pipelines: None,
    };

    let team = client
        .teams
        .create_team("my-great-org", create_team)
        .await
        .unwrap();

    assert_eq!(team.id, Some("team-new".to_string()));
    assert_eq!(team.name, Some("New Team".to_string()));
    assert_eq!(team.slug, Some("new-team".to_string()));
}

#[tokio::test]
async fn test_teams_update() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("PATCH"))
        .and(path("/v2/organizations/my-great-org/teams/team-123"))
        .respond_with(json_response(
            r#"{
            "id": "team-123",
            "name": "Updated Team",
            "description": "Updated description"
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);

    let update_team = CreateTeam {
        name: Some("Updated Team".to_string()),
        description: Some("Updated description".to_string()),
        ..Default::default()
    };

    let team = client
        .teams
        .update_team("my-great-org", "team-123", update_team)
        .await
        .unwrap();

    assert_eq!(team.name, Some("Updated Team".to_string()));
    assert_eq!(team.description, Some("Updated description".to_string()));
}

#[tokio::test]
async fn test_teams_delete() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/organizations/my-great-org/teams/team-123"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let result = client.teams.delete_team("my-great-org", "team-123").await;

    assert!(result.is_ok());
}
