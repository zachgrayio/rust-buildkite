mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestBlockStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_dependency_failure: Option<AllowDependencyFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_teams: Option<AllowedTeams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocked_state: Option<BlockStepBlockedState>,
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
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<BlockStepType>,
}

impl TestBlockStep {
    fn new() -> Self {
        Self {
            allow_dependency_failure: None,
            allowed_teams: None,
            block: None,
            blocked_state: None,
            branches: None,
            depends_on: None,
            fields: None,
            id: None,
            identifier: None,
            if_: None,
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
    let val = TestBlockStep {
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(value)),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"allow_dependency_failure":true}"#);
}

#[test]
fn test_allowed_teams() {
    let value = "string".to_string();
    let val = TestBlockStep {
        allowed_teams: Some(AllowedTeams::String(value)),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"allowed_teams":"string"}"#);
}

#[test]
fn test_block() {
    let value = "string".to_string();
    let val = TestBlockStep {
        block: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"block":"string"}"#);
}

#[test]
fn test_blocked_state() {
    let value = BlockStepBlockedState::Passed;
    let val = TestBlockStep {
        blocked_state: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"blocked_state":"passed"}"#);
}

#[test]
fn test_branches() {
    let value = "branch".to_string();
    let val = TestBlockStep {
        branches: Some(Branches::String(value)),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"branches":"branch"}"#);
}

#[test]
fn test_depends_on() {
    let value = "value".to_string();
    let val = TestBlockStep {
        depends_on: Some(DependsOn::String(value)),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"depends_on":"value"}"#);
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
    let val = TestBlockStep {
        fields: Some(Fields(fields)),
        ..TestBlockStep::new()
    };
    check_result(
        val,
        r#"{"fields":[{"key":"key","required":true,"text":"textField"}]}"#,
    );
}

#[test]
fn test_id() {
    let value = "value".to_string();
    let val = TestBlockStep {
        id: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"id":"value"}"#);
}

#[test]
fn test_identifier() {
    let value = "value".to_string();
    let val = TestBlockStep {
        identifier: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"identifier":"value"}"#);
}

#[test]
fn test_if() {
    let value = "value".to_string();
    let val = TestBlockStep {
        if_: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"if":"value"}"#);
}

#[test]
fn test_key() {
    let value = "value".to_string();
    let val = TestBlockStep {
        key: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"key":"value"}"#);
}

#[test]
fn test_label() {
    let value = "value".to_string();
    let val = TestBlockStep {
        label: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"label":"value"}"#);
}

#[test]
fn test_name() {
    let value = "value".to_string();
    let val = TestBlockStep {
        name: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"name":"value"}"#);
}

#[test]
fn test_prompt() {
    let value = "value".to_string();
    let val = TestBlockStep {
        prompt: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"prompt":"value"}"#);
}

#[test]
fn test_type() {
    let value = BlockStepType::Block;
    let val = TestBlockStep {
        type_: Some(value),
        ..TestBlockStep::new()
    };
    check_result(val, r#"{"type":"block"}"#);
}

#[test]
fn test_all() {
    let allow_dependency_failure = true;
    let allowed_teams = "allowedTeams".to_string();
    let block = "block".to_string();
    let blocked_state = BlockStepBlockedState::Passed;
    let branches = "branch".to_string();
    let depends_on = "step".to_string();
    let fields_text = "textField".to_string();
    let text_field = TextField::builder()
        .key("key")
        .text(fields_text)
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
    let type_value = BlockStepType::Block;

    let val = TestBlockStep {
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(allow_dependency_failure)),
        allowed_teams: Some(AllowedTeams::String(allowed_teams)),
        block: Some(block),
        blocked_state: Some(blocked_state),
        branches: Some(Branches::String(branches)),
        depends_on: Some(DependsOn::String(depends_on)),
        fields: Some(Fields(fields)),
        id: Some(id),
        identifier: Some(identifier),
        if_: Some(if_value),
        key: Some(key),
        label: Some(label),
        name: Some(name),
        prompt: Some(prompt),
        type_: Some(type_value),
    };
    check_result(
        val,
        r#"{"allow_dependency_failure":true,"allowed_teams":"allowedTeams","block":"block","blocked_state":"passed","branches":"branch","depends_on":"step","fields":[{"key":"key","required":true,"text":"textField"}],"id":"id","identifier":"identifier","if":"if","key":"key","label":"label","name":"name","prompt":"prompt","type":"block"}"#,
    );
}
