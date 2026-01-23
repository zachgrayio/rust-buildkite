use rust_buildkite::*;
use serde_json::json;

#[test]
fn test_agents_list() {
    let agents = Agents::List(AgentsList(vec!["queue=default".to_string()]));
    let json = serde_json::to_value(&agents).unwrap();
    assert_eq!(json, json!(["queue=default"]));
}

#[test]
fn test_allow_dependency_failure_boolean() {
    let allow = AllowDependencyFailure(true);
    let json = serde_json::to_value(&allow).unwrap();
    assert_eq!(json, json!(true));
}

#[test]
fn test_branches_string() {
    let branches = Branches::String("main".to_string());
    let json = serde_json::to_value(&branches).unwrap();
    assert_eq!(json, json!("main"));
}

#[test]
fn test_cache_string() {
    let cache = Cache::String("node_modules".to_string());
    let json = serde_json::to_value(&cache).unwrap();
    assert_eq!(json, json!("node_modules"));
}

#[test]
fn test_cancel_on_build_failing_boolean() {
    let cancel = CancelOnBuildFailing(true);
    let json = serde_json::to_value(&cancel).unwrap();
    assert_eq!(json, json!(true));
}

#[test]
fn test_depends_on_string() {
    let depends = DependsOn::String("step1".to_string());
    let json = serde_json::to_value(&depends).unwrap();
    assert_eq!(json, json!("step1"));
}

#[test]
fn test_skip_boolean() {
    let skip = Skip::Boolean(true);
    let json = serde_json::to_value(&skip).unwrap();
    assert_eq!(json, json!(true));
}

#[test]
fn test_command_step_builder() {
    let step: CommandStep = CommandStep::builder()
        .command(CommandStepCommand::String("echo hello".to_string()))
        .try_into()
        .unwrap();

    let json = serde_json::to_value(&step).unwrap();
    assert!(json.get("command").is_some());
}

#[test]
fn test_trigger_step_builder() {
    let step: TriggerStep = TriggerStep::builder()
        .trigger("deploy-pipeline".to_string())
        .try_into()
        .unwrap();

    let json = serde_json::to_value(&step).unwrap();
    assert_eq!(json.get("trigger").unwrap(), &json!("deploy-pipeline"));
}
