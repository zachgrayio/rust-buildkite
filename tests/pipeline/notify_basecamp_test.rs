
use rust_buildkite::*;

#[test]
fn test_notify_basecamp() {
    let notify = NotifyBasecamp {
        message: Some("Build completed".to_string()),
        basecamp_campfire: Some("https://example.com".to_string()),
    };
    
    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""message":"Build completed""#));
}
