mod pipeline_test_common;

use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct PipelineWrapper {
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Secrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agents: Option<Agents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify: Option<BuildNotify>,
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<PipelineSteps>,
}

#[test]
fn test_pipeline_secrets_string_array() {
    let pipeline = PipelineWrapper {
        secrets: Some(Secrets::Array(vec!["MY_SECRET".to_string()])),
        agents: None,
        env: None,
        notify: None,
        steps: None,
    };

    check_result(pipeline, r#"{"secrets":["MY_SECRET"]}"#);
}

#[test]
fn test_pipeline_secrets_object() {
    let mut secrets_map = ::std::collections::HashMap::new();
    secrets_map.insert("MY_SECRET".to_string(), "API_TOKEN".to_string());

    let pipeline = PipelineWrapper {
        secrets: Some(Secrets::Object(secrets_map)),
        agents: None,
        env: None,
        notify: None,
        steps: None,
    };

    check_result(pipeline, r#"{"secrets":{"MY_SECRET":"API_TOKEN"}}"#);
}

#[test]
fn test_pipeline_add_agent() {
    let mut agents_map = ::serde_json::Map::new();
    agents_map.insert(
        "foo".to_string(),
        ::serde_json::Value::String("bar".to_string()),
    );

    let pipeline = PipelineWrapper {
        secrets: None,
        agents: Some(Agents::Object(AgentsObject(agents_map))),
        env: None,
        notify: None,
        steps: None,
    };

    check_result(pipeline, r#"{"agents":{"foo":"bar"}}"#);
}

#[test]
fn test_pipeline_add_environment_variable() {
    let mut env_map = ::serde_json::Map::new();
    env_map.insert(
        "FOO".to_string(),
        ::serde_json::Value::String("bar".to_string()),
    );

    let pipeline = PipelineWrapper {
        secrets: None,
        agents: None,
        env: Some(Env(env_map)),
        notify: None,
        steps: None,
    };

    check_result(pipeline, r#"{"env":{"FOO":"bar"}}"#);
}

#[test]
fn test_pipeline_notify() {
    let notify_item = BuildNotifyItem::Email(NotifyEmail {
        email: Some("person@example.com".to_string()),
        if_: None,
    });

    let pipeline = PipelineWrapper {
        secrets: None,
        agents: None,
        env: None,
        notify: Some(BuildNotify(vec![notify_item])),
        steps: None,
    };

    check_result(pipeline, r#"{"notify":[{"email":"person@example.com"}]}"#);
}

#[test]
fn test_pipeline_steps() {
    let step = PipelineStepsItem {
        command_step: Some(CommandStep {
            command: Some(CommandStepCommand::String("build.sh".to_string())),
            ..::std::default::Default::default()
        }),
        ..::std::default::Default::default()
    };

    let pipeline = PipelineWrapper {
        secrets: None,
        agents: None,
        env: None,
        notify: None,
        steps: Some(PipelineSteps(vec![step])),
    };

    check_result(pipeline, r#"{"steps":[{"command":"build.sh"}]}"#);
}
