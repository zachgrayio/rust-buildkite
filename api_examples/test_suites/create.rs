use clap::Parser;
use rust_buildkite::{Client, TestSuiteCreate};

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

    let suite_create = TestSuiteCreate {
        name: "RSpec tests".to_string(),
        default_branch: Some("main".to_string()),
        show_api_token: None,
        team_uuids: Some(vec!["474de468-84d6-46dc-ba23-bac1add44a60".to_string()]),
    };

    let suite = client.test_suites.create(&args.org, suite_create).await?;

    println!("{}", serde_json::to_string_pretty(&suite)?);

    Ok(())
}
