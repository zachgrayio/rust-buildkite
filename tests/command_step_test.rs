mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestCommandStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    agents: Option<Agents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_dependency_failure: Option<AllowDependencyFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artifact_paths: Option<CommandStepArtifactPaths>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branches: Option<Branches>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<Cache>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_on_build_failing: Option<CancelOnBuildFailing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<CommandStepCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<CommandStepCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concurrency_method: Option<CommandStepConcurrencyMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<DependsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<String>,
    #[serde(rename = "if", skip_serializing_if = "Option::is_none")]
    if_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    if_changed: Option<IfChanged>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matrix: Option<Matrix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify: Option<CommandStepNotify>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parallelism: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugins: Option<Plugins>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry: Option<CommandStepRetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Secrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<CommandStepSignature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<Skip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    soft_fail: Option<SoftFail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_in_minutes: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<CommandStepType>,
}

impl TestCommandStep {
    fn new() -> Self {
        Self {
            agents: None,
            allow_dependency_failure: None,
            artifact_paths: None,
            branches: None,
            cache: None,
            cancel_on_build_failing: None,
            command: None,
            commands: None,
            concurrency: None,
            concurrency_group: None,
            concurrency_method: None,
            depends_on: None,
            env: None,
            id: None,
            identifier: None,
            if_: None,
            if_changed: None,
            image: None,
            key: None,
            label: None,
            matrix: None,
            name: None,
            notify: None,
            parallelism: None,
            plugins: None,
            priority: None,
            retry: None,
            secrets: None,
            signature: None,
            skip: None,
            soft_fail: None,
            timeout_in_minutes: None,
            type_: None,
        }
    }
}

#[test]
fn test_agents() {
    let agents = vec!["agent".to_string()];
    let val = TestCommandStep {
        agents: Some(Agents::List(AgentsList(agents))),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"agents":["agent"]}"#);
}

#[test]
fn test_allow_dependency_failure() {
    let allow_dependency_failure = true;
    let val = TestCommandStep {
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(allow_dependency_failure)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"allow_dependency_failure":true}"#);
}

#[test]
fn test_artifact_paths_string() {
    let artifact_path = "path".to_string();
    let val = TestCommandStep {
        artifact_paths: Some(CommandStepArtifactPaths::String(artifact_path)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"artifact_paths":"path"}"#);
}

#[test]
fn test_artifact_paths_string_array() {
    let artifact_paths = vec!["one".to_string(), "two".to_string()];
    let val = TestCommandStep {
        artifact_paths: Some(CommandStepArtifactPaths::Array(artifact_paths)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"artifact_paths":["one","two"]}"#);
}

#[test]
fn test_branches() {
    let branches = "branch".to_string();
    let val = TestCommandStep {
        branches: Some(Branches::String(branches)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"branches":"branch"}"#);
}

#[test]
fn test_cache() {
    let cache = "cache".to_string();
    let val = TestCommandStep {
        cache: Some(Cache::String(cache)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"cache":"cache"}"#);
}

#[test]
fn test_cancel_on_build_failing() {
    let cancel_on_build_failing = true;
    let val = TestCommandStep {
        cancel_on_build_failing: Some(CancelOnBuildFailing::Boolean(cancel_on_build_failing)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"cancel_on_build_failing":true}"#);
}

#[test]
fn test_command() {
    let command = "command".to_string();
    let val = TestCommandStep {
        command: Some(CommandStepCommand::String(command)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"command":"command"}"#);
}

#[test]
fn test_commands() {
    let commands = vec!["one".to_string(), "two".to_string()];
    let val = TestCommandStep {
        commands: Some(CommandStepCommand::Array(commands)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"commands":["one","two"]}"#);
}

#[test]
fn test_concurrency() {
    let concurrency = 1;
    let val = TestCommandStep {
        concurrency: Some(concurrency),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"concurrency":1}"#);
}

#[test]
fn test_concurrency_group() {
    let concurrency_group = "group".to_string();
    let val = TestCommandStep {
        concurrency_group: Some(concurrency_group),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"concurrency_group":"group"}"#);
}

#[test]
fn test_concurrency_method() {
    let concurrency_method = CommandStepConcurrencyMethod::Ordered;
    let val = TestCommandStep {
        concurrency_method: Some(concurrency_method),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"concurrency_method":"ordered"}"#);
}

#[test]
fn test_depends_on() {
    let depends_on = "step".to_string();
    let val = TestCommandStep {
        depends_on: Some(DependsOn::String(depends_on)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"depends_on":"step"}"#);
}

#[test]
fn test_env() {
    let mut env = serde_json::Map::new();
    env.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );
    let val = TestCommandStep {
        env: Some(Env(env)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"env":{"foo":"bar"}}"#);
}

#[test]
fn test_if() {
    let if_value = "ifValue".to_string();
    let val = TestCommandStep {
        if_: Some(if_value),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"if":"ifValue"}"#);
}

#[test]
fn test_if_changed() {
    let if_changed = "ifChanged".to_string();
    let val = TestCommandStep {
        if_changed: Some(IfChanged::String(if_changed)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"if_changed":"ifChanged"}"#);
}

#[test]
fn test_key() {
    let key = "key".to_string();
    let val = TestCommandStep {
        key: Some(key),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"key":"key"}"#);
}

#[test]
fn test_identifier() {
    let identifier = "identifier".to_string();
    let val = TestCommandStep {
        identifier: Some(identifier),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"identifier":"identifier"}"#);
}

#[test]
fn test_id() {
    let id = "id".to_string();
    let val = TestCommandStep {
        id: Some(id),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"id":"id"}"#);
}

#[test]
fn test_image() {
    let image = "image".to_string();
    let val = TestCommandStep {
        image: Some(image),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"image":"image"}"#);
}

#[test]
fn test_label() {
    let label = "label".to_string();
    let val = TestCommandStep {
        label: Some(label),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"label":"label"}"#);
}

#[test]
fn test_signature_algorithm() {
    let algorithm = "algorithm".to_string();
    let val = TestCommandStep {
        signature: Some(CommandStepSignature {
            algorithm: Some(algorithm),
            ..::std::default::Default::default()
        }),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"signature":{"algorithm":"algorithm"}}"#);
}

#[test]
fn test_signature_value() {
    let value = "value".to_string();
    let val = TestCommandStep {
        signature: Some(CommandStepSignature {
            value: Some(value),
            ..::std::default::Default::default()
        }),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"signature":{"value":"value"}}"#);
}

#[test]
fn test_signature_signed_fields() {
    let signed_fields = vec!["one".to_string(), "two".to_string()];
    let val = TestCommandStep {
        signature: Some(CommandStepSignature {
            signed_fields,
            ..::std::default::Default::default()
        }),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"signature":{"signed_fields":["one","two"]}}"#);
}

#[test]
fn test_signature_all() {
    let algorithm = "algorithm".to_string();
    let value = "value".to_string();
    let signed_fields = vec!["one".to_string(), "two".to_string()];
    let val = TestCommandStep {
        signature: Some(CommandStepSignature {
            algorithm: Some(algorithm),
            value: Some(value),
            signed_fields,
        }),
        ..TestCommandStep::new()
    };
    check_result(
        val,
        r#"{"signature":{"algorithm":"algorithm","signed_fields":["one","two"],"value":"value"}}"#,
    );
}

#[test]
fn test_matrix() {
    let element = "value".to_string();
    let list = MatrixElementList(vec![MatrixElement::String(element)]);
    let val = TestCommandStep {
        matrix: Some(Matrix::ElementList(list)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"matrix":["value"]}"#);
}

#[test]
fn test_name() {
    let name = "name".to_string();
    let val = TestCommandStep {
        name: Some(name),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"name":"name"}"#);
}

#[test]
fn test_notify() {
    let notify_slack = "#channel".to_string();
    let val = TestCommandStep {
        notify: Some(CommandStepNotify(vec![CommandStepNotifyItem::Slack(
            NotifySlack {
                if_: None,
                slack: Some(NotifySlackSlack::String(notify_slack)),
            },
        )])),
        ..TestCommandStep::new()
    };
    check_result(val, r##"{"notify":[{"slack":"#channel"}]}"##);
}

#[test]
fn test_parallelism() {
    let parallelism = 2;
    let val = TestCommandStep {
        parallelism: Some(parallelism),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"parallelism":2}"#);
}

#[test]
fn test_plugins() {
    let mut docker_config = serde_json::Map::new();
    docker_config.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );

    let mut plugin_obj = serde_json::Map::new();
    plugin_obj.insert(
        "docker".to_string(),
        serde_json::Value::Object(docker_config),
    );

    let val = TestCommandStep {
        plugins: Some(Plugins::List(PluginsList(vec![PluginsListItem::Object(
            plugin_obj,
        )]))),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"plugins":[{"docker":{"foo":"bar"}}]}"#);
}

#[test]
fn test_secrets_string_array() {
    let val = TestCommandStep {
        secrets: Some(Secrets::Array(vec!["MY_SECRET".to_string()])),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"secrets":["MY_SECRET"]}"#);
}

#[test]
fn test_secrets_object() {
    let mut secrets_obj = ::std::collections::HashMap::new();
    secrets_obj.insert("MY_SECRET".to_string(), "API_TOKEN".to_string());
    let val = TestCommandStep {
        secrets: Some(Secrets::Object(secrets_obj)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"secrets":{"MY_SECRET":"API_TOKEN"}}"#);
}

#[test]
fn test_soft_fail() {
    let soft_fail = true;
    let val = TestCommandStep {
        soft_fail: Some(SoftFail::Variant0(SoftFailVariant0::Boolean(soft_fail))),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"soft_fail":true}"#);
}

#[test]
fn test_retry_automatic() {
    let limit = 1;
    let list = vec![AutomaticRetry {
        exit_status: None,
        limit: Some(limit),
        signal: None,
        signal_reason: None,
    }];
    let val = TestCommandStep {
        retry: Some(CommandStepRetry {
            automatic: Some(CommandStepAutomaticRetry::Variant2(AutomaticRetryList(
                list,
            ))),
            manual: None,
        }),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"retry":{"automatic":[{"limit":1}]}}"#);
}

#[test]
fn test_retry_manual() {
    let value = true;
    let val = TestCommandStep {
        retry: Some(CommandStepRetry {
            automatic: None,
            manual: Some(CommandStepManualRetry::Variant0(
                CommandStepManualRetryVariant0::Boolean(value),
            )),
        }),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"retry":{"manual":true}}"#);
}

#[test]
fn test_skip() {
    let skip = "true".parse::<SkipString>().unwrap();
    let val = TestCommandStep {
        skip: Some(Skip::String(skip)),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"skip":"true"}"#);
}

#[test]
fn test_timeout_in_minutes() {
    let timeout = 2;
    let val = TestCommandStep {
        timeout_in_minutes: Some(timeout),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"timeout_in_minutes":2}"#);
}

#[test]
fn test_type() {
    let command_type = CommandStepType::Command;
    let val = TestCommandStep {
        type_: Some(command_type),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"type":"command"}"#);
}

#[test]
fn test_priority() {
    let priority = 1;
    let val = TestCommandStep {
        priority: Some(priority),
        ..TestCommandStep::new()
    };
    check_result(val, r#"{"priority":1}"#);
}

#[test]
fn test_all() {
    let agents = vec!["agent".to_string()];
    let allow_dependency_failure = true;
    let artifact_paths = vec!["one".to_string(), "two".to_string()];
    let branches = "branch".to_string();
    let cache = "cache".to_string();
    let cancel_on_build_failing = true;
    let command = "command".to_string();
    let commands = vec!["one".to_string(), "two".to_string()];
    let concurrency = 1;
    let concurrency_group = "group".to_string();
    let concurrency_method = CommandStepConcurrencyMethod::Ordered;
    let depends_on = "step".to_string();
    let mut env = serde_json::Map::new();
    env.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );
    let if_value = "ifValue".to_string();
    let key = "key".to_string();
    let identifier = "identifier".to_string();
    let id = "id".to_string();
    let image = "image".to_string();
    let label = "label".to_string();
    let algorithm = "algorithm".to_string();
    let value = "value".to_string();
    let signed_fields = vec!["one".to_string(), "two".to_string()];
    let signature = CommandStepSignature {
        algorithm: Some(algorithm),
        value: Some(value),
        signed_fields,
    };
    let matrix_element = "value".to_string();
    let matrix_list = MatrixElementList(vec![MatrixElement::String(matrix_element)]);
    let name = "name".to_string();
    let notify_slack = "#channel".to_string();
    let parallelism = 2;
    let soft_fail = true;
    let manual_retry = true;
    let skip = "true".parse::<SkipString>().unwrap();
    let timeout = 2;
    let command_type = CommandStepType::Command;
    let priority = 1;

    let mut docker_config = serde_json::Map::new();
    docker_config.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );

    let mut plugin_obj = serde_json::Map::new();
    plugin_obj.insert(
        "docker".to_string(),
        serde_json::Value::Object(docker_config),
    );

    let val = TestCommandStep {
        agents: Some(Agents::List(AgentsList(agents))),
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(allow_dependency_failure)),
        artifact_paths: Some(CommandStepArtifactPaths::Array(artifact_paths)),
        branches: Some(Branches::String(branches)),
        cache: Some(Cache::String(cache)),
        cancel_on_build_failing: Some(CancelOnBuildFailing::Boolean(cancel_on_build_failing)),
        command: Some(CommandStepCommand::String(command)),
        commands: Some(CommandStepCommand::Array(commands)),
        concurrency: Some(concurrency),
        concurrency_group: Some(concurrency_group),
        concurrency_method: Some(concurrency_method),
        depends_on: Some(DependsOn::String(depends_on)),
        env: Some(Env(env)),
        if_: Some(if_value),
        if_changed: Some(IfChanged::String("ifChanged".to_string())),
        key: Some(key),
        identifier: Some(identifier),
        id: Some(id),
        image: Some(image),
        label: Some(label),
        signature: Some(signature),
        matrix: Some(Matrix::ElementList(matrix_list)),
        name: Some(name),
        notify: Some(CommandStepNotify(vec![CommandStepNotifyItem::Slack(
            NotifySlack {
                if_: None,
                slack: Some(NotifySlackSlack::String(notify_slack)),
            },
        )])),
        parallelism: Some(parallelism),
        plugins: Some(Plugins::List(PluginsList(vec![PluginsListItem::Object(
            plugin_obj,
        )]))),
        priority: Some(priority),
        retry: Some(CommandStepRetry {
            automatic: None,
            manual: Some(CommandStepManualRetry::Variant0(
                CommandStepManualRetryVariant0::Boolean(manual_retry),
            )),
        }),
        secrets: None,
        skip: Some(Skip::String(skip)),
        soft_fail: Some(SoftFail::Variant0(SoftFailVariant0::Boolean(soft_fail))),
        timeout_in_minutes: Some(timeout),
        type_: Some(command_type),
    };
    check_result(
        val,
        r##"{"agents":["agent"],"allow_dependency_failure":true,"artifact_paths":["one","two"],"branches":"branch","cache":"cache","cancel_on_build_failing":true,"command":"command","commands":["one","two"],"concurrency":1,"concurrency_group":"group","concurrency_method":"ordered","depends_on":"step","env":{"foo":"bar"},"id":"id","identifier":"identifier","if":"ifValue","if_changed":"ifChanged","image":"image","key":"key","label":"label","matrix":["value"],"name":"name","notify":[{"slack":"#channel"}],"parallelism":2,"plugins":[{"docker":{"foo":"bar"}}],"priority":1,"retry":{"manual":true},"signature":{"algorithm":"algorithm","signed_fields":["one","two"],"value":"value"},"skip":"true","soft_fail":true,"timeout_in_minutes":2,"type":"command"}"##,
    );
}

#[derive(Serialize)]
struct TestNestedCommandStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<TestCommandStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<TestCommandStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    script: Option<TestCommandStep>,
}

#[test]
fn test_nested_command_step_command() {
    let command = "command".to_string();
    let val = TestNestedCommandStep {
        command: Some(TestCommandStep {
            command: Some(CommandStepCommand::String(command)),
            ..TestCommandStep::new()
        }),
        commands: None,
        script: None,
    };
    check_result(val, r#"{"command":{"command":"command"}}"#);
}

#[test]
fn test_nested_command_step_commands() {
    let command = "command".to_string();
    let val = TestNestedCommandStep {
        command: None,
        commands: Some(TestCommandStep {
            command: Some(CommandStepCommand::String(command)),
            ..TestCommandStep::new()
        }),
        script: None,
    };
    check_result(val, r#"{"commands":{"command":"command"}}"#);
}

#[test]
fn test_nested_command_step_script() {
    let command = "command".to_string();
    let val = TestNestedCommandStep {
        command: None,
        commands: None,
        script: Some(TestCommandStep {
            command: Some(CommandStepCommand::String(command)),
            ..TestCommandStep::new()
        }),
    };
    check_result(val, r#"{"script":{"command":"command"}}"#);
}
