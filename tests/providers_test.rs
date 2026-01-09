use rust_buildkite::{Provider, ProviderSettings};

#[test]
fn test_provider_bitbucket() {
    let json = r#"{"id":"bitbucket","settings":{"repository":"my-bitbucket-repo"}}"#;
    let provider: Provider = serde_json::from_str(json).unwrap();

    assert_eq!(provider.id, "bitbucket");
    match provider.settings {
        ProviderSettings::Bitbucket(settings) => {
            assert_eq!(settings.repository, Some("my-bitbucket-repo".to_string()));
        }
        ProviderSettings::GitHub(_)
        | ProviderSettings::GitHubEnterprise(_)
        | ProviderSettings::GitLab(_)
        | ProviderSettings::Unknown => panic!("Expected Bitbucket settings"),
    }
}

#[test]
fn test_provider_github() {
    let json = r#"{"id":"github","settings":{"repository":"my-github-repo"}}"#;
    let provider: Provider = serde_json::from_str(json).unwrap();

    assert_eq!(provider.id, "github");
    match provider.settings {
        ProviderSettings::GitHub(settings) => {
            assert_eq!(settings.repository, Some("my-github-repo".to_string()));
        }
        ProviderSettings::Bitbucket(_)
        | ProviderSettings::GitHubEnterprise(_)
        | ProviderSettings::GitLab(_)
        | ProviderSettings::Unknown => panic!("Expected GitHub settings"),
    }
}

#[test]
fn test_provider_github_enterprise() {
    let json =
        r#"{"id":"github_enterprise","settings":{"repository":"my-github-enterprise-repo"}}"#;
    let provider: Provider = serde_json::from_str(json).unwrap();

    assert_eq!(provider.id, "github_enterprise");
    match provider.settings {
        ProviderSettings::GitHubEnterprise(settings) => {
            assert_eq!(
                settings.repository,
                Some("my-github-enterprise-repo".to_string())
            );
        }
        ProviderSettings::Bitbucket(_)
        | ProviderSettings::GitHub(_)
        | ProviderSettings::GitLab(_)
        | ProviderSettings::Unknown => panic!("Expected GitHub Enterprise settings"),
    }
}

#[test]
fn test_provider_gitlab() {
    let json = r#"{"id":"gitlab","settings":{"repository":"my-gitlab-repo"}}"#;
    let provider: Provider = serde_json::from_str(json).unwrap();

    assert_eq!(provider.id, "gitlab");
    match provider.settings {
        ProviderSettings::GitLab(settings) => {
            assert_eq!(settings.repository, Some("my-gitlab-repo".to_string()));
        }
        ProviderSettings::Bitbucket(_)
        | ProviderSettings::GitHub(_)
        | ProviderSettings::GitHubEnterprise(_)
        | ProviderSettings::Unknown => panic!("Expected GitLab settings"),
    }
}

#[test]
fn test_provider_gitlab_ee() {
    let json = r#"{"id":"gitlab_ee","settings":{"repository":"my-gitlab-repo"}}"#;
    let provider: Provider = serde_json::from_str(json).unwrap();

    assert_eq!(provider.id, "gitlab_ee");
    match provider.settings {
        ProviderSettings::GitLab(settings) => {
            assert_eq!(settings.repository, Some("my-gitlab-repo".to_string()));
        }
        ProviderSettings::Bitbucket(_)
        | ProviderSettings::GitHub(_)
        | ProviderSettings::GitHubEnterprise(_)
        | ProviderSettings::Unknown => panic!("Expected GitLab settings"),
    }
}

#[test]
fn test_provider_unknown() {
    let json = r#"{"id":"unknown","settings":{"emoji":":shrug:"}}"#;
    let provider: Provider = serde_json::from_str(json).unwrap();

    assert_eq!(provider.id, "unknown");
    assert!(matches!(provider.settings, ProviderSettings::Unknown));
}

#[test]
fn test_provider_with_common_settings() {
    let json = r#"{
        "id":"github",
        "settings":{
            "repository":"test-repo",
            "trigger_mode":"code",
            "build_pull_requests":true,
            "skip_pull_request_builds_for_existing_commits":false
        }
    }"#;
    let provider: Provider = serde_json::from_str(json).unwrap();

    assert_eq!(provider.id, "github");
    match provider.settings {
        ProviderSettings::GitHub(settings) => {
            assert_eq!(settings.repository, Some("test-repo".to_string()));
            assert_eq!(settings.trigger_mode, Some("code".to_string()));
            assert_eq!(settings.build_pull_requests, Some(true));
            assert_eq!(
                settings.skip_pull_request_builds_for_existing_commits,
                Some(false)
            );
        }
        ProviderSettings::Bitbucket(_)
        | ProviderSettings::GitHubEnterprise(_)
        | ProviderSettings::GitLab(_)
        | ProviderSettings::Unknown => panic!("Expected GitHub settings"),
    }
}
