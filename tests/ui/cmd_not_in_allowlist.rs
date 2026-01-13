// This test verifies that using a command not in the allowlist produces a compile error

use rust_buildkite::{cmd, pipeline};

fn main() {
    let _pipeline = pipeline! {
        allowed_commands: ["npm", "cargo"],
        steps: [
            command(cmd!("docker build .")).key("build")
        ]
    };
}
