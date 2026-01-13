use rust_buildkite::*;

#[test]
fn test_command_step_with_command() {
    let command = "echo hello".to_string();
    let step = CommandStep {
        command: Some(CommandStepCommand::String(command)),
        ..Default::default()
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""command":"echo hello""#));
}

#[test]
fn test_command_step_with_label() {
    let label = Label("Test Label".to_string());
    let step = CommandStep {
        label: Some(label),
        ..Default::default()
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""label":"Test Label""#));
}

#[test]
fn test_command_step_with_key() {
    let key: Key = "my-key".to_string().try_into().unwrap();
    let step = CommandStep {
        key: Some(key),
        ..Default::default()
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""key":"my-key""#));
}

#[test]
fn test_command_step_with_agents_list() {
    let agents = Agents::List(AgentsList(vec!["agent1".to_string()]));
    let step = CommandStep {
        agents: Some(agents),
        ..Default::default()
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""agents":["agent1"]"#));
}

#[test]
fn test_command_step_with_concurrency() {
    let step = CommandStep {
        concurrency: Some(5),
        ..Default::default()
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""concurrency":5"#));
}

#[test]
fn test_command_step_with_timeout() {
    let step = CommandStep {
        timeout_in_minutes: Some(std::num::NonZeroU64::new(30).unwrap()),
        ..Default::default()
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""timeout_in_minutes":30"#));
}
