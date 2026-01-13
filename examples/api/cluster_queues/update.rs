use clap::Parser;
use rust_buildkite::{Client, ClusterQueueUpdate, RetryAgentAffinity};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    cluster_id: String,
    #[arg(long)]
    queue_id: String,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)]
    retry_agent_affinity: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let retry_affinity = args.retry_agent_affinity.map(|s| match s.as_str() {
        "prefer-different" => RetryAgentAffinity::PreferDifferent,
        _ => RetryAgentAffinity::PreferWarmest,
    });

    let update = ClusterQueueUpdate {
        description: args.description,
        retry_agent_affinity: retry_affinity,
    };

    let queue = client
        .cluster_queues
        .update(&args.org, &args.cluster_id, &args.queue_id, update)
        .await?;

    println!("{}", serde_json::to_string_pretty(&queue)?);

    Ok(())
}
