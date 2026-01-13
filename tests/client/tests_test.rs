use super::common::{buildkite_client, json_response, setup_mock_server};
use wiremock::Mock;
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_tests_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/analytics/organizations/my-great-org/suites/suite-example/tests/b3abe2e9-35c5-4905-85e1-8c9f2da3240f",
        ))
        .respond_with(json_response(
            r#"{
                "id": "b3abe2e9-35c5-4905-85e1-8c9f2da3240f",
                "url": "https://api.buildkite.com/v2/analytics/organizations/my-great-org/suite-example/tests/b3abe2e9-35c5-4905-85e1-8c9f2da3240f",
                "web_url": "https://buildkite.com/organizations/my-great-org/analytics/suite-example/tests/b3abe2e9-35c5-4905-85e1-8c9f2da3240f",
                "name": "TestExample1_Create",
                "scope": "User#email",
                "location": "./resources/test_example_test.go:123",
                "file_name": "./resources/test_example_test.go"
            }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let test = client
        .tests
        .get(
            "my-great-org",
            "suite-example",
            "b3abe2e9-35c5-4905-85e1-8c9f2da3240f",
        )
        .await
        .unwrap();

    assert_eq!(
        test.id,
        Some("b3abe2e9-35c5-4905-85e1-8c9f2da3240f".to_string())
    );
    assert_eq!(
        test.url,
        Some("https://api.buildkite.com/v2/analytics/organizations/my-great-org/suite-example/tests/b3abe2e9-35c5-4905-85e1-8c9f2da3240f".to_string())
    );
    assert_eq!(
        test.web_url,
        Some("https://buildkite.com/organizations/my-great-org/analytics/suite-example/tests/b3abe2e9-35c5-4905-85e1-8c9f2da3240f".to_string())
    );
    assert_eq!(test.name, Some("TestExample1_Create".to_string()));
    assert_eq!(test.scope, Some("User#email".to_string()));
    assert_eq!(
        test.location,
        Some("./resources/test_example_test.go:123".to_string())
    );
    assert_eq!(
        test.file_name,
        Some("./resources/test_example_test.go".to_string())
    );
}
