mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use wiremock::Mock;
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_user_current() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/user"))
        .respond_with(json_response(
            r#"{"id":"123","name":"Jane Doe","email":"jane@doe.com"}"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let user = client.user.current_user().await.unwrap();

    assert_eq!(user.id, Some("123".to_string()));
    assert_eq!(user.name, Some("Jane Doe".to_string()));
    assert_eq!(user.email, Some("jane@doe.com".to_string()));
}
