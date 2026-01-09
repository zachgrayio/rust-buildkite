
use rust_buildkite::*;

#[test]
fn test_command_step_with_automatic_retry() {
    let retry = CommandStepRetry::Automatic(CommandStepRetryVariant0::Object(AutomaticRetry {
        exit_status: None,
        limit: Some(2),
        signal_reason: None,
    }));
    
    let step = CommandStep {
        retry: Some(retry),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""retry""#));
}
