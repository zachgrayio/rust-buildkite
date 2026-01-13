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
    token_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    client
        .cluster_tokens
        .delete(&args.org, &args.cluster_id, &args.token_id)
        .await?;

    println!("Cluster token deleted successfully");

    Ok(())
}
