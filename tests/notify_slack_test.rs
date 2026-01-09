mod pipeline_test_common;

use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestNotifySlack {
    #[serde(flatten)]
    notify_slack: NotifySlack,
}

#[test]
fn test_slack_string() {
    let val = TestNotifySlack {
        notify_slack: NotifySlack {
            slack: Some(NotifySlackSlack::String("#general".to_string())),
            ..::std::default::Default::default()
        },
    };
    check_result(val, "{\"slack\":\"#general\"}");
}

#[test]
fn test_slack_object() {
    let val = TestNotifySlack {
        notify_slack: NotifySlack {
            slack: Some(NotifySlackSlack::NotifySlackObject(NotifySlackObject {
                channels: vec!["one".to_string(), "two".to_string()],
                message: Some("hi".to_string()),
            })),
            ..::std::default::Default::default()
        },
    };
    check_result(
        val,
        "{\"slack\":{\"channels\":[\"one\",\"two\"],\"message\":\"hi\"}}",
    );
}

#[test]
fn test_if() {
    let val = TestNotifySlack {
        notify_slack: NotifySlack {
            if_: Some(If("string".to_string())),
            ..::std::default::Default::default()
        },
    };
    check_result(val, "{\"if\":\"string\"}");
}

#[test]
fn test_all() {
    let val = TestNotifySlack {
        notify_slack: NotifySlack {
            if_: Some(If("string".to_string())),
            slack: Some(NotifySlackSlack::String("#general".to_string())),
        },
    };
    check_result(val, "{\"if\":\"string\",\"slack\":\"#general\"}");
}
