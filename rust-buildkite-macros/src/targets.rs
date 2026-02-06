//! Re-exports from shared validation crate.

pub use rust_buildkite_validation::bazel::labels::{
    extract_targets_from_args, get_current_package, is_external_repo, is_wildcard_pattern,
    should_skip_validation as should_skip_fast_validation,
};
pub use rust_buildkite_validation::bazel::targets::validate_target_exists;
