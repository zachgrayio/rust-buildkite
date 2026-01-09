mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestNotifyBasecamp {
    notify: NotifyBasecamp,
}

#[test]
fn test_notify_basecamp() {
    let test_val = TestNotifyBasecamp {
        notify: NotifyBasecamp {
            basecamp_campfire: Some("string".to_string()),
            if_: None,
        },
    };
    check_result(test_val, r#"{"notify":{"basecamp_campfire":"string"}}"#);
}

#[test]
fn test_notify_basecamp_if() {
    let test_val = TestNotifyBasecamp {
        notify: NotifyBasecamp {
            basecamp_campfire: None,
            if_: Some(If("string".to_string())),
        },
    };
    check_result(test_val, r#"{"notify":{"if":"string"}}"#);
}

#[test]
fn test_notify_basecamp_all() {
    let test_val = TestNotifyBasecamp {
        notify: NotifyBasecamp {
            basecamp_campfire: Some("value".to_string()),
            if_: Some(If("if".to_string())),
        },
    };
    check_result(
        test_val,
        r#"{"notify":{"basecamp_campfire":"value","if":"if"}}"#,
    );
}
