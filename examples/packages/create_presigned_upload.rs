use clap::Parser;
use rust_buildkite::Client;
use std::path::PathBuf;
use tokio::fs::File;

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

    println!("Requesting presigned upload URL...");
    let presigned = client
        .packages
        .request_presigned_upload(&args.org, &args.registry)
        .await?;
    println!("Got presigned upload URL: {}", presigned.uri);

    let file = File::open(&args.file_path).await?;
    let filename = args
        .file_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid filename")?;

    println!("Uploading to S3...");
    let s3_url = client
        .packages
        .perform_upload(&presigned, file, filename)
        .await?;
    println!("Uploaded to: {}", s3_url);

    println!("Finalizing package...");
    let package = client
        .packages
        .finalize_upload(&args.org, &args.registry, &s3_url)
        .await?;

    println!("Package created:");
    println!("{}", serde_json::to_string_pretty(&package)?);

    Ok(())
}
