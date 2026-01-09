mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct NotifySlackObjectWrapper {
    #[serde(flatten)]
    inner: NotifySlackObject,
}

#[test]
fn test_notify_slack_object() {
    {
        let val = NotifySlackObjectWrapper {
            inner: NotifySlackObject {
                message: Some("hi".to_string()),
                ..Default::default()
            },
        };
        check_result(val, r#"{"message":"hi"}"#);
    }

    {
        let val = NotifySlackObjectWrapper {
            inner: NotifySlackObject {
                channels: vec!["one".to_string(), "two".to_string()],
                ..Default::default()
            },
        };
        check_result(val, r#"{"channels":["one","two"]}"#);
    }

    {
        let val = NotifySlackObjectWrapper {
            inner: NotifySlackObject {
                message: Some("hi".to_string()),
                channels: vec!["one".to_string(), "two".to_string()],
            },
        };
        check_result(val, r#"{"channels":["one","two"],"message":"hi"}"#);
    }
}
