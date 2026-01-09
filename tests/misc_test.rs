mod common;

use common::{buildkite_client, json_response, setup_mock_server};
use wiremock::Mock;
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_list_emojis() {
    let mock_server = setup_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/v2/organizations/my-great-org/emojis"))
        .respond_with(json_response(
            r#"[{
                "name":"rocket",
                "url":"https://a.buildboxassets.com/assets/emoji2/unicode/1f680.png?v2"
            }]"#,
        ))
        .mount(&mock_server)
        .await;

    let client = buildkite_client(&mock_server);
    let emojis = client
        .organizations
        .list_emojis("my-great-org")
        .await
        .unwrap();

    assert_eq!(emojis.len(), 1);
    let first_emoji = emojis.first().expect("first emoji");
    assert_eq!(first_emoji.name, Some("rocket".to_string()));
    assert_eq!(
        first_emoji.url,
        Some("https://a.buildboxassets.com/assets/emoji2/unicode/1f680.png?v2".to_string())
    );
}
