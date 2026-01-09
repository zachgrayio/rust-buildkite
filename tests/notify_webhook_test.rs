mod pipeline_test_common;
use pipeline_test_common::check_result;
use serde::Serialize;

#[derive(Serialize)]
struct TestNotifyWebhook {
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    if_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<String>,
}

#[test]
fn test_webhook() {
    let val = TestNotifyWebhook {
        if_: None,
        webhook: Some("string".to_string()),
    };
    check_result(val, r#"{"webhook":"string"}"#);
}

#[test]
fn test_if() {
    let val = TestNotifyWebhook {
        if_: Some("string".to_string()),
        webhook: None,
    };
    check_result(val, r#"{"if":"string"}"#);
}

#[test]
fn test_all() {
    let val = TestNotifyWebhook {
        if_: Some("if".to_string()),
        webhook: Some("string".to_string()),
    };
    check_result(val, r#"{"if":"if","webhook":"string"}"#);
}
