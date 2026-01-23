use super::common::check_result;
#[allow(unused_imports)]
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestDependsOnListObjectAllowFailure {
    allow_failure: bool,
}

#[derive(Serialize)]
struct DependsOnListObject {
    #[serde(skip_serializing_if = "is_default_allow_failure")]
    allow_failure: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<String>,
}

fn is_default_allow_failure(value: &bool) -> bool {
    !value
}

#[derive(Serialize)]
#[serde(untagged)]
enum CustomDependsOnListItem {
    String(String),
    Object(DependsOnListObject),
}

#[derive(Serialize)]
struct TestDependsOnList {
    depends_on: Vec<CustomDependsOnListItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depends_on_list_object_allow_failure_bool() {
        let test_val = TestDependsOnListObjectAllowFailure {
            allow_failure: true,
        };

        check_result(test_val, r#"{"allow_failure":true}"#);
    }

    #[test]
    fn test_depends_on_list_object_step() {
        let test_val = DependsOnListObject {
            allow_failure: false,
            step: Some("step".to_string()),
        };

        check_result(test_val, r#"{"step":"step"}"#);
    }

    #[test]
    fn test_depends_on_list_object_allow_failure() {
        let test_val = DependsOnListObject {
            allow_failure: true,
            step: None,
        };

        check_result(test_val, r#"{"allow_failure":true}"#);
    }

    #[test]
    fn test_depends_on_list_object_all() {
        let test_val = DependsOnListObject {
            allow_failure: true,
            step: Some("step".to_string()),
        };

        check_result(test_val, r#"{"allow_failure":true,"step":"step"}"#);
    }

    #[test]
    fn test_depends_on_list_string() {
        let test_val = TestDependsOnList {
            depends_on: vec![
                CustomDependsOnListItem::String("one".to_string()),
                CustomDependsOnListItem::String("two".to_string()),
            ],
        };

        check_result(test_val, r#"{"depends_on":["one","two"]}"#);
    }

    #[test]
    fn test_depends_on_list_object() {
        let test_val = TestDependsOnList {
            depends_on: vec![
                CustomDependsOnListItem::Object(DependsOnListObject {
                    allow_failure: false,
                    step: Some("step1".to_string()),
                }),
                CustomDependsOnListItem::Object(DependsOnListObject {
                    allow_failure: false,
                    step: Some("step2".to_string()),
                }),
            ],
        };

        check_result(
            test_val,
            r#"{"depends_on":[{"step":"step1"},{"step":"step2"}]}"#,
        );
    }

    #[test]
    fn test_depends_on_list_mixed() {
        let test_val = TestDependsOnList {
            depends_on: vec![
                CustomDependsOnListItem::String("one".to_string()),
                CustomDependsOnListItem::Object(DependsOnListObject {
                    allow_failure: false,
                    step: Some("step2".to_string()),
                }),
            ],
        };

        check_result(test_val, r#"{"depends_on":["one",{"step":"step2"}]}"#);
    }
}
