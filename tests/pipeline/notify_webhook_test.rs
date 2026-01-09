
use rust_buildkite::*;

#[test]
fn test_notify_webhook() {
    let notify = NotifyWebhook {
        url: Some("https://example.com/webhook".to_string()),
        webhook: None,
    };
    
    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""url":"https://example.com/webhook""#));
}
