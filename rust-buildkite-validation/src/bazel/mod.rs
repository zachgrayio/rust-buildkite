//! Bazel target and flag validation.

pub mod flags;
pub mod labels;
pub mod targets;

use crate::{FlagsCacheEntry, should_skip_validation, state, validation_failure};
use std::sync::atomic::{AtomicBool, Ordering};

static WARNED_NO_WORKSPACE: AtomicBool = AtomicBool::new(false);

fn skip_no_workspace() {
    if !WARNED_NO_WORKSPACE.swap(true, Ordering::Relaxed) {
        eprintln!(
            "warning: Bazel workspace not found, skipping Bazel validation. \
             Set BUILD_WORKSPACE_DIRECTORY or run from a Bazel workspace."
        );
    }
}

/// Validate a single Bazel target.
pub fn validate_target(target: &str) {
    if should_skip_validation() {
        return;
    }

    if labels::should_skip_validation(target) {
        return;
    }

    let mut guard = match state().lock() {
        Ok(g) => g,
        Err(_) => return,
    };

    if guard.validated_targets.contains(target) {
        return;
    }

    let Some(ref ws) = guard.bazel_workspace else {
        skip_no_workspace();
        return;
    };
    let ws = ws.clone();

    let (pkg, name) = match labels::parse_label(target) {
        Ok(p) => p,
        Err(_) => return,
    };

    let build_path = match targets::find_build_file(&ws, &pkg) {
        Ok(p) => p,
        Err(e) => {
            drop(guard);
            validation_failure(&format!("Target validation failed for '{}': {}", target, e));
            return;
        }
    };

    let target_exists = match targets::target_exists_in_build_file(&build_path, &name) {
        Ok(exists) => exists,
        Err(e) => {
            drop(guard);
            validation_failure(&format!(
                "Failed to parse BUILD file for '{}': {}",
                target, e
            ));
            return;
        }
    };

    if !target_exists {
        drop(guard);
        validation_failure(&format!(
            "Target '//{}:{}' not found in {}",
            pkg,
            name,
            build_path.display()
        ));
        return;
    }

    guard.validated_targets.insert(target.to_string());
}

/// Validate multiple targets from whitespace-separated string.
pub fn validate_targets(targets: &str) {
    for target in targets.split_whitespace() {
        if target.starts_with('-') {
            continue;
        }
        validate_target(target);
    }
}

/// Validate flags using `bazel canonicalize-flags`.
pub fn validate_flags(verb: &str, flags_slice: &[&str]) {
    if should_skip_validation() {
        return;
    }

    if flags_slice.is_empty() {
        return;
    }

    let before_separator: &[&str] = flags_slice
        .iter()
        .position(|f| *f == "--")
        .map(|pos| &flags_slice[..pos])
        .unwrap_or(flags_slice);

    let flag_vec: Vec<&str> = before_separator
        .iter()
        .filter(|f| f.starts_with('-') && !f.starts_with("-//") && !f.starts_with("-@"))
        .copied()
        .collect();

    if flag_vec.is_empty() {
        return;
    }

    let cache_key = flags::make_cache_key(verb, &flag_vec);

    let guard = match state().lock() {
        Ok(g) => g,
        Err(_) => return,
    };

    if guard.flags_cache.contains_key(&cache_key) {
        return;
    }

    let Some(ref ws) = guard.bazel_workspace else {
        skip_no_workspace();
        return;
    };
    let ws = ws.clone();
    drop(guard);

    let output = std::process::Command::new("bazel")
        .current_dir(&ws)
        .arg("canonicalize-flags")
        .arg(format!("--for_command={}", verb))
        .arg("--")
        .args(&flag_vec)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let canonical: Vec<String> = String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|s| s.to_string())
                .collect();

            if let Ok(mut guard) = state().lock() {
                guard.flags_cache.insert(
                    cache_key,
                    FlagsCacheEntry {
                        canonical_flags: canonical,
                    },
                );
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let error_msg = stderr
                .lines()
                .find(|l| l.contains("Unrecognized") || l.contains("Error"))
                .unwrap_or(stderr.trim());
            validation_failure(&format!(
                "Flag validation failed for {:?}: {}",
                flag_vec, error_msg
            ));
        }
        Err(e) => {
            eprintln!("Warning: Could not validate flags (bazel not found): {}", e);
        }
    }
}

/// Validate flags from whitespace-separated string.
pub fn validate_flags_str(verb: &str, flags: &str) {
    let parts: Vec<&str> = flags.split_whitespace().collect();
    validate_flags(verb, &parts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_target_skips_wildcards() {
        validate_target("//...");
        validate_target("//app:all");
        validate_target("//app:*");
        validate_target("@external//lib:util");
    }

    #[test]
    fn test_validate_targets_handles_mixed() {
        validate_targets("//... //app:all @external//lib:util");
    }

    #[test]
    fn test_validate_targets_skips_subtractions() {
        validate_targets("-//excluded:target //... -@external//lib:util");
    }

    #[test]
    fn test_validate_target_skips_invalid_labels() {
        validate_target("not-a-label");
        validate_target("invalid pattern");
    }
}
