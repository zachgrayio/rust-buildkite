//! Parity test between the pipeline! macro and the builder API
//!
//! This test constructs pipelines using both approaches and verifies
//! they produce identical YAML output.

use rust_buildkite::*;

/// Build a pipeline using the builder API with key fields demonstrated
fn build_pipeline_with_builder() -> JsonSchemaForBuildkitePipelineConfigurationFiles {
    let mut env_map = serde_json::Map::new();
    env_map.insert(
        "CI".to_string(),
        serde_json::Value::String("true".to_string()),
    );
    env_map.insert(
        "NODE_ENV".to_string(),
        serde_json::Value::String("test".to_string()),
    );
    let mut cmd_env = serde_json::Map::new();
    cmd_env.insert(
        "DEBUG".to_string(),
        serde_json::Value::String("1".to_string()),
    );

    let mut cmd_agents = serde_json::Map::new();
    cmd_agents.insert(
        "queue".to_string(),
        serde_json::Value::String("default".to_string()),
    );

    let command_step = PipelineStepsItem::CommandStep(
        CommandStep::builder()
            .command(Some(CommandStepCommand::String("npm test".to_string())))
            .label(Some(Label("Run Tests".to_string())))
            .key(Some("test".to_string().try_into().expect("key")))
            .env(Some(Env(cmd_env)))
            .agents(Some(Agents::Object(AgentsObject(cmd_agents))))
            .branches(Some(Branches::Array(vec![
                "main".to_string(),
                "develop".to_string(),
            ])))
            .cache(Some(Cache::Array(vec!["node_modules".to_string()])))
            .if_(Some(If("build.branch == 'main'".to_string())))
            .timeout_in_minutes(Some(std::num::NonZeroU64::new(30).unwrap()))
            .soft_fail(Some(SoftFail::Variant0(SoftFailVariant0::Boolean(true))))
            .parallelism(Some(4))
            .artifact_paths(Some(CommandStepArtifactPaths::Array(vec![
                "coverage/**/*".to_string(),
            ])))
            .concurrency(Some(2))
            .concurrency_group(Some("test/main".to_string()))
            .priority(Some(Priority(5)))
            .allow_dependency_failure(Some(AllowDependencyFailure::Boolean(true)))
            .retry(Some(
                CommandStepRetry::builder()
                    .automatic(Some(
                        serde_json::from_value(serde_json::json!({ "limit": 3 }))
                            .expect("automatic retry"),
                    ))
                    .manual(Some(
                        serde_json::from_value(serde_json::json!({ "allowed": true }))
                            .expect("manual retry"),
                    ))
                    .try_into()
                    .expect("retry"),
            ))
            .plugins(Some(Plugins::List(PluginsList(vec![
                serde_json::from_value(
                    serde_json::json!({ "docker#v5.0.0": { "image": "node:18" } }),
                )
                .expect("plugin"),
            ]))))
            .notify(Some(CommandStepNotify(vec![
                serde_json::from_value(serde_json::json!({ "slack": "#builds" })).expect("notify"),
            ])))
            .try_into()
            .expect("command step"),
    );
    let wait_step = PipelineStepsItem::WaitStep(
        WaitStep::builder()
            .continue_on_failure(WaitStepContinueOnFailure::Boolean(true))
            .if_(If("build.branch == 'main'".to_string()))
            .try_into()
            .expect("wait step"),
    );
    let block_step = PipelineStepsItem::BlockStep(
        BlockStep::builder()
            .block(Some("Deploy to Production?".to_string()))
            .key(Some("approval".to_string().try_into().expect("key")))
            .depends_on(Some(DependsOn::DependsOnList(DependsOnList(vec![
                DependsOnListItem::String("test".to_string()),
            ]))))
            .allowed_teams(Some(AllowedTeams::Array(vec!["platform-team".to_string()])))
            .blocked_state(BlockStepBlockedState::Running)
            .branches(Some(Branches::Array(vec!["main".to_string()])))
            .if_(Some(If("build.branch == 'main'".to_string())))
            .prompt(Some(Prompt("Are you sure?".to_string())))
            .allow_dependency_failure(Some(AllowDependencyFailure::Boolean(true)))
            .try_into()
            .expect("block step"),
    );
    let input_step = PipelineStepsItem::InputStep(
        InputStep::builder()
            .input(Some("Enter configuration".to_string()))
            .key(Some("config".to_string().try_into().expect("key")))
            .depends_on(Some(DependsOn::DependsOnList(DependsOnList(vec![
                DependsOnListItem::String("approval".to_string()),
            ]))))
            .allowed_teams(Some(AllowedTeams::Array(vec!["admins".to_string()])))
            .blocked_state(InputStepBlockedState::Running)
            .branches(Some(Branches::Array(vec!["main".to_string()])))
            .if_(Some(If("build.branch == 'main'".to_string())))
            .prompt(Some(Prompt("Fill in form".to_string())))
            .allow_dependency_failure(Some(AllowDependencyFailure::Boolean(true)))
            .try_into()
            .expect("input step"),
    );
    let mut trigger_env = serde_json::Map::new();
    trigger_env.insert(
        "TARGET".to_string(),
        serde_json::Value::String("production".to_string()),
    );

    let mut trigger_meta = serde_json::Map::new();
    trigger_meta.insert(
        "deploy_id".to_string(),
        serde_json::Value::String("123".to_string()),
    );

    let trigger_step = PipelineStepsItem::TriggerStep(
        TriggerStep::builder()
            .trigger("deploy-pipeline".to_string())
            .label(Some(Label("Deploy".to_string())))
            .key(Some("deploy".to_string().try_into().expect("key")))
            .depends_on(Some(DependsOn::DependsOnList(DependsOnList(vec![
                DependsOnListItem::String("config".to_string()),
            ]))))
            .async_(TriggerStepAsync::Boolean(true))
            .build(Some(
                TriggerStepBuild::builder()
                    .branch("main".to_string())
                    .commit("HEAD".to_string())
                    .message("Auto deploy".to_string())
                    .env(Some(Env(trigger_env)))
                    .meta_data(trigger_meta)
                    .try_into()
                    .expect("build config"),
            ))
            .branches(Some(Branches::Array(vec!["main".to_string()])))
            .if_(Some(If("build.branch == 'main'".to_string())))
            .skip(Some(Skip::Boolean(false)))
            .soft_fail(Some(SoftFail::Variant0(SoftFailVariant0::Boolean(true))))
            .allow_dependency_failure(Some(AllowDependencyFailure::Boolean(true)))
            .try_into()
            .expect("trigger step"),
    );
    let group_nested_step = GroupStepsItem::CommandStep(
        CommandStep::builder()
            .command(Some(CommandStepCommand::String(
                "npm run integration".to_string(),
            )))
            .label(Some(Label("Integration Tests".to_string())))
            .key(Some("integration".to_string().try_into().expect("key")))
            .try_into()
            .expect("nested command"),
    );

    let group_step = PipelineStepsItem::GroupStep(
        GroupStep::builder()
            .group(Some("Test Suite".to_string()))
            .key(Some("suite".to_string().try_into().expect("key")))
            .depends_on(Some(DependsOn::DependsOnList(DependsOnList(vec![
                DependsOnListItem::String("deploy".to_string()),
            ]))))
            .steps(GroupSteps(vec![group_nested_step]))
            .if_(Some(If("build.branch == 'main'".to_string())))
            .skip(Some(Skip::Boolean(false)))
            .notify(Some(BuildNotify(vec![
                serde_json::from_value(serde_json::json!({ "slack": "#results" })).expect("notify"),
            ])))
            .allow_dependency_failure(Some(AllowDependencyFailure::Boolean(true)))
            .try_into()
            .expect("group step"),
    );

    JsonSchemaForBuildkitePipelineConfigurationFiles::builder()
        .env(Some(Env(env_map)))
        .steps(PipelineSteps(vec![
            command_step,
            wait_step,
            block_step,
            input_step,
            trigger_step,
            group_step,
        ]))
        .try_into()
        .expect("pipeline construction failed")
}

/// Build the same pipeline using the macro
fn build_pipeline_with_macro() -> JsonSchemaForBuildkitePipelineConfigurationFiles {
    pipeline! {
        env: {
            CI: "true",
            NODE_ENV: "test"
        },
        steps: [
            command {
                command: "npm test",
                label: "Run Tests",
                key: "test",
                env: { DEBUG: "1" },
                agents: { queue: "default" },
                branches: ["main", "develop"],
                cache: ["node_modules"],
                condition: "build.branch == 'main'",
                timeout_in_minutes: 30,
                soft_fail: true,
                parallelism: 4,
                artifact_paths: ["coverage/**/*"],
                concurrency: 2,
                concurrency_group: "test/main",
                priority: 5,
                allow_dependency_failure: true,
                retry: {
                    automatic: { limit: 3 },
                    manual: { allowed: true }
                },
                plugins: [
                    { "docker#v5.0.0": { image: "node:18" } }
                ],
                notify: [
                    { slack: "#builds" }
                ]
            },
            wait {
                continue_on_failure: true,
                r#if: "build.branch == 'main'"
            },
            block {
                block: "Deploy to Production?",
                key: "approval",
                depends_on: ["test"],
                allowed_teams: ["platform-team"],
                blocked_state: "running",
                branches: ["main"],
                r#if: "build.branch == 'main'",
                prompt: "Are you sure?",
                allow_dependency_failure: true
            },
            input {
                input: "Enter configuration",
                key: "config",
                depends_on: ["approval"],
                allowed_teams: ["admins"],
                blocked_state: "running",
                branches: ["main"],
                r#if: "build.branch == 'main'",
                prompt: "Fill in form",
                allow_dependency_failure: true
            },
            trigger {
                trigger: "deploy-pipeline",
                label: "Deploy",
                key: "deploy",
                depends_on: ["config"],
                r#async: true,
                build: {
                    branch: "main",
                    commit: "HEAD",
                    message: "Auto deploy",
                    env: { TARGET: "production" },
                    meta_data: { deploy_id: "123" }
                },
                branches: ["main"],
                r#if: "build.branch == 'main'",
                skip: false,
                soft_fail: true,
                allow_dependency_failure: true
            },
            group {
                group: "Test Suite",
                key: "suite",
                depends_on: ["deploy"],
                steps: [
                    command {
                        command: "npm run integration",
                        label: "Integration Tests",
                        key: "integration"
                    }
                ],
                r#if: "build.branch == 'main'",
                skip: false,
                notify: [
                    { slack: "#results" }
                ],
                allow_dependency_failure: true
            }
        ]
    }
}

#[test]
fn test_builder_and_macro_parity() {
    let builder_pipeline = build_pipeline_with_builder();
    let macro_pipeline = build_pipeline_with_macro();

    let builder_yaml = serde_yaml::to_string(&builder_pipeline).unwrap();
    let macro_yaml = serde_yaml::to_string(&macro_pipeline).unwrap();

    if builder_yaml != macro_yaml {
        eprintln!("=== BUILDER YAML ===\n{}", builder_yaml);
        eprintln!("=== MACRO YAML ===\n{}", macro_yaml);
        let builder_lines: Vec<&str> = builder_yaml.lines().collect();
        let macro_lines: Vec<&str> = macro_yaml.lines().collect();
        for (i, (b, m)) in builder_lines.iter().zip(macro_lines.iter()).enumerate() {
            if b != m {
                eprintln!("\nFirst difference at line {}:", i + 1);
                eprintln!("  Builder: {:?}", b);
                eprintln!("  Macro:   {:?}", m);
                break;
            }
        }
        if builder_lines.len() != macro_lines.len() {
            eprintln!(
                "\nLine count difference: builder={}, macro={}",
                builder_lines.len(),
                macro_lines.len()
            );
        }
    }

    assert_eq!(
        builder_yaml, macro_yaml,
        "Builder and macro must produce identical YAML"
    );
}
