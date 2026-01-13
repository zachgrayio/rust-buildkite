use clap::Parser;
use rust_buildkite::Client;

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    uuid: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();

    let template = client.pipeline_templates.get(&args.org, &args.uuid).await?;

    println!("{}", serde_json::to_string_pretty(&template)?);

    Ok(())
}
