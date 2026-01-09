mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use wiremock::Mock;
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_rate_limit_get() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-org/rate_limit"))
        .respond_with(json_response(
            r#"{
            "scopes": {
                "graphql": {
                    "current": 50,
                    "enforced": true,
                    "limit": 1000,
                    "reset": 1672531200,
                    "reset_at": "2023-01-01T00:00:00.000Z"
                },
                "rest": {
                    "current": 100,
                    "enforced": true,
                    "limit": 200,
                    "reset": 1672531200,
                    "reset_at": "2023-01-01T00:00:00.000Z"
                }
            }
        }"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let rate_limit = client.rate_limit.get("my-org").await.unwrap();

    assert!(rate_limit.scopes.is_some());
    let scopes = rate_limit.scopes.unwrap();

    assert!(scopes.graphql.is_some());
    let graphql = scopes.graphql.unwrap();
    assert_eq!(graphql.current, Some(50));
    assert_eq!(graphql.enforced, Some(true));
    assert_eq!(graphql.limit, Some(1000));

    assert!(scopes.rest.is_some());
    let rest = scopes.rest.unwrap();
    assert_eq!(rest.current, Some(100));
    assert_eq!(rest.enforced, Some(true));
    assert_eq!(rest.limit, Some(200));
}
