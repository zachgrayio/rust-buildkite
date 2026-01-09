mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestInputStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_dependency_failure: Option<AllowDependencyFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_teams: Option<AllowedTeams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Branches>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<DependsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Fields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<String>,
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    if_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<InputStepType>,
}

impl TestInputStep {
    fn new() -> Self {
        Self {
            allow_dependency_failure: None,
            allowed_teams: None,
            branches: None,
            depends_on: None,
            fields: None,
            id: None,
            identifier: None,
            if_: None,
            input: None,
            key: None,
            label: None,
            name: None,
            prompt: None,
            type_: None,
        }
    }
}

#[test]
fn test_allow_dependency_failure() {
    let value = true;
    let val = TestInputStep {
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(value)),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"allow_dependency_failure":true}"#);
}

#[test]
fn test_allowed_teams() {
    let value = "allowedTeams".to_string();
    let val = TestInputStep {
        allowed_teams: Some(AllowedTeams::String(value)),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"allowed_teams":"allowedTeams"}"#);
}

#[test]
fn test_input() {
    let value = "input".to_string();
    let val = TestInputStep {
        input: Some(value),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"input":"input"}"#);
}

#[test]
fn test_branches() {
    let value = "branch".to_string();
    let val = TestInputStep {
        branches: Some(Branches::String(value)),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"branches":"branch"}"#);
}

#[test]
fn test_depends_on() {
    let value = "step".to_string();
    let val = TestInputStep {
        depends_on: Some(DependsOn::String(value)),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"depends_on":"step"}"#);
}

#[test]
fn test_fields() {
    let text = "textField".to_string();
    let text_field = TextField::builder()
        .key("key")
        .text(text)
        .try_into()
        .unwrap();
    let fields = vec![FieldsItem::TextField(text_field)];
    let val = TestInputStep {
        fields: Some(Fields(fields)),
        ..TestInputStep::new()
    };
    check_result(
        val,
        r#"{"fields":[{"key":"key","required":true,"text":"textField"}]}"#,
    );
}

#[test]
fn test_id() {
    let value = "id".to_string();
    let val = TestInputStep {
        id: Some(value),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"id":"id"}"#);
}

#[test]
fn test_identifier() {
    let value = "identifier".to_string();
    let val = TestInputStep {
        identifier: Some(value),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"identifier":"identifier"}"#);
}

#[test]
fn test_if() {
    let value = "if".to_string();
    let val = TestInputStep {
        if_: Some(value),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"if":"if"}"#);
}

#[test]
fn test_key() {
    let value = "key".to_string();
    let val = TestInputStep {
        key: Some(value),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"key":"key"}"#);
}

#[test]
fn test_label() {
    let value = "label".to_string();
    let val = TestInputStep {
        label: Some(value),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"label":"label"}"#);
}

#[test]
fn test_name() {
    let value = "name".to_string();
    let val = TestInputStep {
        name: Some(value),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"name":"name"}"#);
}

#[test]
fn test_prompt() {
    let value = "prompt".to_string();
    let val = TestInputStep {
        prompt: Some(value),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"prompt":"prompt"}"#);
}

#[test]
fn test_type() {
    let value = InputStepType::Input;
    let val = TestInputStep {
        type_: Some(value),
        ..TestInputStep::new()
    };
    check_result(val, r#"{"type":"input"}"#);
}

#[test]
fn test_all() {
    let allow_dependency_failure = true;
    let allowed_teams = "allowedTeams".to_string();
    let input = "input".to_string();
    let branches = "branch".to_string();
    let depends_on = "step".to_string();
    let text = "textField".to_string();
    let text_field = TextField::builder()
        .key("key")
        .text(text)
        .try_into()
        .unwrap();
    let fields = vec![FieldsItem::TextField(text_field)];
    let id = "id".to_string();
    let identifier = "identifier".to_string();
    let if_value = "if".to_string();
    let key = "key".to_string();
    let label = "label".to_string();
    let name = "name".to_string();
    let prompt = "prompt".to_string();
    let type_value = InputStepType::Input;

    let val = TestInputStep {
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(allow_dependency_failure)),
        allowed_teams: Some(AllowedTeams::String(allowed_teams)),
        branches: Some(Branches::String(branches)),
        depends_on: Some(DependsOn::String(depends_on)),
        fields: Some(Fields(fields)),
        id: Some(id),
        identifier: Some(identifier),
        if_: Some(if_value),
        input: Some(input),
        key: Some(key),
        label: Some(label),
        name: Some(name),
        prompt: Some(prompt),
        type_: Some(type_value),
    };
    check_result(
        val,
        r#"{"allow_dependency_failure":true,"allowed_teams":"allowedTeams","branches":"branch","depends_on":"step","fields":[{"key":"key","required":true,"text":"textField"}],"id":"id","identifier":"identifier","if":"if","input":"input","key":"key","label":"label","name":"name","prompt":"prompt","type":"input"}"#,
    );
}
