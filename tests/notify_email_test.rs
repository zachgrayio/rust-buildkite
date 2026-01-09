mod pipeline_test_common;
use pipeline_test_common::check_result;
use serde::Serialize;

#[derive(Serialize)]
struct TestNotifyEmail {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    if_: Option<String>,
}

#[test]
fn test_email() {
    let val = TestNotifyEmail {
        email: Some("string".to_string()),
        if_: None,
    };
    check_result(val, r#"{"email":"string"}"#);
}

#[test]
fn test_if() {
    let val = TestNotifyEmail {
        email: None,
        if_: Some("string".to_string()),
    };
    check_result(val, r#"{"if":"string"}"#);
}

#[test]
fn test_all() {
    let val = TestNotifyEmail {
        email: Some("email".to_string()),
        if_: Some("if".to_string()),
    };
    check_result(val, r#"{"email":"email","if":"if"}"#);
}
