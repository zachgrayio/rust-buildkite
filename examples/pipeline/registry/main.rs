use rust_buildkite::{
    CommandStep, CommandStepCommand, JsonSchemaForBuildkitePipelineConfigurationFiles as Pipeline,
    Label, PipelineSteps, PipelineStepsItem, register, registered_pipelines,
};

fn build_simple_pipeline(name: &str) -> Pipeline {
    let step: CommandStep = CommandStep::builder()
        .command(Some(CommandStepCommand::String(format!("echo '{name}'"))))
        .label(Some(Label(name.to_string())))
        .try_into()
        .expect("valid step");

    Pipeline::builder()
        .steps(PipelineSteps(vec![PipelineStepsItem::CommandStep(step)]))
        .try_into()
        .expect("valid pipeline")
}

#[register]
fn premerge() {
    let pipeline = build_simple_pipeline("Premerge");
    println!("{}", serde_yaml::to_string(&pipeline).unwrap());
}

#[register(branch = Exact("main"))]
fn postmerge() {
    let pipeline = build_simple_pipeline("Postmerge");
    println!("{}", serde_yaml::to_string(&pipeline).unwrap());
}

#[register(cron = "0 7 * * 1-5")]
fn nightly() {
    let pipeline = build_simple_pipeline("Nightly");
    println!("{}", serde_yaml::to_string(&pipeline).unwrap());
}

#[register(branch = Prefix("release/"))]
fn release() {
    let pipeline = build_simple_pipeline("Release");
    println!("{}", serde_yaml::to_string(&pipeline).unwrap());
}

fn main() {
    let branch = std::env::var("BUILDKITE_BRANCH").unwrap_or_default();
    let pipeline_env = std::env::var("PIPELINE").ok();

    println!("Available pipelines:");
    for p in registered_pipelines() {
        println!("  - {} (id: {})", p.name, p.id);
        if let Some(cron) = p.cron {
            println!("    schedule: {cron}");
        }
        if let Some(ref branch_pattern) = p.branch {
            println!("    branch: {branch_pattern:?}");
        }
    }
    println!();

    if let Some(id) = pipeline_env {
        if let Some(p) = registered_pipelines().find(|p| p.id == id) {
            println!("Running pipeline: {}", p.name);
            (p.generate)();
            return;
        }
        eprintln!("Unknown pipeline: {id}");
        std::process::exit(1);
    }

    for p in registered_pipelines() {
        if let Some(ref pattern) = p.branch
            && pattern.matches(&branch)
        {
            println!("Branch '{}' matched pipeline: {}", branch, p.name);
            (p.generate)();
        }
    }

    println!("No matching pipeline for branch: {branch}");
}
