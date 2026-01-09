mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use wiremock::Mock;
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_flaky_tests_list() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/analytics/organizations/my-org/suites/my-suite/flaky-tests",
        ))
        .respond_with(json_response(
            r#"[
            {
                "id": "flaky-123",
                "web_url": "https://buildkite.com/organizations/my-org/analytics/suites/my-suite/tests/flaky-123",
                "scope": "User#login",
                "name": "logs in successfully",
                "location": "spec/models/user_spec.rb:10",
                "file_name": "spec/models/user_spec.rb",
                "instances": 5,
                "most_recent_instance_at": "2023-06-01T12:00:00.000Z"
            },
            {
                "id": "flaky-456",
                "scope": "User#logout",
                "name": "logs out",
                "instances": 3
            }
        ]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let flaky_tests = client.flaky_tests.list("my-org", "my-suite").await.unwrap();

    assert_eq!(flaky_tests.len(), 2);
    let first = flaky_tests.first().unwrap();
    assert_eq!(first.id, Some("flaky-123".to_string()));
    assert_eq!(first.scope, Some("User#login".to_string()));
    assert_eq!(first.name, Some("logs in successfully".to_string()));
    assert_eq!(
        first.location,
        Some("spec/models/user_spec.rb:10".to_string())
    );
    assert_eq!(first.instances, Some(5));
    let second = flaky_tests.get(1).unwrap();
    assert_eq!(second.id, Some("flaky-456".to_string()));
    assert_eq!(second.instances, Some(3));
}
