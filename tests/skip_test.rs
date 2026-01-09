mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestSkip {
    skip: Skip,
}

#[test]
fn test_skip_bool() {
    let value = true;
    let val = TestSkip {
        skip: Skip::Boolean(value),
    };
    check_result(val, r#"{"skip":true}"#);
}

#[test]
fn test_skip_string() {
    let value = "string".parse::<SkipString>().unwrap();
    let val = TestSkip {
        skip: Skip::String(value),
    };
    check_result(val, r#"{"skip":"string"}"#);
}
