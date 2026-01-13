use clap::Parser;
use rust_buildkite::{Client, PipelineTemplateCreate};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    name: String,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)]
    configuration: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();
    
    let input = PipelineTemplateCreate {
        name: Some(args.name),
        description: args.description,
        configuration: Some(args.configuration),
        available: Some(true),
    };
    
    let template = client.pipeline_templates.create(&args.org, input).await?;
    
    println!("{}", serde_json::to_string_pretty(&template)?);
    
    Ok(())
}
