use clap::Parser;
use rust_buildkite::Client;

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    slug: String,
    #[arg(long)]
    run_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let test_run = client
        .test_runs
        .get(&args.org, &args.slug, &args.run_id)
        .await?;

    println!("{}", serde_json::to_string_pretty(&test_run)?);

    Ok(())
}
