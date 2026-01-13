use rust_buildkite::*;

#[test]
fn test_notify_webhook() {
    let notify = NotifyWebhook {
        if_: None,
        webhook: Some("https://example.com/webhook".to_string()),
    };

    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""webhook":"https://example.com/webhook""#));
}
