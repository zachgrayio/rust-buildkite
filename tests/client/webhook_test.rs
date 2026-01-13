use rust_buildkite::webhook::{
    EVENT_TYPE_HEADER, Event, WebhookError, WebhookEventType, parse_webhook, validate_payload,
    webhook_type,
};
use std::collections::HashMap;

#[test]
fn test_parse_all_event_types() {
    let test_cases = vec![
        (
            WebhookEventType::AgentConnected,
            r#"{"event":"agent.connected"}"#,
        ),
        (
            WebhookEventType::AgentDisconnected,
            r#"{"event":"agent.disconnected"}"#,
        ),
        (WebhookEventType::AgentLost, r#"{"event":"agent.lost"}"#),
        (
            WebhookEventType::AgentStopped,
            r#"{"event":"agent.stopped"}"#,
        ),
        (
            WebhookEventType::AgentStopping,
            r#"{"event":"agent.stopping"}"#,
        ),
        (
            WebhookEventType::BuildFailing,
            r#"{"event":"build.failing"}"#,
        ),
        (
            WebhookEventType::BuildFinished,
            r#"{"event":"build.finished"}"#,
        ),
        (
            WebhookEventType::BuildRunning,
            r#"{"event":"build.running"}"#,
        ),
        (
            WebhookEventType::BuildScheduled,
            r#"{"event":"build.scheduled"}"#,
        ),
        (
            WebhookEventType::JobActivated,
            r#"{"event":"job.activated"}"#,
        ),
        (WebhookEventType::JobFinished, r#"{"event":"job.finished"}"#),
        (
            WebhookEventType::JobScheduled,
            r#"{"event":"job.scheduled"}"#,
        ),
        (WebhookEventType::JobStarted, r#"{"event":"job.started"}"#),
        (WebhookEventType::Ping, r#"{"event":"ping"}"#),
    ];

    for (event_type, payload) in test_cases {
        let result = parse_webhook(&event_type, payload.as_bytes());
        assert!(
            result.is_ok(),
            "Failed to parse {:?}: {:?}",
            event_type,
            result
        );

        match (event_type, result.unwrap()) {
            (WebhookEventType::AgentConnected, Event::AgentConnected(_)) => {}
            (WebhookEventType::AgentDisconnected, Event::AgentDisconnected(_)) => {}
            (WebhookEventType::AgentLost, Event::AgentLost(_)) => {}
            (WebhookEventType::AgentStopped, Event::AgentStopped(_)) => {}
            (WebhookEventType::AgentStopping, Event::AgentStopping(_)) => {}
            (WebhookEventType::BuildFailing, Event::BuildFailing(_)) => {}
            (WebhookEventType::BuildFinished, Event::BuildFinished(_)) => {}
            (WebhookEventType::BuildRunning, Event::BuildRunning(_)) => {}
            (WebhookEventType::BuildScheduled, Event::BuildScheduled(_)) => {}
            (WebhookEventType::JobActivated, Event::JobActivated(_)) => {}
            (WebhookEventType::JobFinished, Event::JobFinished(_)) => {}
            (WebhookEventType::JobScheduled, Event::JobScheduled(_)) => {}
            (WebhookEventType::JobStarted, Event::JobStarted(_)) => {}
            (WebhookEventType::Ping, Event::Ping(_)) => {}
            _ => panic!("Event type mismatch"),
        }
    }
}

#[test]
fn test_parse_unknown_event_type() {
    let result = WebhookEventType::parse("invalid");
    assert!(result.is_none());
}

#[test]
fn test_validate_payload_missing_auth_headers() {
    let payload = b"test payload";
    let result = validate_payload(payload, None, None, "secret");
    assert!(matches!(result, Err(WebhookError::MissingHeader(_))));
}

#[test]
fn test_validate_payload_invalid_signature_format() {
    let payload = b"test payload";
    let result = validate_payload(payload, Some("invalid"), None, "secret");
    assert!(matches!(result, Err(WebhookError::InvalidSignatureFormat)));
}

#[test]
fn test_validate_payload_non_hex_signature() {
    let payload = b"test payload";
    let result = validate_payload(
        payload,
        Some("timestamp=1642080837,signature=yo"),
        None,
        "secret",
    );
    assert!(matches!(result, Err(WebhookError::InvalidSignatureFormat)));
}

#[test]
fn test_validate_payload_invalid_signature() {
    let payload = b"test payload";
    let timestamp = "1234567890";
    let secret = "my-secret";

    let valid_signature = rust_buildkite::webhook::gen_mac(payload, timestamp, secret);
    let invalid_signature = valid_signature.replace("a", "b");

    let sig_header = format!("timestamp={},signature={}", timestamp, invalid_signature);
    let result = validate_payload(payload, Some(&sig_header), None, secret);
    assert!(matches!(result, Err(WebhookError::InvalidSignature)));
}

#[test]
fn test_validate_payload_invalid_token() {
    let payload = b"test payload";
    let result = validate_payload(payload, None, Some("invalid-token"), "correct-token");
    assert!(matches!(result, Err(WebhookError::InvalidToken)));
}

#[test]
fn test_validate_payload_valid_signature() {
    let payload = b"test payload";
    let timestamp = "1642080837";
    let secret = "29b1ff5779c76bd48ba6705eb99ff970";

    let signature = rust_buildkite::webhook::gen_mac(payload, timestamp, secret);
    let sig_header = format!("timestamp={},signature={}", timestamp, signature);
    let result = validate_payload(payload, Some(&sig_header), None, secret);
    assert!(
        result.is_ok(),
        "Expected valid signature, got: {:?}",
        result
    );
}

#[test]
fn test_validate_payload_valid_token() {
    let payload = b"test payload";
    let token = "29b1ff5779c76bd48ba6705eb99ff970";
    let result = validate_payload(payload, None, Some(token), token);
    assert!(result.is_ok());
}

#[test]
fn test_webhook_type_extraction() {
    let mut headers = HashMap::new();
    headers.insert(EVENT_TYPE_HEADER.to_string(), "ping".to_string());

    let result = webhook_type(&headers);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), WebhookEventType::Ping);
}

#[test]
fn test_webhook_type_extraction_lowercase_header() {
    let mut headers = HashMap::new();
    headers.insert(
        EVENT_TYPE_HEADER.to_lowercase(),
        "build.finished".to_string(),
    );

    let result = webhook_type(&headers);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), WebhookEventType::BuildFinished);
}

#[test]
fn test_webhook_type_missing_header() {
    let headers = HashMap::new();
    let result = webhook_type(&headers);
    assert!(matches!(result, Err(WebhookError::MissingHeader(_))));
}

#[test]
fn test_webhook_type_invalid_event() {
    let mut headers = HashMap::new();
    headers.insert(EVENT_TYPE_HEADER.to_string(), "invalid.event".to_string());

    let result = webhook_type(&headers);
    assert!(matches!(result, Err(WebhookError::InvalidEventType(_))));
}
