//! Bazel target validation using fast BUILD file parsing and bazel query.

use crate::debug::debug_log;
use crate::targets;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Instant, UNIX_EPOCH};

/// In-memory cache for canonicalize-flags results within a single compilation.
/// This avoids repeated disk I/O and subprocess calls for the same flags.
static FLAGS_CACHE: Mutex<Option<FlagsCache>> = Mutex::new(None);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct FlagsCache {
    /// Cache entries keyed by (verb, flags_hash)
    entries: HashMap<String, FlagsCacheEntry>,
    /// Timestamp of bazelrc files when cache was created (for invalidation)
    #[serde(default)]
    bazelrc_mtime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FlagsCacheEntry {
    canonical_flags: Vec<String>,
}

pub fn find_bazel_workspace_from_env() -> Result<std::path::PathBuf, String> {
    let (workspace, _) = find_bazel_workspace_and_script_dir()?;
    Ok(workspace)
}

pub fn find_bazel_workspace_and_script_dir(
) -> Result<(std::path::PathBuf, std::path::PathBuf), String> {
    use std::path::PathBuf;

    let start = std::env::var("RUST_SCRIPT_BASE_PATH")
        .or_else(|_| std::env::var("CARGO_MANIFEST_DIR"))
        .map(PathBuf::from)
        .map_err(|_| "Neither RUST_SCRIPT_BASE_PATH nor CARGO_MANIFEST_DIR is set".to_string())?;

    let script_dir = start.clone();

    let mut dir = start.as_path();
    loop {
        if dir.join("MODULE.bazel").exists() || dir.join("WORKSPACE").exists() {
            return Ok((dir.to_path_buf(), script_dir));
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }

    Err(format!(
        "Could not find bazel workspace (MODULE.bazel or WORKSPACE) above {}",
        start.display()
    ))
}

pub fn fast_validate_targets(
    args: &[&str],
    workspace: &Path,
    current_pkg: Option<&str>,
) -> Result<(), String> {
    let target_args = targets::extract_targets_from_args(args);

    for target in target_args {
        if targets::should_skip_fast_validation(&target) {
            debug_log!(
                "bazel",
                "Skipping wildcard/external target: {}",
                target
            );
            continue;
        }

        debug_log!("bazel", "Validating target: {}", target);
        let start = Instant::now();

        if let Err(e) = targets::validate_target_exists(workspace, &target, current_pkg) {
            debug_log!("bazel", "Target not found in {:.2?}: {}", start.elapsed(), e);
            return Err(e);
        }

        debug_log!("bazel", "Target found in {:.2?}", start.elapsed());
    }

    Ok(())
}

#[derive(Debug, Default)]
pub struct QueryResult {
    pub kind: Option<String>,
}

pub fn validate_with_query(
    verb: &str,
    args: &[&str],
    workspace: &Path,
    current_pkg: Option<&str>,
) -> Result<HashMap<String, QueryResult>, String> {
    use std::process::Command;

    let target_args = targets::extract_targets_from_args(args);
    if target_args.is_empty() {
        return Ok(HashMap::new());
    }

    let resolved_targets: Vec<String> = target_args
        .iter()
        .map(|t| {
            if targets::is_wildcard_pattern(t) || targets::is_external_repo(t) {
                t.clone()
            } else if t.starts_with("//") {
                t.clone()
            } else {
                let pkg = current_pkg.unwrap_or("");
                if t.starts_with(':') {
                    format!("//{}:{}", pkg, &t[1..])
                } else {
                    format!("//{}:{}", pkg, t)
                }
            }
        })
        .collect();

    let query_expr = resolved_targets.join(" + ");

    let mut cmd = Command::new("bazel");
    cmd.current_dir(workspace);
    cmd.args(["query", &query_expr, "--output=label_kind"]);

    debug_log!("bazel", "Running: {:?}", cmd);
    let start = Instant::now();

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run bazel query: {}", e))?;

    debug_log!("bazel", "Query completed in {:.2?}", start.elapsed());

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("no such target") || stderr.contains("no such package") {
            for line in stderr.lines() {
                if line.contains("no such target") || line.contains("no such package") {
                    return Err(line.trim().to_string());
                }
            }
        }
        return Err(format!("Bazel query failed: {}", stderr.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = HashMap::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() >= 3 && parts[1] == "rule" {
            let kind = parts[0].to_string();
            let label = parts[2].to_string();
            results.insert(label, QueryResult { kind: Some(kind) });
        }
    }

    match verb {
        "run" => {
            if results.len() != 1 {
                return Err(format!(
                    "bazel run requires exactly one target, found {}",
                    results.len()
                ));
            }
            for (target, result) in &results {
                if let Some(kind) = &result.kind {
                    validate_verb_target_compatibility(verb, target, kind)?;
                }
            }
        }
        "test" => {
            let has_test_target = results.values().any(|r| {
                r.kind
                    .as_ref()
                    .map(|k| k.contains("_test"))
                    .unwrap_or(false)
            });
            if !has_test_target && !results.is_empty() {
                let targets: Vec<_> = results.keys().collect();
                return Err(format!(
                    "No test targets found. Targets {:?} are not test targets.",
                    targets
                ));
            }
        }
        _ => {}
    }

    Ok(results)
}

pub fn validate_verb_target_compatibility(
    verb: &str,
    target: &str,
    kind: &str,
) -> Result<(), String> {
    match verb {
        "run" => {
            if !kind.contains("_binary") && !kind.contains("_test") {
                return Err(format!(
                    "Cannot run '{}': '{}' is not an executable target (must be *_binary or *_test).",
                    target, kind
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

/// Get the maximum mtime of bazelrc files in the workspace for cache invalidation.
fn get_bazelrc_mtime(workspace: &Path) -> u64 {
    let bazelrc_files = [
        ".bazelrc",
        ".bazelrc.user",
        "bazel/bazelrc",
    ];
    
    let mut max_mtime: u64 = 0;
    for file in bazelrc_files {
        if let Ok(metadata) = fs::metadata(workspace.join(file)) {
            if let Ok(mtime) = metadata.modified() {
                if let Ok(duration) = mtime.duration_since(UNIX_EPOCH) {
                    max_mtime = max_mtime.max(duration.as_secs());
                }
            }
        }
    }
    max_mtime
}

/// Generate a cache key from verb and flags.
fn make_cache_key(verb: &str, flags: &[&str]) -> String {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    
    let mut hasher = DefaultHasher::new();
    verb.hash(&mut hasher);
    for flag in flags {
        flag.hash(&mut hasher);
    }
    format!("{}_{:x}", verb, hasher.finish())
}

/// Get the cache file path for a workspace.
fn get_cache_file_path(workspace: &Path) -> std::path::PathBuf {
    workspace.join(".buildkite").join(".bazel-flags-cache.json")
}

/// Load the flags cache from disk, validating against bazelrc mtime.
fn load_flags_cache(workspace: &Path) -> FlagsCache {
    let cache_path = get_cache_file_path(workspace);
    let current_mtime = get_bazelrc_mtime(workspace);
    
    if let Ok(contents) = fs::read_to_string(&cache_path) {
        if let Ok(cache) = serde_json::from_str::<FlagsCache>(&contents) {
            // Invalidate if bazelrc files have changed
            if cache.bazelrc_mtime == current_mtime {
                debug_log!("bazel", "Loaded flags cache with {} entries", cache.entries.len());
                return cache;
            } else {
                debug_log!("bazel", "Cache invalidated: bazelrc mtime changed ({} -> {})", 
                    cache.bazelrc_mtime, current_mtime);
            }
        }
    }
    
    FlagsCache {
        entries: HashMap::new(),
        bazelrc_mtime: current_mtime,
    }
}

/// Save the flags cache to disk.
fn save_flags_cache(workspace: &Path, cache: &FlagsCache) {
    let cache_path = get_cache_file_path(workspace);
    if let Some(parent) = cache_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    
    if let Ok(json) = serde_json::to_string_pretty(cache) {
        if let Err(e) = fs::write(&cache_path, json) {
            debug_log!("bazel", "Failed to save flags cache: {}", e);
        } else {
            debug_log!("bazel", "Saved flags cache with {} entries", cache.entries.len());
        }
    }
}

pub fn canonicalize_flags(
    verb: &str,
    args: &[&str],
    workspace: &Path,
) -> Result<Vec<String>, String> {
    use std::process::Command;

    let args_before_separator: &[&str] = match args.iter().position(|&a| a == "--") {
        Some(pos) => &args[..pos],
        None => args,
    };

    let flags: Vec<&str> = args_before_separator
        .iter()
        .filter(|a| {
            a.starts_with('-')
                && !a.starts_with("-//")
                && !a.starts_with("-@")
                && !a.starts_with("-:")
        })
        .copied()
        .collect();

    if flags.is_empty() {
        return Ok(Vec::new());
    }

    let cache_key = make_cache_key(verb, &flags);
    {
        let mut guard = FLAGS_CACHE.lock().unwrap();
        if guard.is_none() {
            *guard = Some(load_flags_cache(workspace));
        }
        
        if let Some(ref cache) = *guard {
            if let Some(entry) = cache.entries.get(&cache_key) {
                debug_log!("bazel", "Cache hit for {} ({} flags)", verb, flags.len());
                return Ok(entry.canonical_flags.clone());
            }
        }
    }

    let mut cmd = Command::new("bazel");
    cmd.current_dir(workspace);
    cmd.arg("canonicalize-flags");
    cmd.arg(format!("--for_command={}", verb));
    cmd.arg("--");
    cmd.args(&flags);

    debug_log!("bazel", "Running: {:?}", cmd);
    let start = Instant::now();

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run bazel canonicalize-flags: {}", e))?;

    debug_log!(
        "bazel",
        "canonicalize-flags completed in {:.2?}",
        start.elapsed()
    );

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let error_lines: Vec<&str> = stderr
            .lines()
            .filter(|l| l.contains("Unrecognized") || l.contains("Error") || l.contains("error"))
            .collect();
        if !error_lines.is_empty() {
            return Err(format!("Invalid flags: {}", error_lines.join("; ")));
        }
        return Err(format!("Flag validation failed: {}", stderr.trim()));
    }

    let canonical: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();

    {
        let mut guard = FLAGS_CACHE.lock().unwrap();
        if let Some(ref mut cache) = *guard {
            cache.entries.insert(cache_key, FlagsCacheEntry {
                canonical_flags: canonical.clone(),
            });
            save_flags_cache(workspace, cache);
        }
    }

    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verb_target_compatibility() {
        assert!(validate_verb_target_compatibility("run", "//foo:bar", "cc_binary rule").is_ok());
        assert!(validate_verb_target_compatibility("run", "//foo:bar", "cc_test rule").is_ok());
        assert!(validate_verb_target_compatibility("run", "//foo:bar", "cc_library rule").is_err());
        assert!(validate_verb_target_compatibility("test", "//foo:bar", "cc_test rule").is_ok());
        assert!(validate_verb_target_compatibility("test", "//foo:bar", "cc_binary rule").is_ok());
        assert!(validate_verb_target_compatibility("build", "//foo:bar", "cc_library rule").is_ok());
    }
}
