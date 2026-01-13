use super::common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestCommandStepCommand {
    command: CommandStepCommand,
}

#[test]
fn test_command_step_command_string() {
    let val = TestCommandStepCommand {
        command: CommandStepCommand::String("string".to_string()),
    };
    check_result(val, r#"{"command":"string"}"#);
}

#[test]
fn test_command_step_command_string_array() {
    let val = TestCommandStepCommand {
        command: CommandStepCommand::Array(vec!["one".to_string(), "two".to_string()]),
    };
    check_result(val, r#"{"command":["one","two"]}"#);
}
