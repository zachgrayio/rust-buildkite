use clap::Parser;
use rust_buildkite::{Client, PipelineTemplateUpdate};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BUILDKITE_TOKEN")]
    token: String,
    #[arg(long)]
    org: String,
    #[arg(long)]
    uuid: String,
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)]
    configuration: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::builder(args.token).build();
    
    let input = PipelineTemplateUpdate {
        name: args.name,
        description: args.description,
        configuration: args.configuration,
        available: None,
    };
    
    let template = client.pipeline_templates.update(&args.org, &args.uuid, input).await?;
    
    println!("{}", serde_json::to_string_pretty(&template)?);
    
    Ok(())
}
