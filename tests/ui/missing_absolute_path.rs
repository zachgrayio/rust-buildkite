// This test verifies that using an absolute path that doesn't exist produces a compile error

use rust_buildkite::{cmd, pipeline};

fn main() {
    let _pipeline = pipeline! {
        steps: [
            command(cmd!("/nonexistent/path/to/binary")).key("binary")
        ]
    };
}
