use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Timestamp wrapper for time handling
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub time::OffsetDateTime);

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let formatted = self
            .0
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = time::OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
            .map_err(serde::de::Error::custom)?;
        Ok(Timestamp(dt))
    }
}

/// User represents a buildkite user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

/// Creator represents the creator of a build
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

/// ClusterCreator represents the creator of a cluster, cluster queue, or cluster token
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ClusterCreator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

/// PipelineTemplateCreator represents the creator of a pipeline template
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PipelineTemplateCreator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

/// Organization represents a buildkite organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Organization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emojis_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

/// Team represents a buildkite team
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Team {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<User>,
}

/// CreateTeam represents a request to create a team
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTeam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_team: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_can_create_pipelines: Option<bool>,
}

/// TeamMember represents a member of a team.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TeamMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

/// CreateTeamMember specifies the input parameters for creating a team member
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateTeamMember {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TeamPipeline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TeamSuite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

/// AccessTokenUser represents the user who owns the access token
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AccessTokenUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// AccessToken represents an API access token
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AccessToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<AccessTokenUser>,
}

/// Token represents an OAuth access token for the buildkite service
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

/// Annotation represents an annotation which has been stored from a build
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Annotation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append: Option<bool>,
}

/// Agent represents a buildkite build agent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Agent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_job_finished_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<Job>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_by: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_timeout_in_minutes: Option<i32>,
}

/// AgentPauseOptions specifies options for pausing an agent
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentPauseOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
}

/// AgentListOptions specifies the optional parameters to the AgentsService.List method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// Author of a commit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Author {
    String(String),
    Object(AuthorObject),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AuthorObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// RebuiltFrom references a previous build
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RebuiltFrom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// PullRequest represents a Github PR
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PullRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TriggeredFrom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_pipeline_slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TestEngineSuite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TestEngineRun {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite: Option<TestEngineSuite>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TestEngineProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<Vec<TestEngineRun>>,
}

/// Build represents a build which has run in buildkite
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Build {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Box<Pipeline>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuilt_from: Option<RebuiltFrom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_from: Option<TriggeredFrom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_engine: Option<TestEngineProperty>,
}

/// CreateBuild - Create a build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBuild {
    pub commit: String,
    pub branch: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<AuthorObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_checkout: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_pipeline_branch_filters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_base_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_repository: Option<String>,
}

/// JobRetrySource represents what triggered this retry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRetrySource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_type: Option<String>,
}

/// UnblockedBy represents the unblocked status of a job, when present
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UnblockedBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TriggeredBuild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

/// JobPriority represents the priority of the job
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobPriority {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

/// StepSignature represents the signature of a step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StepSignature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_fields: Option<Vec<String>>,
}

/// StepInfo contains information about a step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StepInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<StepSignature>,
}

/// Job represents a job run during a build in buildkite
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Job {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub job_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_log_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_paths: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runnable_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unblocked_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_query_rules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried_in_job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_source: Option<JobRetrySource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_failed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unblocked_by: Option<UnblockedBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unblockable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unblock_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_group_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_group_total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_queue_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_build: Option<TriggeredBuild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<JobPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<StepInfo>,
}

/// JobUnblockOptions specifies the optional parameters to UnblockJob
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JobUnblockOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, String>>,
}

/// JobLog represents a job log output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_times: Option<Vec<i64>>,
}

/// JobEnvs represent job environments output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobEnvs {
    #[serde(skip_serializing_if = "Option::is_none", rename = "env")]
    pub environment_variables: Option<HashMap<String, String>>,
}

/// Provider represents a source code provider
#[derive(Debug, Clone, PartialEq)]
pub struct Provider {
    pub id: String,
    pub webhook_url: Option<String>,
    pub settings: ProviderSettings,
}

impl<'de> serde::Deserialize<'de> for Provider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;

        let value = Value::deserialize(deserializer)?;
        let id = value
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| D::Error::missing_field("id"))?
            .to_string();
        let webhook_url = value
            .get("webhook_url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let settings = match value.get("settings") {
            Some(settings_value) => match id.as_str() {
                "bitbucket" => ProviderSettings::Bitbucket(
                    serde_json::from_value(settings_value.clone()).map_err(D::Error::custom)?,
                ),
                "github" => ProviderSettings::GitHub(
                    serde_json::from_value(settings_value.clone()).map_err(D::Error::custom)?,
                ),
                "github_enterprise" => ProviderSettings::GitHubEnterprise(
                    serde_json::from_value(settings_value.clone()).map_err(D::Error::custom)?,
                ),
                "gitlab" | "gitlab_ee" => ProviderSettings::GitLab(
                    serde_json::from_value(settings_value.clone()).map_err(D::Error::custom)?,
                ),
                _ => ProviderSettings::Unknown,
            },
            None => ProviderSettings::Unknown,
        };

        Ok(Provider {
            id,
            webhook_url,
            settings,
        })
    }
}

impl serde::Serialize for Provider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Provider", 3)?;
        state.serialize_field("id", &self.id)?;
        if let Some(ref webhook_url) = self.webhook_url {
            state.serialize_field("webhook_url", webhook_url)?;
        }
        state.serialize_field("settings", &self.settings)?;
        state.end()
    }
}

/// Settings for pipelines building from Bitbucket repositories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BitbucketSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_pull_requests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_branches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_branch_filter_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_branch_filter_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_pull_request_builds_for_existing_commits: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_tags: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_commit_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_commit_status_per_step: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

/// Settings for pipelines building from GitHub repositories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GitHubSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_pull_requests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_branches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_branch_filter_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_branch_filter_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_pull_request_builds_for_existing_commits: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_pull_request_forks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_pull_request_fork_branch_names: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_tags: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_commit_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_commit_status_per_step: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_pull_request_statuses: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_blocked_as_pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

/// Settings for pipelines building from GitHub Enterprise repositories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GitHubEnterpriseSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_pull_requests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_branches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_branch_filter_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_branch_filter_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_pull_request_builds_for_existing_commits: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_tags: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_commit_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_commit_status_per_step: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

/// Settings for pipelines building from GitLab repositories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GitLabSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

/// ProviderSettings represents the sum type of settings for different source code providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ProviderSettings {
    Bitbucket(BitbucketSettings),
    GitHub(GitHubSettings),
    GitHubEnterprise(GitHubEnterpriseSettings),
    GitLab(GitLabSettings),
    Unknown,
}

/// Step represents a build step in buildkites build pipeline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Step {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub step_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_paths: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_query_rules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<serde_json::Value>,
}

/// Pipeline represents a buildkite pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Pipeline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_queued_branch_builds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_queued_branch_builds_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_running_branch_builds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_running_branch_builds_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_builds_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_builds_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_jobs_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_jobs_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting_jobs_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Step>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_command_step_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_command_step_timeout: Option<i32>,
}

/// CreatePipeline - Create a Pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePipeline {
    pub name: String,
    pub repository: String,
    pub cluster_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Step>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_command_step_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_command_step_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_settings: Option<ProviderSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_queued_branch_builds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_queued_branch_builds_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_running_branch_builds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_running_branch_builds_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_uuids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePipeline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Step>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_command_step_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_command_step_timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_settings: Option<ProviderSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_queued_branch_builds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_queued_branch_builds_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_running_branch_builds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_running_branch_builds_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ClusterMaintainer {
    #[serde(skip_serializing_if = "Option::is_none", rename = "user")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "team")]
    pub team_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ClusterMaintainerActor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub actor_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// ClusterMaintainerEntry represents either a user or a team which is indicated by the Type field in the Actor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ClusterMaintainerEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<ClusterMaintainerActor>,
}

/// ClusterMaintainersList represents the maintainers of a cluster with separate lists for users and teams.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ClusterMaintainersList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ClusterMaintainerEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<ClusterMaintainerEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Cluster {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_queue_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_queue_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ClusterCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainers: Option<ClusterMaintainersList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterCreate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainers: Option<Vec<ClusterMaintainer>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_queue_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ClusterToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ClusterCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ip_addresses: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterTokenCreateUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ip_addresses: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum RetryAgentAffinity {
    #[default]
    #[serde(rename = "prefer-warmest")]
    PreferWarmest,
    #[serde(rename = "prefer-different")]
    PreferDifferent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ClusterQueue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_agent_affinity: Option<RetryAgentAffinity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_paused_by: Option<ClusterCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_paused_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatch_paused_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<ClusterCreator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterQueueCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_agent_affinity: Option<RetryAgentAffinity>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterQueueUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_agent_affinity: Option<RetryAgentAffinity>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterQueuePause {
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "dispatch_paused_note"
    )]
    pub note: Option<String>,
}

/// Artifact represents an artifact which has been stored from a build
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Artifact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dirname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glob_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sha1sum")]
    pub sha1: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FlakyTest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_instance_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Test {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TestRun {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
}

/// FailureExpanded contains expanded failure details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FailureExpanded {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrace: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<Vec<String>>,
}

/// FailedExecution represents a failed test execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FailedExecution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_expanded: Option<Vec<FailureExpanded>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_url: Option<String>,
}

/// FailedExecutionsOptions specifies the optional parameters to GetFailedExecutions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FailedExecutionsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_failure_expanded: Option<bool>,
}

/// TestRunsListOptions specifies the optional parameters to the TestRunsService.List method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestRunsListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// FlakyTestsListOptions specifies the optional parameters to the FlakyTestsService.List method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlakyTestsListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// TestSuiteListOptions specifies the optional parameters to the TestSuitesService.List method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestSuiteListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TestSuite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteCreate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_api_token: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "team_ids")]
    pub team_uuids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestSuiteUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
}

/// RateLimit represents the shape of the rate limit response (a list of scopes)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RateLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<RateLimitScopes>,
}

/// RateLimitScopes contains GraphQL and REST rate limit details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RateLimitScopes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql: Option<RateLimitDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest: Option<RateLimitDetails>,
}

/// RateLimitDetails describes the shape of a scope's response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RateLimitDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_at: Option<Timestamp>,
}

/// PackageRegistry represents a package registry within Buildkite
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PackageRegistry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecosystem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_policy: Option<String>,
}

pub type PackageRegistryOIDCPolicy = Vec<OIDCPolicyStatement>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OIDCPolicyStatement {
    #[serde(rename = "iss", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<HashMap<String, ClaimRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClaimRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<serde_json::Value>,
    #[serde(rename = "in", skip_serializing_if = "Option::is_none")]
    pub in_list: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_in: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<String>>,
}

/// CreatePackageRegistryInput represents the input to create a package registry.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePackageRegistryInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecosystem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_policy: Option<PackageRegistryOIDCPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePackageRegistryInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_policy: Option<PackageRegistryOIDCPolicy>,
}

/// Package represents a package which has been stored in a registry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Package {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<PackageRegistry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistryPackagesLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "prev")]
    pub previous: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "self")]
    pub current: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistryPackages {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Package>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<RegistryPackagesLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegistryPackagesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<String>,
}

/// PackagePresignedUpload represents a presigned upload URL for a Buildkite package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagePresignedUpload {
    pub uri: String,
    pub form: PackagePresignedUploadForm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagePresignedUploadForm {
    pub file_input: String,
    pub method: String,
    pub url: String,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PipelineTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<PipelineTemplateCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<PipelineTemplateCreator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PipelineTemplateCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PipelineTemplateUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

/// ListOptions specifies the optional parameters to various List methods that support pagination.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// PipelineListOptions specifies the optional parameters to the PipelinesService.List method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PipelineListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

/// BuildsListOptions specifies the optional parameters to the BuildsService.List method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildsListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_from: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_to: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_from: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_retried_jobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_pipeline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_jobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildGetOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_retried_jobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_test_engine: Option<bool>,
}

/// AnnotationListOptions specifies the optional parameters to the AnnotationsService.List method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnnotationListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// ArtifactListOptions specifies the optional parameters to the ArtifactsService.List method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtifactListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClustersListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterQueuesListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterTokensListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

/// AgentEvent is a wrapper for an agent event notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgentEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<User>,
}

/// AgentConnectedEvent is triggered when an agent has connected to the API
pub type AgentConnectedEvent = AgentEvent;

/// AgentDisconnectedEvent is triggered when an agent has disconnected
pub type AgentDisconnectedEvent = AgentEvent;

/// AgentLostEvent is triggered when an agent has been marked as lost
pub type AgentLostEvent = AgentEvent;

/// AgentStoppedEvent is triggered when an agent has stopped
pub type AgentStoppedEvent = AgentEvent;

/// AgentStoppingEvent is triggered when an agent is stopping
pub type AgentStoppingEvent = AgentEvent;

/// BuildEvent is a wrapper for a build event notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BuildEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Pipeline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<User>,
}

/// BuildFailingEvent is triggered when a build enters a failing state
pub type BuildFailingEvent = BuildEvent;

/// BuildFinishedEvent is triggered when a build finishes
pub type BuildFinishedEvent = BuildEvent;

/// BuildRunningEvent is triggered when a build starts running
pub type BuildRunningEvent = BuildEvent;

/// BuildScheduledEvent is triggered when a build is scheduled
pub type BuildScheduledEvent = BuildEvent;

/// JobEvent is a wrapper for a job event notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Pipeline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<User>,
}

/// JobActivatedEvent is triggered when a job is activated
pub type JobActivatedEvent = JobEvent;

/// JobFinishedEvent is triggered when a job is finished
pub type JobFinishedEvent = JobEvent;

/// JobScheduledEvent is triggered when a job is scheduled
pub type JobScheduledEvent = JobEvent;

/// JobStartedEvent is triggered when a job is started
pub type JobStartedEvent = JobEvent;

/// PingEvent is triggered when a webhook notification setting is changed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PingEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<User>,
}

/// Emoji represents a buildkite emoji
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Emoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// TeamsListOptions specifies the optional parameters to the TeamsService.List method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamsListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// OrganizationListOptions specifies the optional parameters to the OrganizationsService.List method.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}
