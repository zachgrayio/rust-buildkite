// This test verifies that undefined shell variables produce a compile error

use rust_buildkite::pipeline;

fn main() {
    let _pipeline = pipeline! {
        steps: [
            command(cmd!("echo $UNDEFINED_VAR")).key("test")
        ]
    };
}
