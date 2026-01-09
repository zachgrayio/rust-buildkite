use clap::Parser;
use rust_buildkite::{Client, ClusterQueueCreate, RetryAgentAffinity};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    cluster_id: String,
    #[arg(long)]
    key: String,
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

    let create_queue = ClusterQueueCreate {
        key: Some(args.key),
        description: args.description,
        retry_agent_affinity: retry_affinity,
    };

    let queue = client
        .cluster_queues
        .create(&args.org, &args.cluster_id, create_queue)
        .await?;

    println!("{}", serde_json::to_string_pretty(&queue)?);

    Ok(())
}
