// This test verifies that undefined environment variables produce a compile error
// unless they are defined in the pipeline env block

use rust_buildkite::pipeline;

fn main() {
    let _pipeline = pipeline! {
        env: {
            DEFINED_VAR: "value"
        },
        steps: [
            command(cmd!("echo \"$MY_UNDEFINED_VAR\"")).key("test")
        ]
    };
}
