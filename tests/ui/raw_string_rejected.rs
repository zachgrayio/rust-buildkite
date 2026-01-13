// This test verifies that raw string commands are always rejected - cmd!() is required

use rust_buildkite::pipeline;

fn main() {
    let _pipeline = pipeline! {
        steps: [
            command("npm install").key("install")
        ]
    };
}
