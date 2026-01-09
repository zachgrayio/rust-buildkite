use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    cluster_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = reqwest::Client::new();

    let update = serde_json::json!({
        "name": "Updated Cluster"
    });

    let cluster: serde_json::Value = client
        .patch(format!(
            "https://api.buildkite.com/v2/organizations/{}/clusters/{}",
            args.org, args.cluster_id
        ))
        .bearer_auth(&args.token)
        .json(&update)
        .send()
        .await?
        .json()
        .await?;

    println!("{}", serde_json::to_string_pretty(&cluster)?);

    Ok(())
}
