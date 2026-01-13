use clap::Parser;
use rust_buildkite::{Client, ClusterTokenCreateUpdate};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    cluster_id: String,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)]
    allowed_ip_addresses: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let create_token = ClusterTokenCreateUpdate {
        description: args.description,
        allowed_ip_addresses: args.allowed_ip_addresses,
    };

    let token = client
        .cluster_tokens
        .create(&args.org, &args.cluster_id, create_token)
        .await?;

    println!("{}", serde_json::to_string_pretty(&token)?);

    Ok(())
}
