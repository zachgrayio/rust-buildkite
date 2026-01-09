use clap::Parser;

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
    let client = reqwest::Client::new();

    let update_pipeline = serde_json::json!({
        "name": "Updated Pipeline Name"
    });

    let pipeline: serde_json::Value = client
        .patch(format!(
            "https://api.buildkite.com/v2/organizations/{}/pipelines/{}",
            args.org, args.slug
        ))
        .bearer_auth(&args.token)
        .json(&update_pipeline)
        .send()
        .await?
        .json()
        .await?;

    println!("{}", serde_json::to_string_pretty(&pipeline)?);

    Ok(())
}
