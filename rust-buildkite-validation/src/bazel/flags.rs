//! Bazel flag validation.

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;

static FLAGS_CACHE: Mutex<Option<FlagsCache>> = Mutex::new(None);

#[derive(Default)]
struct FlagsCache {
    entries: HashMap<u64, FlagsCacheEntry>,
}

struct FlagsCacheEntry {
    #[allow(dead_code)]
    canonical_flags: Vec<String>,
}

/// Generate cache key for (verb, flags).
pub fn make_cache_key(verb: &str, flags: &[&str]) -> u64 {
    let mut hasher = DefaultHasher::new();
    verb.hash(&mut hasher);
    for flag in flags {
        flag.hash(&mut hasher);
    }
    hasher.finish()
}

fn is_cached(key: u64) -> bool {
    FLAGS_CACHE
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().map(|c| c.entries.contains_key(&key)))
        .unwrap_or(false)
}

fn cache_flags(key: u64, canonical: Vec<String>) {
    if let Ok(mut guard) = FLAGS_CACHE.lock() {
        let cache = guard.get_or_insert_with(FlagsCache::default);
        cache.entries.insert(
            key,
            FlagsCacheEntry {
                canonical_flags: canonical,
            },
        );
    }
}

/// Validate flags via `bazel canonicalize-flags`.
pub fn validate_flags_with_bazel(
    verb: &str,
    flags: &[&str],
    workspace: &Path,
) -> Result<(), String> {
    let before_separator: &[&str] = flags
        .iter()
        .position(|f| *f == "--")
        .map(|pos| &flags[..pos])
        .unwrap_or(flags);

    let flag_vec: Vec<&str> = before_separator
        .iter()
        .filter(|f| f.starts_with('-') && !f.starts_with("-//") && !f.starts_with("-@"))
        .copied()
        .collect();

    if flag_vec.is_empty() {
        return Ok(());
    }

    let cache_key = make_cache_key(verb, &flag_vec);
    if is_cached(cache_key) {
        return Ok(());
    }

    let output = Command::new("bazel")
        .current_dir(workspace)
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
            cache_flags(cache_key, canonical);
            Ok(())
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let error_msg = stderr
                .lines()
                .find(|l| l.contains("Unrecognized") || l.contains("Error"))
                .unwrap_or(stderr.trim());
            Err(format!(
                "Flag validation failed for {:?}: {}",
                flag_vec, error_msg
            ))
        }
        Err(e) => {
            eprintln!("Warning: Could not validate flags (bazel not found): {}", e);
            Ok(())
        }
    }
}

/// Validate flags from string.
pub fn validate_flags_str_with_bazel(
    verb: &str,
    flags: &str,
    workspace: &Path,
) -> Result<(), String> {
    let parts: Vec<&str> = flags.split_whitespace().collect();
    validate_flags_with_bazel(verb, &parts, workspace)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_cache_key_deterministic() {
        let key1 = make_cache_key("build", &["--config=ci"]);
        let key2 = make_cache_key("build", &["--config=ci"]);
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_make_cache_key_differs_by_verb() {
        let key1 = make_cache_key("build", &["--config=ci"]);
        let key2 = make_cache_key("test", &["--config=ci"]);
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_make_cache_key_differs_by_flags() {
        let key1 = make_cache_key("build", &["--config=ci"]);
        let key2 = make_cache_key("build", &["--config=opt"]);
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_filter_excludes_target_subtractions() {
        let flags = vec!["-//foo:bar", "--config=ci", "-@external//lib"];
        let filtered: Vec<&str> = flags
            .iter()
            .filter(|f| f.starts_with('-') && !f.starts_with("-//") && !f.starts_with("-@"))
            .copied()
            .collect();
        assert_eq!(filtered, vec!["--config=ci"]);
    }
}
