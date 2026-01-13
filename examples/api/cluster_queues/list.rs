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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let queues = client
        .cluster_queues
        .list(&args.org, &args.cluster_id)
        .await?;

    println!("{}", serde_json::to_string_pretty(&queues)?);

    Ok(())
}
