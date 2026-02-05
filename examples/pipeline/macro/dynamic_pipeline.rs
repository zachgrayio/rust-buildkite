//! Example: Buildkite pipeline using the declarative pipeline! macro

use rust_buildkite::pipeline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = pipeline! {
        env: {
            CI: "true",
            NODE_ENV: "test"
        },
        expect_env: [SHELL_ENV],
        steps: [
            command {
                command: cmd!("echo 'Hello, World!'"),
                label: "👋 Say Hello"
            },
            command {
                command: cmd!("echo \"Build: $USER\""),
                label: "📋 Build Info"
            },
            command {
                command: cmd!("npm install && npm test"),
                label: ":npm: Run Tests",
                key: "tests",
                env: {
                    DEBUG: "1"
                }
            },
            wait,
            block {
                block: "Deploy to Production?",
                key: "approval",
                prompt: "Are you sure?"
            },
            command {
                command: cmd!("echo 'Deploying...'"),
                label: "🚀 Deploy",
                depends_on: ["tests", "approval"]
            }
        ]
    };

    println!("{}", serde_yaml::to_string(&pipeline)?);
    Ok(())
}
