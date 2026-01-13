use rust_buildkite::*;

#[test]
fn test_notify_pagerduty() {
    let notify = NotifyPagerduty {
        if_: None,
        pagerduty_change_event: Some("integration-key".to_string()),
    };

    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""pagerduty_change_event":"integration-key""#));
}
