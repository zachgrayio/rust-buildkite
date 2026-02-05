use rust_buildkite::{BranchPattern, register, registered_pipelines};

#[register]
pub fn simple_pipeline() {}

#[register(cron = "0 7 * * *")]
pub fn scheduled_pipeline() {}

#[register(branch = Exact("main"))]
pub fn branch_exact_pipeline() {}

#[register(branch = Prefix("release/"))]
pub fn branch_prefix_pipeline() {}

#[register(returns_early = true)]
pub fn early_return_pipeline() {}

#[register(cron = "0 0 * * *", returns_early = true)]
pub fn combined_attrs_pipeline() {}

#[test]
fn test_simple_registration() {
    let pipelines: Vec<_> = registered_pipelines().collect();
    let simple = pipelines.iter().find(|p| p.id == "SimplePipeline");
    assert!(simple.is_some(), "SimplePipeline should be registered");
    let simple = simple.unwrap();
    assert_eq!(simple.name, "Simple Pipeline");
    assert!(simple.branch.is_none());
    assert!(simple.cron.is_none());
    assert!(!simple.returns_early);
}

#[test]
fn test_scheduled_registration() {
    let pipelines: Vec<_> = registered_pipelines().collect();
    let scheduled = pipelines.iter().find(|p| p.id == "ScheduledPipeline");
    assert!(
        scheduled.is_some(),
        "ScheduledPipeline should be registered"
    );
    let scheduled = scheduled.unwrap();
    assert_eq!(scheduled.name, "Scheduled Pipeline");
    assert_eq!(scheduled.cron, Some("0 7 * * *"));
}

#[test]
fn test_branch_exact_registration() {
    let pipelines: Vec<_> = registered_pipelines().collect();
    let branch_exact = pipelines.iter().find(|p| p.id == "BranchExactPipeline");
    assert!(
        branch_exact.is_some(),
        "BranchExactPipeline should be registered"
    );
    let branch_exact = branch_exact.unwrap();
    assert_eq!(branch_exact.name, "Branch Exact Pipeline");
    assert!(matches!(
        branch_exact.branch,
        Some(BranchPattern::Exact("main"))
    ));
}

#[test]
fn test_branch_prefix_registration() {
    let pipelines: Vec<_> = registered_pipelines().collect();
    let branch_prefix = pipelines.iter().find(|p| p.id == "BranchPrefixPipeline");
    assert!(
        branch_prefix.is_some(),
        "BranchPrefixPipeline should be registered"
    );
    let branch_prefix = branch_prefix.unwrap();
    assert!(matches!(
        branch_prefix.branch,
        Some(BranchPattern::Prefix("release/"))
    ));
}

#[test]
fn test_early_return_registration() {
    let pipelines: Vec<_> = registered_pipelines().collect();
    let early = pipelines.iter().find(|p| p.id == "EarlyReturnPipeline");
    assert!(early.is_some(), "EarlyReturnPipeline should be registered");
    let early = early.unwrap();
    assert!(early.returns_early);
}

#[test]
fn test_combined_attrs_registration() {
    let pipelines: Vec<_> = registered_pipelines().collect();
    let combined = pipelines.iter().find(|p| p.id == "CombinedAttrsPipeline");
    assert!(
        combined.is_some(),
        "CombinedAttrsPipeline should be registered"
    );
    let combined = combined.unwrap();
    assert_eq!(combined.cron, Some("0 0 * * *"));
    assert!(combined.returns_early);
}

#[test]
fn test_snake_to_pascal_conversion() {
    let pipelines: Vec<_> = registered_pipelines().collect();
    let early = pipelines.iter().find(|p| p.id == "EarlyReturnPipeline");
    assert!(early.is_some());
    assert_eq!(early.unwrap().id, "EarlyReturnPipeline");
}

#[test]
fn test_snake_to_title_conversion() {
    let pipelines: Vec<_> = registered_pipelines().collect();
    let early = pipelines.iter().find(|p| p.id == "EarlyReturnPipeline");
    assert!(early.is_some());
    assert_eq!(early.unwrap().name, "Early Return Pipeline");
}

#[test]
fn test_branch_pattern_matches() {
    assert!(BranchPattern::Exact("main").matches("main"));
    assert!(!BranchPattern::Exact("main").matches("master"));

    assert!(BranchPattern::Prefix("release/").matches("release/1.0"));
    assert!(BranchPattern::Prefix("release/").matches("release/2.0.0"));
    assert!(!BranchPattern::Prefix("release/").matches("feature/x"));

    assert!(BranchPattern::AnyPrefix(&["release/", "hotfix/"]).matches("release/1.0"));
    assert!(BranchPattern::AnyPrefix(&["release/", "hotfix/"]).matches("hotfix/urgent"));
    assert!(!BranchPattern::AnyPrefix(&["release/", "hotfix/"]).matches("feature/x"));
}

#[test]
fn test_generate_fn_callable() {
    let pipelines: Vec<_> = registered_pipelines().collect();
    let simple = pipelines.iter().find(|p| p.id == "SimplePipeline").unwrap();
    (simple.generate)();
}
