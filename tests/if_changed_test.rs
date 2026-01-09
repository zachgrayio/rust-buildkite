mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestIfChanged {
    if_changed: IfChanged,
}

#[test]
fn test_if_changed_string() {
    let test_val = TestIfChanged {
        if_changed: IfChanged::String("*.txt".to_string()),
    };
    check_result(test_val, r#"{"if_changed":"*.txt"}"#);
}

#[test]
fn test_if_changed_string_array() {
    let test_val = TestIfChanged {
        if_changed: IfChanged::Array(vec!["*.txt".to_string()]),
    };
    check_result(test_val, r#"{"if_changed":["*.txt"]}"#);
}

#[test]
fn test_if_changed_object_string() {
    let test_val = TestIfChanged {
        if_changed: IfChanged::Object {
            include: IfChangedObjectInclude::String("*.txt".to_string()),
            exclude: Some(IfChangedObjectExclude::String("*.md".to_string())),
        },
    };
    check_result(
        test_val,
        r#"{"if_changed":{"exclude":"*.md","include":"*.txt"}}"#,
    );
}

#[test]
fn test_if_changed_object_string_array() {
    let test_val = TestIfChanged {
        if_changed: IfChanged::Object {
            include: IfChangedObjectInclude::Array(vec!["*.txt".to_string()]),
            exclude: Some(IfChangedObjectExclude::Array(vec!["*.md".to_string()])),
        },
    };
    check_result(
        test_val,
        r#"{"if_changed":{"exclude":["*.md"],"include":["*.txt"]}}"#,
    );
}
