use rust_buildkite::register;

#[register(unknown_field = "value")]
pub fn my_pipeline() {}

fn main() {}
