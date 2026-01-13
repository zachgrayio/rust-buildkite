use rust_buildkite::*;

#[test]
fn test_notify_basecamp() {
    let notify = NotifyBasecamp {
        basecamp_campfire: Some("https://example.com".to_string()),
        if_: None,
    };

    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""basecamp_campfire":"https://example.com""#));
}
