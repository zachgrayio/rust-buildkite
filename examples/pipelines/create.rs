use clap::Parser;
use rust_buildkite::{Client, CreatePipeline, GitHubSettings, ProviderSettings};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let clusters = client.clusters.list(&args.org).await?;

    if clusters.is_empty() {
        eprintln!("No clusters found, please create one before creating a pipeline");
        std::process::exit(1);
    }

    let cluster = clusters.first().unwrap();
    let cluster_id = cluster.id.as_ref().unwrap();

    println!(
        "Using cluster: {} ({})",
        cluster.name.as_ref().unwrap(),
        cluster_id
    );

    let provider_settings = ProviderSettings::GitHub(GitHubSettings {
        trigger_mode: Some("code".to_string()),
        build_pull_requests: Some(true),
        pull_request_branch_filter_enabled: Some(false),
        pull_request_branch_filter_configuration: None,
        skip_pull_request_builds_for_existing_commits: Some(true),
        build_pull_request_forks: Some(false),
        prefix_pull_request_fork_branch_names: Some(true),
        build_branches: Some(true),
        build_tags: Some(false),
        filter_enabled: Some(false),
        filter_condition: None,
        publish_commit_status: Some(true),
        publish_blocked_as_pending: Some(true),
        publish_commit_status_per_step: Some(false),
        separate_pull_request_statuses: Some(false),
        repository: None,
    });

    let create_pipeline = CreatePipeline {
        name: "my-great-pipeline".to_string(),
        repository: "git@github.com:my_great_org/my_great_repo2.git".to_string(),
        cluster_id: cluster_id.clone(),
        configuration: Some("env:\n  \"FOO\": \"bar\"\nsteps:\n  - command: \"script/release.sh\"\n    \"name\": \"Build 📦\"".to_string()),
        steps: None,
        default_branch: Some("main".to_string()),
        default_command_step_timeout: None,
        description: Some("A great pipeline for building and deploying".to_string()),
        env: None,
        maximum_command_step_timeout: None,
        provider_settings: Some(provider_settings),
        branch_configuration: None,
        skip_queued_branch_builds: Some(false),
        skip_queued_branch_builds_filter: None,
        cancel_running_branch_builds: Some(false),
        cancel_running_branch_builds_filter: None,
        team_uuids: None,
        visibility: Some("private".to_string()),
        tags: Some(vec!["great".to_string(), "pipeline".to_string()]),
    };

    let pipeline = client.pipelines.create(&args.org, create_pipeline).await?;

    println!("{}", serde_json::to_string_pretty(&pipeline)?);

    Ok(())
}
