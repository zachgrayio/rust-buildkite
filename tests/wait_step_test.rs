mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestWaitStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_dependency_failure: Option<AllowDependencyFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Branches>,
    #[serde(skip_serializing_if = "Option::is_none")]
    continue_on_failure: Option<WaitStepContinueOnFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<DependsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<String>,
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    if_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<WaitStepType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<String>,
}

impl TestWaitStep {
    fn new() -> Self {
        Self {
            allow_dependency_failure: None,
            branches: None,
            continue_on_failure: None,
            depends_on: None,
            id: None,
            identifier: None,
            if_: None,
            key: None,
            label: None,
            name: None,
            type_: None,
            wait: None,
        }
    }
}

#[test]
fn test_allow_dependency_failure() {
    let allow_dependency_failure = true;
    let val = TestWaitStep {
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(allow_dependency_failure)),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"allow_dependency_failure":true}"#);
}

#[test]
fn test_branches() {
    let branches = vec!["one".to_string(), "two".to_string()];
    let val = TestWaitStep {
        branches: Some(Branches::Array(branches)),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"branches":["one","two"]}"#);
}

#[test]
fn test_continue_on_failure_string() {
    let val = TestWaitStep {
        continue_on_failure: Some(WaitStepContinueOnFailure::String(
            WaitStepContinueOnFailureString::True,
        )),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"continue_on_failure":"true"}"#);
}

#[test]
fn test_continue_on_failure_bool() {
    let continue_on_failure = true;
    let val = TestWaitStep {
        continue_on_failure: Some(WaitStepContinueOnFailure::Boolean(continue_on_failure)),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"continue_on_failure":true}"#);
}

#[test]
fn test_depends_on() {
    let depends_on = "step".to_string();
    let val = TestWaitStep {
        depends_on: Some(DependsOn::String(depends_on)),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"depends_on":"step"}"#);
}

#[test]
fn test_if() {
    let if_value = "if".to_string();
    let val = TestWaitStep {
        if_: Some(if_value),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"if":"if"}"#);
}

#[test]
fn test_key() {
    let key = "key".to_string();
    let val = TestWaitStep {
        key: Some(key),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"key":"key"}"#);
}

#[test]
fn test_label() {
    let label = "label".to_string();
    let val = TestWaitStep {
        label: Some(label),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"label":"label"}"#);
}

#[test]
fn test_name() {
    let name = "name".to_string();
    let val = TestWaitStep {
        name: Some(name),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"name":"name"}"#);
}

#[test]
fn test_identifier() {
    let identifier = "identifier".to_string();
    let val = TestWaitStep {
        identifier: Some(identifier),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"identifier":"identifier"}"#);
}

#[test]
fn test_id() {
    let id = "id".to_string();
    let val = TestWaitStep {
        id: Some(id),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"id":"id"}"#);
}

#[test]
fn test_type() {
    let val = TestWaitStep {
        type_: Some(WaitStepType::Wait),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"type":"wait"}"#);
}

#[test]
fn test_wait() {
    let wait = "wait".to_string();
    let val = TestWaitStep {
        wait: Some(wait),
        ..TestWaitStep::new()
    };
    check_result(val, r#"{"wait":"wait"}"#);
}

#[test]
fn test_all() {
    let allow_dependency_failure = true;
    let branches = vec!["one".to_string(), "two".to_string()];
    let continue_on_failure = true;
    let depends_on = "step".to_string();
    let if_value = "if".to_string();
    let key = "key".to_string();
    let label = "label".to_string();
    let name = "name".to_string();
    let identifier = "identifier".to_string();
    let id = "id".to_string();
    let wait = "wait".to_string();

    let val = TestWaitStep {
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(allow_dependency_failure)),
        branches: Some(Branches::Array(branches)),
        continue_on_failure: Some(WaitStepContinueOnFailure::Boolean(continue_on_failure)),
        depends_on: Some(DependsOn::String(depends_on)),
        id: Some(id),
        identifier: Some(identifier),
        if_: Some(if_value),
        key: Some(key),
        label: Some(label),
        name: Some(name),
        type_: Some(WaitStepType::Wait),
        wait: Some(wait),
    };
    check_result(
        val,
        r#"{"allow_dependency_failure":true,"branches":["one","two"],"continue_on_failure":true,"depends_on":"step","id":"id","identifier":"identifier","if":"if","key":"key","label":"label","name":"name","type":"wait","wait":"wait"}"#,
    );
}

#[derive(Serialize)]
struct TestNestedWaitStepWait {
    wait: NestedWaitStepWait,
}

#[derive(Serialize)]
struct NestedWaitStepWait {
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<String>,
}

#[test]
fn test_nested_wait_step_wait() {
    let wait = "wait".to_string();
    let val = TestNestedWaitStepWait {
        wait: NestedWaitStepWait { wait: Some(wait) },
    };
    check_result(val, r#"{"wait":{"wait":"wait"}}"#);
}

#[derive(Serialize)]
struct TestNestedWaitStepWaiter {
    waiter: NestedWaitStepWaiter,
}

#[derive(Serialize)]
struct NestedWaitStepWaiter {
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<String>,
}

#[test]
fn test_nested_wait_step_waiter() {
    let wait = "wait".to_string();
    let val = TestNestedWaitStepWaiter {
        waiter: NestedWaitStepWaiter { wait: Some(wait) },
    };
    check_result(val, r#"{"waiter":{"wait":"wait"}}"#);
}
