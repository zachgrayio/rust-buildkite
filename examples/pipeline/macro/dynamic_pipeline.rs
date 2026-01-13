//! Example: Building a Buildkite pipeline using the pipeline! DSL macro
//!
//! This demonstrates how to use the pipeline! macro for declarative, type-safe
//! pipeline definitions with compile-time validation.

use rust_buildkite::pipeline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = pipeline! {
        env: {
            CI: "true"
        },
        steps: [
            command("echo 'Hello, World!'")
                .label("👋 Say Hello"),
            command("npm install && npm test")
                .label(":npm: Run Tests")
                .key("tests"),
            wait,
            block("Deploy to Production?"),
            command("./deploy.sh production")
                .label("🚀 Deploy")
                .depends_on("tests")
        ]
    };
    println!("{}", serde_yaml::to_string(&pipeline)?);
    Ok(())
}
