use clap::Parser;
use rust_buildkite::{Client, CreatePackageRegistryInput};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    name: String,
    #[arg(long)]
    ecosystem: String,
    #[arg(long)]
    description: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();
    
    let input = CreatePackageRegistryInput {
        name: Some(args.name),
        ecosystem: Some(args.ecosystem),
        description: args.description,
        emoji: None,
        color: None,
        oidc_policy: None,
    };
    
    let registry = client.package_registries.create(&args.org, input).await?;
    
    println!("{}", serde_json::to_string_pretty(&registry)?);
    
    Ok(())
}
