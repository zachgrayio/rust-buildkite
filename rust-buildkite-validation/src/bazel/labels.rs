//! Bazel label parsing.

/// Check if label is relative.
pub fn is_relative_label(label: &str) -> bool {
    !label.starts_with("//") && !label.starts_with('@')
}

/// Resolve label to (package, target).
pub fn resolve_label(label: &str, current_pkg: Option<&str>) -> Result<(String, String), String> {
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
        let pkg = current_pkg.unwrap_or("");
        Ok((pkg.to_string(), target.to_string()))
    } else if !label.contains('/') && !label.contains(':') {
        let pkg = current_pkg.unwrap_or("");
        Ok((pkg.to_string(), label.to_string()))
    } else {
        Err(format!(
            "Invalid label '{}': relative paths with / are not supported",
            label
        ))
    }
}

/// Parse absolute label to (package, target).
pub fn parse_label(label: &str) -> Result<(String, String), String> {
    resolve_label(label, None)
}

/// Check if target is a wildcard pattern.
pub fn is_wildcard_pattern(target: &str) -> bool {
    target.contains("...") || target.ends_with(":all") || target.ends_with(":*")
}

/// Check if target is from external repository.
pub fn is_external_repo(target: &str) -> bool {
    target.starts_with('@') && !target.starts_with("@@")
}

/// Check if target should skip validation.
pub fn should_skip_validation(target: &str) -> bool {
    is_wildcard_pattern(target) || is_external_repo(target)
}

/// Extract targets from args, stopping at --.
pub fn extract_targets_from_args(args: &[&str]) -> Vec<String> {
    let args_before_separator: &[&str] = match args.iter().position(|&a| a == "--") {
        Some(pos) => &args[..pos],
        None => args,
    };

    args_before_separator
        .iter()
        .filter(|arg| !arg.starts_with('-'))
        .filter(|arg| arg.starts_with("//") || arg.starts_with('@') || arg.starts_with(':'))
        .map(|s| s.to_string())
        .collect()
}

/// Get current package from workspace and script paths.
pub fn get_current_package(
    workspace: &std::path::Path,
    script_dir: &std::path::Path,
) -> Option<String> {
    script_dir
        .strip_prefix(workspace)
        .ok()
        .map(|rel| rel.to_string_lossy().to_string())
        .map(|s| s.replace('\\', "/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_label_absolute() {
        let (pkg, target) = resolve_label("//cpp:hello-world", None).unwrap();
        assert_eq!(pkg, "cpp");
        assert_eq!(target, "hello-world");
    }

    #[test]
    fn test_resolve_label_absolute_no_target() {
        let (pkg, target) = resolve_label("//cpp/subdir", None).unwrap();
        assert_eq!(pkg, "cpp/subdir");
        assert_eq!(target, "subdir");
    }

    #[test]
    fn test_resolve_label_root_package() {
        let (pkg, target) = resolve_label("//:main", None).unwrap();
        assert_eq!(pkg, "");
        assert_eq!(target, "main");
    }

    #[test]
    fn test_resolve_label_relative_colon() {
        let (pkg, target) = resolve_label(":baz", Some("foo/bar")).unwrap();
        assert_eq!(pkg, "foo/bar");
        assert_eq!(target, "baz");
    }

    #[test]
    fn test_resolve_label_relative_bare() {
        let (pkg, target) = resolve_label("baz", Some("foo/bar")).unwrap();
        assert_eq!(pkg, "foo/bar");
        assert_eq!(target, "baz");
    }

    #[test]
    fn test_resolve_label_relative_no_current_pkg() {
        let (pkg, target) = resolve_label(":main", None).unwrap();
        assert_eq!(pkg, "");
        assert_eq!(target, "main");
    }

    #[test]
    fn test_is_relative_label() {
        assert!(is_relative_label(":baz"));
        assert!(is_relative_label("baz"));
        assert!(!is_relative_label("//foo:bar"));
        assert!(!is_relative_label("@repo//foo:bar"));
    }

    #[test]
    fn test_is_wildcard_pattern() {
        assert!(is_wildcard_pattern("//..."));
        assert!(is_wildcard_pattern("//foo/..."));
        assert!(is_wildcard_pattern("//foo:all"));
        assert!(is_wildcard_pattern("//foo:*"));
        assert!(is_wildcard_pattern(":all"));
        assert!(!is_wildcard_pattern("//foo:bar"));
        assert!(!is_wildcard_pattern(":bar"));
    }

    #[test]
    fn test_is_external_repo() {
        assert!(is_external_repo("@rules_cc//cc:defs"));
        assert!(!is_external_repo("//foo:bar"));
        assert!(!is_external_repo("@@//foo:bar"));
    }

    #[test]
    fn test_extract_targets_from_args() {
        let args = vec!["//foo:bar", "--config=opt", "//baz:qux", "-k", ":local"];
        let targets = extract_targets_from_args(&args);
        assert_eq!(targets, vec!["//foo:bar", "//baz:qux", ":local"]);
    }

    #[test]
    fn test_extract_targets_stops_at_separator() {
        let args = vec!["//foo:bar", "--config=opt", "--", "--cicd_env", "stage"];
        let targets = extract_targets_from_args(&args);
        assert_eq!(targets, vec!["//foo:bar"]);
    }

    #[test]
    fn test_get_current_package() {
        use std::path::Path;
        let workspace = Path::new("/home/user/myproject");
        let script_dir = Path::new("/home/user/myproject/foo/bar");
        assert_eq!(
            get_current_package(workspace, script_dir),
            Some("foo/bar".to_string())
        );
    }

    #[test]
    fn test_get_current_package_root() {
        use std::path::Path;
        let workspace = Path::new("/home/user/myproject");
        let script_dir = Path::new("/home/user/myproject");
        assert_eq!(
            get_current_package(workspace, script_dir),
            Some("".to_string())
        );
    }
}
