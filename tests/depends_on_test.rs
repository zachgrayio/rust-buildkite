mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestDependsOn {
    depends_on: DependsOn,
}

#[test]
fn test_depends_on_string() {
    let val = "string".to_string();
    let test_val = TestDependsOn {
        depends_on: DependsOn::String(val),
    };
    check_result(test_val, r#"{"depends_on":"string"}"#);
}

#[test]
fn test_depends_on_list_simple() {
    let one = "one".to_string();
    let two = "two".to_string();
    let val = DependsOnList(vec![
        DependsOnListItem::String(one),
        DependsOnListItem::String(two),
    ]);
    let test_val = TestDependsOn {
        depends_on: DependsOn::DependsOnList(val),
    };
    check_result(test_val, r#"{"depends_on":["one","two"]}"#);
}

#[test]
fn test_depends_on_list_mixed() {
    let one = "one".to_string();
    let two = "step2".to_string();
    let val = DependsOnList(vec![
        DependsOnListItem::String(one),
        DependsOnListItem::Object {
            allow_failure: DependsOnListItemObjectAllowFailure::default(),
            step: Some(two),
        },
    ]);
    let test_val = TestDependsOn {
        depends_on: DependsOn::DependsOnList(val),
    };
    check_result(
        test_val,
        r#"{"depends_on":["one",{"allow_failure":false,"step":"step2"}]}"#,
    );
}
