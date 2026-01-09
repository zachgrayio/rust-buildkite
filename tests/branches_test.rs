mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestBranches {
    branches: Branches,
}

#[test]
fn test_branches_string() {
    let val = "string".to_string();
    let test_val = TestBranches {
        branches: Branches::String(val),
    };
    check_result(test_val, r#"{"branches":"string"}"#);
}

#[test]
fn test_branches_string_array() {
    let val = vec!["one".to_string(), "two".to_string()];
    let test_val = TestBranches {
        branches: Branches::Array(val),
    };
    check_result(test_val, r#"{"branches":["one","two"]}"#);
}
