use rust_buildkite::*;

#[test]
fn test_notify_slack_string() {
    let notify = NotifySlack {
        if_: None,
        slack: Some(NotifySlackSlack::String("#general".to_string())),
    };

    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r##""#general""##));
}
