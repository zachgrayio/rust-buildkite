use rust_buildkite::{cmd, pipeline};

fn main() {
    let _pipeline = pipeline! {
        steps: [
            command(cmd!("scripts/nonexistent.sh")).key("script")
        ]
    };
}
