mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;

#[test]
fn test_pagerduty() {
    let val = NotifyPagerduty {
        pagerduty_change_event: Some("event".to_string()),
        ..::std::default::Default::default()
    };
    check_result(val, r#"{"pagerduty_change_event":"event"}"#);
}

#[test]
fn test_if() {
    let val = NotifyPagerduty {
        if_: Some("string".to_string().into()),
        ..::std::default::Default::default()
    };
    check_result(val, r#"{"if":"string"}"#);
}

#[test]
fn test_all() {
    let val = NotifyPagerduty {
        pagerduty_change_event: Some("event".to_string()),
        if_: Some("if".to_string().into()),
    };
    check_result(val, r#"{"if":"if","pagerduty_change_event":"event"}"#);
}
