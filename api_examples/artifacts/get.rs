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
    #[arg(long)]
    job: String,
    #[arg(long)]
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let artifact = client
        .artifacts
        .get(&args.org, &args.pipeline, &args.build, &args.job, &args.id)
        .await?;

    println!("{}", serde_json::to_string_pretty(&artifact)?);

    Ok(())
}
