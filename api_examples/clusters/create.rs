use clap::Parser;
use rust_buildkite::{Client, ClusterCreate};

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

    let cluster_create = ClusterCreate {
        name: "Development Cluster".to_string(),
        description: Some("A cluster for development work".to_string()),
        emoji: Some(":toolbox:".to_string()),
        color: Some("#A9CCE3".to_string()),
        maintainers: None,
    };

    let cluster = client.clusters.create(&args.org, cluster_create).await?;

    println!("{}", serde_json::to_string_pretty(&cluster)?);

    Ok(())
}
