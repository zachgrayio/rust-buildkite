use clap::Parser;
use rust_buildkite::{AnnotationCreate, Client};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    slug: String,
    #[arg(long)]
    number: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = Client::builder(args.token).build();

    let annotation_create = AnnotationCreate {
        style: Some("info".to_string()),
        context: Some("default".to_string()),
        body: Some("An example annotation!".to_string()),
        append: Some(false),
    };

    let annotation = client
        .annotations
        .create(&args.org, &args.slug, &args.number, annotation_create)
        .await?;

    println!("{}", serde_json::to_string_pretty(&annotation)?);

    Ok(())
}
