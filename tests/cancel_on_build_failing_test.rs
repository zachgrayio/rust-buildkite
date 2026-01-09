mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestCancelOnBuildFailing {
    cancel_on_build_failing: CancelOnBuildFailing,
}

#[test]
fn test_cancel_on_build_failing_string() {
    let val = CancelOnBuildFailingString::True;
    let test_val = TestCancelOnBuildFailing {
        cancel_on_build_failing: CancelOnBuildFailing::String(val),
    };
    check_result(test_val, r#"{"cancel_on_build_failing":"true"}"#);
}

#[test]
fn test_cancel_on_build_failing_boolean() {
    let val = true;
    let test_val = TestCancelOnBuildFailing {
        cancel_on_build_failing: CancelOnBuildFailing::Boolean(val),
    };
    check_result(test_val, r#"{"cancel_on_build_failing":true}"#);
}
