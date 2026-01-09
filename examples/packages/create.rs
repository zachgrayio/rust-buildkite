use clap::Parser;
use rust_buildkite::Client;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    registry: String,
    #[arg(long)]
    file_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let package = client
        .packages
        .create_from_file(&args.org, &args.registry, &args.file_path)
        .await?;

    println!("{}", serde_json::to_string_pretty(&package)?);

    Ok(())
}
