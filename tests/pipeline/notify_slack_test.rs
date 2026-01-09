
use rust_buildkite::*;

#[test]
fn test_notify_slack_string() {
    let notify = NotifySlack::String("#general".to_string());
    
    let json = serde_json::to_string(&notify).unwrap();
    assert_eq!(json, r#""#general""#);
}
