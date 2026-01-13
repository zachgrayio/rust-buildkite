use clap::Parser;
use rust_buildkite::{Client, ClusterQueuePause};

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
    note: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let pause = ClusterQueuePause { note: args.note };

    let queue = client
        .cluster_queues
        .pause(&args.org, &args.cluster_id, &args.queue_id, pause)
        .await?;

    println!("{}", serde_json::to_string_pretty(&queue)?);

    Ok(())
}
