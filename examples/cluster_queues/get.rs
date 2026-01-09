use clap::Parser;
use rust_buildkite::Client;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let queue = client
        .cluster_queues
        .get(&args.org, &args.cluster_id, &args.queue_id)
        .await?;

    println!("{}", serde_json::to_string_pretty(&queue)?);

    Ok(())
}
