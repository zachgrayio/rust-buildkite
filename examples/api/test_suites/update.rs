use clap::Parser;
use rust_buildkite::{Client, TestSuiteUpdate};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    slug: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let suite_update = TestSuiteUpdate {
        name: Some("Updated RSpec tests".to_string()),
        default_branch: Some("main".to_string()),
    };

    let suite = client
        .test_suites
        .update(&args.org, &args.slug, suite_update)
        .await?;

    println!("{}", serde_json::to_string_pretty(&suite)?);

    Ok(())
}
