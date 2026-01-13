//! Example: Building a Buildkite pipeline dynamically using the generated types directly
//!
//! This demonstrates how to use the code generated from the Buildkite pipeline schema
//! to construct pipelines programmatically and serialize them to JSON or YAML.

use rust_buildkite::{
    BlockStep, CommandStep, CommandStepCommand, DependsOn, DependsOnList, DependsOnListItem, Env,
    JsonSchemaForBuildkitePipelineConfigurationFiles, Label, PipelineSteps, PipelineStepsItem,
    WaitStep,
};

/// Type alias for cleaner code - matches the Go SDK's `buildkite.Pipeline`
type Pipeline = JsonSchemaForBuildkitePipelineConfigurationFiles;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let echo_step: CommandStep = CommandStep::builder()
        .command(Some(CommandStepCommand::String(
            "echo 'Hello, World!'".to_string(),
        )))
        .label(Some(Label("👋 Say Hello".to_string())))
        .try_into()?;

    let test_step: CommandStep = CommandStep::builder()
        .key(Some("tests".to_string().try_into()?))
        .commands(Some(CommandStepCommand::Array(vec![
            "npm install".to_string(),
            "npm test".to_string(),
        ])))
        .label(Some(Label(":npm: Run Tests".to_string())))
        .try_into()?;

    let wait_step = WaitStep::default();

    let deploy_approval: BlockStep = BlockStep::builder()
        .block(Some("Deploy to Production?".to_string()))
        .try_into()?;

    let deploy_step: CommandStep = CommandStep::builder()
        .command(Some(CommandStepCommand::String(
            "./deploy.sh production".to_string(),
        )))
        .label(Some(Label("🚀 Deploy".to_string())))
        .depends_on(Some(DependsOn::DependsOnList(DependsOnList(vec![
            DependsOnListItem::String("tests".to_string()),
        ]))))
        .try_into()?;

    let steps = PipelineSteps(vec![
        PipelineStepsItem::CommandStep(echo_step),
        PipelineStepsItem::CommandStep(test_step),
        PipelineStepsItem::WaitStep(wait_step),
        PipelineStepsItem::BlockStep(deploy_approval),
        PipelineStepsItem::CommandStep(deploy_step),
    ]);

    let mut env_map = serde_json::Map::new();
    env_map.insert(
        "CI".to_string(),
        serde_json::Value::String("true".to_string()),
    );

    let pipeline: Pipeline = Pipeline::builder()
        .steps(steps)
        .env(Some(Env(env_map)))
        .try_into()?;

    // JSON output
    // println!("{}", serde_json::to_string_pretty(&pipeline)?);

    // YAML output
    println!("{}", serde_yaml::to_string(&pipeline)?);

    Ok(())
}
