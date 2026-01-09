mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestAllowDependencyFailure {
    #[serde(rename = "allowDependencyFailure")]
    allow_dependency_failure: AllowDependencyFailure,
}

#[test]
fn test_allow_dependency_failure_string() {
    let val = AllowDependencyFailureString::True;
    let test_val = TestAllowDependencyFailure {
        allow_dependency_failure: AllowDependencyFailure::String(val),
    };
    check_result(test_val, r#"{"allowDependencyFailure":"true"}"#);
}

#[test]
fn test_allow_dependency_failure_boolean() {
    let val = true;
    let test_val = TestAllowDependencyFailure {
        allow_dependency_failure: AllowDependencyFailure::Boolean(val),
    };
    check_result(test_val, r#"{"allowDependencyFailure":true}"#);
}
