use clap::Parser;
use rust_buildkite::Client;

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    pipeline: String,
    #[arg(long)]
    build: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let artifacts = client
        .artifacts
        .list_by_build(&args.org, &args.pipeline, &args.build)
        .await?;

    println!("{}", serde_json::to_string_pretty(&artifacts)?);

    Ok(())
}
