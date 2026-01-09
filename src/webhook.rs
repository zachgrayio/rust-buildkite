//! Webhook handling for Buildkite webhook events.
//!
//! This module provides functionality to validate and parse webhook payloads
//! from Buildkite.

use crate::types::{
    AgentConnectedEvent, AgentDisconnectedEvent, AgentLostEvent, AgentStoppedEvent,
    AgentStoppingEvent, BuildFailingEvent, BuildFinishedEvent, BuildRunningEvent,
    BuildScheduledEvent, JobActivatedEvent, JobFinishedEvent, JobScheduledEvent, JobStartedEvent,
    PingEvent,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;

/// HTTP header containing the webhook event type
pub const EVENT_TYPE_HEADER: &str = "X-Buildkite-Event";

/// HTTP header containing the webhook HMAC signature (for newer webhooks)
pub const SIGNATURE_HEADER: &str = "X-Buildkite-Signature";

/// HTTP header containing the webhook token (for older webhooks)
pub const TOKEN_HEADER: &str = "X-Buildkite-Token";

/// All supported webhook event types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookEventType {
    AgentConnected,
    AgentDisconnected,
    AgentLost,
    AgentStopped,
    AgentStopping,
    BuildFailing,
    BuildFinished,
    BuildRunning,
    BuildScheduled,
    JobActivated,
    JobFinished,
    JobScheduled,
    JobStarted,
    Ping,
}

impl WebhookEventType {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            WebhookEventType::AgentConnected => "agent.connected",
            WebhookEventType::AgentDisconnected => "agent.disconnected",
            WebhookEventType::AgentLost => "agent.lost",
            WebhookEventType::AgentStopped => "agent.stopped",
            WebhookEventType::AgentStopping => "agent.stopping",
            WebhookEventType::BuildFailing => "build.failing",
            WebhookEventType::BuildFinished => "build.finished",
            WebhookEventType::BuildRunning => "build.running",
            WebhookEventType::BuildScheduled => "build.scheduled",
            WebhookEventType::JobActivated => "job.activated",
            WebhookEventType::JobFinished => "job.finished",
            WebhookEventType::JobScheduled => "job.scheduled",
            WebhookEventType::JobStarted => "job.started",
            WebhookEventType::Ping => "ping",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<WebhookEventType> {
        match s {
            "agent.connected" => Some(WebhookEventType::AgentConnected),
            "agent.disconnected" => Some(WebhookEventType::AgentDisconnected),
            "agent.lost" => Some(WebhookEventType::AgentLost),
            "agent.stopped" => Some(WebhookEventType::AgentStopped),
            "agent.stopping" => Some(WebhookEventType::AgentStopping),
            "build.failing" => Some(WebhookEventType::BuildFailing),
            "build.finished" => Some(WebhookEventType::BuildFinished),
            "build.running" => Some(WebhookEventType::BuildRunning),
            "build.scheduled" => Some(WebhookEventType::BuildScheduled),
            "job.activated" => Some(WebhookEventType::JobActivated),
            "job.finished" => Some(WebhookEventType::JobFinished),
            "job.scheduled" => Some(WebhookEventType::JobScheduled),
            "job.started" => Some(WebhookEventType::JobStarted),
            "ping" => Some(WebhookEventType::Ping),
            _ => None,
        }
    }
}

/// Parsed webhook event
#[derive(Debug, Clone)]
pub enum Event {
    AgentConnected(AgentConnectedEvent),
    AgentDisconnected(AgentDisconnectedEvent),
    AgentLost(AgentLostEvent),
    AgentStopped(AgentStoppedEvent),
    AgentStopping(AgentStoppingEvent),
    BuildFailing(BuildFailingEvent),
    BuildFinished(BuildFinishedEvent),
    BuildRunning(BuildRunningEvent),
    BuildScheduled(BuildScheduledEvent),
    JobActivated(JobActivatedEvent),
    JobFinished(JobFinishedEvent),
    JobScheduled(JobScheduledEvent),
    JobStarted(JobStartedEvent),
    Ping(PingEvent),
}

/// Webhook error types
#[derive(Debug, Clone, PartialEq)]
pub enum WebhookError {
    /// Missing required header
    MissingHeader(String),
    /// Invalid event type
    InvalidEventType(String),
    /// Invalid signature
    InvalidSignature,
    /// Invalid token
    InvalidToken,
    /// Failed to parse payload
    ParseError(String),
    /// Invalid signature format
    InvalidSignatureFormat,
}

impl std::fmt::Display for WebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookError::MissingHeader(header) => write!(f, "missing required header: {}", header),
            WebhookError::InvalidEventType(event) => write!(f, "invalid event type: {}", event),
            WebhookError::InvalidSignature => write!(f, "invalid webhook signature"),
            WebhookError::InvalidToken => write!(f, "invalid webhook token"),
            WebhookError::ParseError(msg) => write!(f, "failed to parse webhook payload: {}", msg),
            WebhookError::InvalidSignatureFormat => write!(f, "invalid signature format"),
        }
    }
}

impl std::error::Error for WebhookError {}

/// Get the webhook event type from request headers
pub fn webhook_type(headers: &HashMap<String, String>) -> Result<WebhookEventType, WebhookError> {
    let event_type = headers
        .get(EVENT_TYPE_HEADER)
        .or_else(|| headers.get(&EVENT_TYPE_HEADER.to_lowercase()))
        .ok_or_else(|| WebhookError::MissingHeader(EVENT_TYPE_HEADER.to_string()))?;

    WebhookEventType::parse(event_type)
        .ok_or_else(|| WebhookError::InvalidEventType(event_type.clone()))
}

/// Parse a webhook payload into the appropriate event type
pub fn parse_webhook(event_type: &WebhookEventType, payload: &[u8]) -> Result<Event, WebhookError> {
    let payload_str =
        std::str::from_utf8(payload).map_err(|e| WebhookError::ParseError(e.to_string()))?;

    match event_type {
        WebhookEventType::AgentConnected => {
            let event: AgentConnectedEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::AgentConnected(event))
        }
        WebhookEventType::AgentDisconnected => {
            let event: AgentDisconnectedEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::AgentDisconnected(event))
        }
        WebhookEventType::AgentLost => {
            let event: AgentLostEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::AgentLost(event))
        }
        WebhookEventType::AgentStopped => {
            let event: AgentStoppedEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::AgentStopped(event))
        }
        WebhookEventType::AgentStopping => {
            let event: AgentStoppingEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::AgentStopping(event))
        }
        WebhookEventType::BuildFailing => {
            let event: BuildFailingEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::BuildFailing(event))
        }
        WebhookEventType::BuildFinished => {
            let event: BuildFinishedEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::BuildFinished(event))
        }
        WebhookEventType::BuildRunning => {
            let event: BuildRunningEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::BuildRunning(event))
        }
        WebhookEventType::BuildScheduled => {
            let event: BuildScheduledEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::BuildScheduled(event))
        }
        WebhookEventType::JobActivated => {
            let event: JobActivatedEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::JobActivated(event))
        }
        WebhookEventType::JobFinished => {
            let event: JobFinishedEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::JobFinished(event))
        }
        WebhookEventType::JobScheduled => {
            let event: JobScheduledEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::JobScheduled(event))
        }
        WebhookEventType::JobStarted => {
            let event: JobStartedEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::JobStarted(event))
        }
        WebhookEventType::Ping => {
            let event: PingEvent = serde_json::from_str(payload_str)
                .map_err(|e| WebhookError::ParseError(e.to_string()))?;
            Ok(Event::Ping(event))
        }
    }
}

/// ValidatePayload validates an incoming webhook payload and signature against a secret.
pub fn validate_payload(
    payload: &[u8],
    signature_header: Option<&str>,
    token: Option<&str>,
    secret: &str,
) -> Result<(), WebhookError> {
    if let Some(sig_header) = signature_header {
        let (timestamp, signature) = get_timestamp_and_signature(sig_header)?;
        validate_signature(payload, &timestamp, &signature, secret)
    } else if let Some(token_value) = token {
        if token_value == secret {
            Ok(())
        } else {
            Err(WebhookError::InvalidToken)
        }
    } else {
        Err(WebhookError::MissingHeader(SIGNATURE_HEADER.to_string()))
    }
}

fn get_timestamp_and_signature(header: &str) -> Result<(String, String), WebhookError> {
    let mut timestamp = None;
    let mut signature = None;

    for part in header.split(',') {
        let mut kv = part.splitn(2, '=');
        if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
            match key.trim() {
                "timestamp" => timestamp = Some(value.to_string()),
                "signature" => signature = Some(value.to_string()),
                _ => {}
            }
        }
    }

    match (timestamp, signature) {
        (Some(ts), Some(sig)) => Ok((ts, sig)),
        _ => Err(WebhookError::InvalidSignatureFormat),
    }
}

fn validate_signature(
    payload: &[u8],
    timestamp: &str,
    signature: &str,
    secret: &str,
) -> Result<(), WebhookError> {
    let mut signed_payload = timestamp.as_bytes().to_vec();
    signed_payload.push(b'.');
    signed_payload.extend_from_slice(payload);

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|_| WebhookError::InvalidSignature)?;
    mac.update(&signed_payload);

    let expected_sig = hex::decode(signature).map_err(|_| WebhookError::InvalidSignatureFormat)?;

    mac.verify_slice(&expected_sig)
        .map_err(|_| WebhookError::InvalidSignature)
}

#[doc(hidden)]
#[must_use]
pub fn gen_mac(payload: &[u8], timestamp: &str, secret: &str) -> String {
    let mut signed_payload = timestamp.as_bytes().to_vec();
    signed_payload.push(b'.');
    signed_payload.extend_from_slice(payload);

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(&signed_payload);
    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_type_parse() {
        assert_eq!(
            WebhookEventType::parse("ping"),
            Some(WebhookEventType::Ping)
        );
        assert_eq!(
            WebhookEventType::parse("build.finished"),
            Some(WebhookEventType::BuildFinished)
        );
        assert_eq!(
            WebhookEventType::parse("agent.connected"),
            Some(WebhookEventType::AgentConnected)
        );
        assert_eq!(WebhookEventType::parse("unknown"), None);
    }

    #[test]
    fn test_webhook_type_as_str() {
        assert_eq!(WebhookEventType::Ping.as_str(), "ping");
        assert_eq!(WebhookEventType::BuildFinished.as_str(), "build.finished");
        assert_eq!(WebhookEventType::AgentConnected.as_str(), "agent.connected");
    }

    #[test]
    fn test_get_timestamp_and_signature() {
        let header = "timestamp=1234567890,signature=abc123def456";
        let (ts, sig) = get_timestamp_and_signature(header).unwrap();
        assert_eq!(ts, "1234567890");
        assert_eq!(sig, "abc123def456");
    }

    #[test]
    fn test_validate_signature() {
        let payload = b"test payload";
        let timestamp = "1234567890";
        let secret = "my-secret";
        let signature = gen_mac(payload, timestamp, secret);

        assert!(validate_signature(payload, timestamp, &signature, secret).is_ok());
        assert!(validate_signature(payload, timestamp, &signature, "wrong-secret").is_err());
    }

    #[test]
    fn test_parse_ping_event() {
        let payload =
            r#"{"event":"ping","organization":{"name":"My Org"},"sender":{"name":"Test User"}}"#;
        let event = parse_webhook(&WebhookEventType::Ping, payload.as_bytes()).unwrap();
        match event {
            Event::Ping(ping) => {
                assert_eq!(ping.event, Some("ping".to_string()));
            }
            Event::AgentConnected(_)
            | Event::AgentDisconnected(_)
            | Event::AgentLost(_)
            | Event::AgentStopped(_)
            | Event::AgentStopping(_)
            | Event::BuildFailing(_)
            | Event::BuildFinished(_)
            | Event::BuildRunning(_)
            | Event::BuildScheduled(_)
            | Event::JobActivated(_)
            | Event::JobFinished(_)
            | Event::JobScheduled(_)
            | Event::JobStarted(_) => panic!("Expected Ping event"),
        }
    }
}
