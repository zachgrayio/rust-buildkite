use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

#[cfg(feature = "bazel")]
use std::collections::{HashMap, HashSet};

static STATE: OnceLock<Mutex<ValidationState>> = OnceLock::new();

struct ValidationState {
    workspace: Option<PathBuf>,
    #[cfg(feature = "bazel")]
    validated_targets: HashSet<String>,
    #[cfg(feature = "bazel")]
    flags_cache: HashMap<u64, FlagsCacheEntry>,
}

#[cfg(feature = "bazel")]
struct FlagsCacheEntry {
    #[allow(dead_code)]
    canonical_flags: Vec<String>,
}

fn state() -> &'static Mutex<ValidationState> {
    STATE.get_or_init(|| {
        Mutex::new(ValidationState {
            workspace: find_workspace(),
            #[cfg(feature = "bazel")]
            validated_targets: HashSet::new(),
            #[cfg(feature = "bazel")]
            flags_cache: HashMap::new(),
        })
    })
}

fn find_workspace() -> Option<PathBuf> {
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

pub fn init() {
    let _ = state();
}

#[must_use]
pub fn workspace() -> Option<PathBuf> {
    state().lock().ok()?.workspace.clone()
}

pub fn validate_path(path: &str) {
    let guard = match state().lock() {
        Ok(g) => g,
        Err(_) => return,
    };

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
        panic!(
            "Path validation failed: '{}' does not exist (resolved to {})",
            path,
            full_path.display()
        );
    }
}

pub fn validate_paths(paths: &[&str]) {
    for path in paths {
        validate_path(path);
    }
}

#[cfg(feature = "bazel")]
mod bazel_validation {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::path::Path;
    use std::process::Command;

    pub fn validate_target(target: &str) {
        if should_skip(target) {
            return;
        }

        let mut guard = match state().lock() {
            Ok(g) => g,
            Err(_) => return,
        };

        if guard.validated_targets.contains(target) {
            return;
        }

        let Some(ref ws) = guard.workspace else {
            return;
        };
        let ws = ws.clone();

        let (pkg, name) = match parse_label(target) {
            Ok(p) => p,
            Err(_) => {
                return;
            }
        };

        let build_path = match find_build_file(&ws, &pkg) {
            Ok(p) => p,
            Err(e) => {
                drop(guard);
                panic!("Target validation failed for '{}': {}", target, e);
            }
        };

        let target_exists = match target_exists_in_build_file(&build_path, &name) {
            Ok(exists) => exists,
            Err(e) => {
                drop(guard);
                panic!("Failed to parse BUILD file for '{}': {}", target, e);
            }
        };

        if !target_exists {
            drop(guard);
            panic!(
                "Target '//{}:{}' not found in {}",
                pkg,
                name,
                build_path.display()
            );
        }

        guard.validated_targets.insert(target.to_string());
    }

    pub fn validate_targets(targets: &str) {
        for target in targets.split_whitespace() {
            if target.starts_with('-') {
                continue;
            }
            validate_target(target);
        }
    }

    pub fn validate_flags(verb: &str, flags: &[&str]) {
        if flags.is_empty() {
            return;
        }

        let flag_vec: Vec<&str> = flags
            .iter()
            .filter(|f| f.starts_with('-') && !f.starts_with("-//") && !f.starts_with("-@"))
            .copied()
            .collect();

        if flag_vec.is_empty() {
            return;
        }

        let cache_key = make_flags_cache_key(verb, &flag_vec);

        let guard = match state().lock() {
            Ok(g) => g,
            Err(_) => return,
        };

        if guard.flags_cache.contains_key(&cache_key) {
            return;
        }

        let Some(ref ws) = guard.workspace else {
            return;
        };
        let ws = ws.clone();
        drop(guard);

        let output = Command::new("bazel")
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
                panic!("Flag validation failed for {:?}: {}", flag_vec, error_msg);
            }
            Err(e) => {
                eprintln!("Warning: Could not validate flags (bazel not found): {}", e);
            }
        }
    }

    pub(crate) fn should_skip(target: &str) -> bool {
        target.contains("...")
            || target.ends_with(":all")
            || target.ends_with(":*")
            || (target.starts_with('@') && !target.starts_with("@@"))
    }

    pub(crate) fn make_flags_cache_key(verb: &str, flags: &[&str]) -> u64 {
        let mut hasher = DefaultHasher::new();
        verb.hash(&mut hasher);
        for flag in flags {
            flag.hash(&mut hasher);
        }
        hasher.finish()
    }

    pub(crate) fn parse_label(label: &str) -> Result<(String, String), String> {
        let label = label.trim_start_matches('@').trim_start_matches('@');

        if let Some(without_slashes) = label.strip_prefix("//") {
            if let Some((pkg, target)) = without_slashes.split_once(':') {
                Ok((pkg.to_string(), target.to_string()))
            } else {
                let pkg = without_slashes;
                let target = pkg.rsplit('/').next().unwrap_or(pkg);
                Ok((pkg.to_string(), target.to_string()))
            }
        } else if let Some(target) = label.strip_prefix(':') {
            Ok(("".to_string(), target.to_string()))
        } else {
            Err(format!("Invalid label format: {}", label))
        }
    }

    fn find_build_file(workspace: &Path, pkg: &str) -> Result<PathBuf, String> {
        let pkg_dir = workspace.join(pkg);

        for name in &["BUILD.bazel", "BUILD"] {
            let path = pkg_dir.join(name);
            if path.exists() {
                return Ok(path);
            }
        }

        Err(format!(
            "No BUILD file found for package '{}' (looked in {})",
            pkg,
            pkg_dir.display()
        ))
    }

    fn target_exists_in_build_file(path: &Path, target_name: &str) -> Result<bool, String> {
        use starlark::analysis::find_call_name::AstModuleFindCallName;
        use starlark::syntax::{AstModule, Dialect};

        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let ast = AstModule::parse("BUILD", content, &Dialect::Standard)
            .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;

        Ok(ast.find_function_call_with_name(target_name).is_some())
    }
}

#[cfg(feature = "bazel")]
pub use bazel_validation::{validate_flags, validate_target, validate_targets};

#[cfg(feature = "bazel")]
pub fn validate_flags_str(verb: &str, flags: &str) {
    let parts: Vec<&str> = flags.split_whitespace().collect();
    validate_flags(verb, &parts);
}

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
    fn test_validate_path_skips_when_no_workspace() {
        validate_path("/nonexistent/path");
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_parse_label_absolute() {
        let (pkg, target) = bazel_validation::parse_label("//app:main").unwrap();
        assert_eq!(pkg, "app");
        assert_eq!(target, "main");
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_parse_label_root_package() {
        let (pkg, target) = bazel_validation::parse_label("//:main").unwrap();
        assert_eq!(pkg, "");
        assert_eq!(target, "main");
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_parse_label_nested_package() {
        let (pkg, target) = bazel_validation::parse_label("//foo/bar:baz").unwrap();
        assert_eq!(pkg, "foo/bar");
        assert_eq!(target, "baz");
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_parse_label_implicit_target() {
        let (pkg, target) = bazel_validation::parse_label("//foo/bar").unwrap();
        assert_eq!(pkg, "foo/bar");
        assert_eq!(target, "bar");
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_should_skip_wildcards() {
        assert!(bazel_validation::should_skip("//..."));
        assert!(bazel_validation::should_skip("//app:all"));
        assert!(bazel_validation::should_skip("//app:*"));
        assert!(bazel_validation::should_skip("@external//app:main"));
        assert!(!bazel_validation::should_skip("//app:main"));
        assert!(!bazel_validation::should_skip("@@//app:main"));
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_flags_cache_key_deterministic() {
        let key1 = bazel_validation::make_flags_cache_key("build", &["--config=ci"]);
        let key2 = bazel_validation::make_flags_cache_key("build", &["--config=ci"]);
        assert_eq!(key1, key2);
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_flags_cache_key_differs_by_verb() {
        let key1 = bazel_validation::make_flags_cache_key("build", &["--config=ci"]);
        let key2 = bazel_validation::make_flags_cache_key("test", &["--config=ci"]);
        assert_ne!(key1, key2);
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_flags_cache_key_differs_by_flags() {
        let key1 = bazel_validation::make_flags_cache_key("build", &["--config=ci"]);
        let key2 = bazel_validation::make_flags_cache_key("build", &["--config=opt"]);
        assert_ne!(key1, key2);
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_validate_target_skips_wildcards() {
        validate_target("//...");
        validate_target("//app:all");
        validate_target("//app:*");
        validate_target("@external//lib:util");
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_validate_targets_handles_mixed() {
        validate_targets("//... //app:all @external//lib:util");
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_validate_targets_skips_flags() {
        validate_targets("-//excluded:target //app:main -@external//lib:util");
    }

    #[test]
    #[cfg(feature = "bazel")]
    fn test_validate_target_skips_invalid_labels() {
        validate_target("not-a-label");
        validate_target("invalid pattern");
    }

    #[test]
    fn test_validate_paths_batch() {
        validate_paths(&[]);
    }

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
    #[cfg(not(feature = "bazel"))]
    fn test_validation_stubs_are_noop() {
        validate_target("anything");
        validate_targets("//foo //bar");
        validate_flags("build", &["--config=ci"]);
        validate_flags_str("test", "--some-flag");
    }
}
