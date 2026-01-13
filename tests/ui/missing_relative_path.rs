// This test verifies that using a relative path that doesn't exist produces a compile error

use rust_buildkite::{cmd, pipeline};

fn main() {
    let _pipeline = pipeline! {
        steps: [
            command(cmd!("./nonexistent-script.sh")).key("script")
        ]
    };
}
