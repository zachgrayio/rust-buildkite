//! Shared validation logic for rust-buildkite.
//!
//! Set `BUILDKITE_SKIP_RUNTIME_VALIDATION=1` to skip validation.
//! Set `BUILDKITE_VALIDATION_WARN_ONLY=1` to warn instead of panic.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

#[cfg(feature = "bazel")]
use std::collections::HashMap;

#[cfg(feature = "bazel")]
pub mod bazel;

/// Returns true if `BUILDKITE_SKIP_RUNTIME_VALIDATION` is set.
pub fn should_skip_validation() -> bool {
    std::env::var("BUILDKITE_SKIP_RUNTIME_VALIDATION")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Returns true if `BUILDKITE_VALIDATION_WARN_ONLY` is set.
pub fn warn_only_mode() -> bool {
    std::env::var("BUILDKITE_VALIDATION_WARN_ONLY")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn validation_failure(msg: &str) {
    if warn_only_mode() {
        eprintln!("warning: {}", msg);
    } else {
        panic!("{}", msg);
    }
}

static STATE: OnceLock<Mutex<ValidationState>> = OnceLock::new();

struct ValidationState {
    workspace: Option<PathBuf>,
    validated_paths: HashSet<String>,
    validated_env_vars: HashSet<String>,
    #[cfg(feature = "bazel")]
    bazel_workspace: Option<PathBuf>,
    #[cfg(feature = "bazel")]
    validated_targets: HashSet<String>,
    #[cfg(feature = "bazel")]
    flags_cache: HashMap<u64, FlagsCacheEntry>,
}

#[cfg(feature = "bazel")]
pub(crate) struct FlagsCacheEntry {
    #[allow(dead_code)]
    pub(crate) canonical_flags: Vec<String>,
}

fn state() -> &'static Mutex<ValidationState> {
    STATE.get_or_init(|| {
        Mutex::new(ValidationState {
            workspace: find_workspace(),
            validated_paths: HashSet::new(),
            validated_env_vars: HashSet::new(),
            #[cfg(feature = "bazel")]
            bazel_workspace: find_bazel_workspace(),
            #[cfg(feature = "bazel")]
            validated_targets: HashSet::new(),
            #[cfg(feature = "bazel")]
            flags_cache: HashMap::new(),
        })
    })
}

/// Find workspace root (Bazel or Cargo).
pub fn find_workspace() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
        let path = PathBuf::from(&dir);
        if path.exists() {
            return Some(path);
        }
    }

    let start = std::env::var("RUST_SCRIPT_BASE_PATH")
        .or_else(|_| std::env::var("CARGO_MANIFEST_DIR"))
        .ok()
        .map(PathBuf::from)?;

    let mut dir = start.as_path();
    loop {
        if dir.join("MODULE.bazel").exists() || dir.join("WORKSPACE").exists() {
            return Some(dir.to_path_buf());
        }
        if dir.join("Cargo.toml").exists()
            && let Ok(content) = std::fs::read_to_string(dir.join("Cargo.toml"))
            && content.contains("[workspace]")
        {
            return Some(dir.to_path_buf());
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }
    None
}

/// Find Bazel workspace root only (not Cargo).
#[cfg(feature = "bazel")]
pub fn find_bazel_workspace() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
        let path = PathBuf::from(&dir);
        if path.exists() && (path.join("MODULE.bazel").exists() || path.join("WORKSPACE").exists())
        {
            return Some(path);
        }
    }

    let start = std::env::var("RUST_SCRIPT_BASE_PATH")
        .or_else(|_| std::env::var("CARGO_MANIFEST_DIR"))
        .ok()
        .map(PathBuf::from)?;

    let mut dir = start.as_path();
    loop {
        if dir.join("MODULE.bazel").exists() || dir.join("WORKSPACE").exists() {
            return Some(dir.to_path_buf());
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }
    None
}

/// Initialize validation state early.
pub fn init() {
    let _ = state();
}

/// Get cached workspace path.
#[must_use]
pub fn workspace() -> Option<PathBuf> {
    state().lock().ok()?.workspace.clone()
}

/// Validate path exists. Results are cached.
pub fn validate_path(path: &str) {
    if should_skip_validation() {
        return;
    }

    let guard = match state().lock() {
        Ok(g) => g,
        Err(_) => return,
    };

    if guard.validated_paths.contains(path) {
        return;
    }

    let Some(ref ws) = guard.workspace else {
        return;
    };

    let full_path = if path.starts_with('/') {
        PathBuf::from(path)
    } else if let Some(stripped) = path.strip_prefix("./") {
        ws.join(stripped)
    } else {
        ws.join(path)
    };

    drop(guard);

    if !full_path.exists() {
        validation_failure(&format!(
            "Path validation failed: '{}' does not exist (resolved to {})",
            path,
            full_path.display()
        ));
        return;
    }

    if let Ok(mut guard) = state().lock() {
        guard.validated_paths.insert(path.to_string());
    }
}

/// Validate multiple paths.
pub fn validate_paths(paths: &[&str]) {
    for path in paths {
        validate_path(path);
    }
}

/// Validate environment variable exists.
pub fn validate_env_var(name: &str) {
    if should_skip_validation() {
        return;
    }

    let guard = match state().lock() {
        Ok(g) => g,
        Err(_) => return,
    };

    if guard.validated_env_vars.contains(name) {
        return;
    }
    drop(guard);

    if std::env::var(name).is_err() {
        validation_failure(&format!("Environment variable '{}' is not set", name));
        return;
    }

    if let Ok(mut guard) = state().lock() {
        guard.validated_env_vars.insert(name.to_string());
    }
}

/// Validate multiple environment variables.
pub fn validate_env_vars(names: &[&str]) {
    for name in names {
        validate_env_var(name);
    }
}

/// Check if path exists without panicking.
pub fn check_path_exists(path: &str) -> Result<(), String> {
    let guard = match state().lock() {
        Ok(g) => g,
        Err(_) => return Ok(()),
    };

    let Some(ref ws) = guard.workspace else {
        return Ok(());
    };

    let full_path = if path.starts_with('/') {
        PathBuf::from(path)
    } else if let Some(stripped) = path.strip_prefix("./") {
        ws.join(stripped)
    } else {
        ws.join(path)
    };

    drop(guard);

    if full_path.exists() {
        Ok(())
    } else {
        Err(format!(
            "Path '{}' does not exist (resolved to {})",
            path,
            full_path.display()
        ))
    }
}

#[cfg(feature = "bazel")]
pub use bazel::{
    flags, labels, targets, validate_flags, validate_flags_str, validate_target, validate_targets,
};

#[cfg(not(feature = "bazel"))]
pub fn validate_target(_target: &str) {}

#[cfg(not(feature = "bazel"))]
pub fn validate_targets(_targets: &str) {}

#[cfg(not(feature = "bazel"))]
pub fn validate_flags(_verb: &str, _flags: &[&str]) {}

#[cfg(not(feature = "bazel"))]
pub fn validate_flags_str(_verb: &str, _flags: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_is_idempotent() {
        init();
        init();
        init();
    }

    #[test]
    fn test_workspace_returns_clone() {
        let ws1 = workspace();
        let ws2 = workspace();
        assert_eq!(ws1, ws2);
    }

    #[test]
    fn test_validate_paths_batch() {
        validate_paths(&[]);
    }

    #[test]
    #[cfg(not(feature = "bazel"))]
    fn test_validation_stubs_are_noop() {
        validate_target("anything");
        validate_targets("//foo //bar");
        validate_flags("build", &["--config=ci"]);
        validate_flags_str("test", "--some-flag");
    }
}
