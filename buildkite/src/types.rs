use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ConnectionState {
    #[serde(alias = "connected")]
    Connected,
    #[serde(alias = "disconnected")]
    Disconnected,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum BuildState {
    #[serde(alias = "creating")]
    Creating,
    #[serde(alias = "scheduled")]
    Scheduled,
    #[serde(alias = "running")]
    Running,
    #[serde(alias = "passed")]
    Passed,
    #[serde(alias = "failed")]
    Failed,
    #[serde(alias = "blocked")]
    Blocked,
    #[serde(alias = "canceling")]
    Canceling,
    #[serde(alias = "canceled")]
    Canceled,
    #[serde(alias = "skipped")]
    Skipped,
    #[serde(alias = "not_run")]
    NotRunning,
    #[serde(alias = "finished")]
    Finished,
}

impl std::fmt::Display for BuildState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum JobState {
    // Passed and Failed are not listed in the docs:
    // https://buildkite.com/docs/pipelines/defining-steps#job-states
    // However, they are both valid job states.
    #[serde(alias = "passed")]
    Passed,
    #[serde(alias = "failed")]
    Failed,

    #[serde(alias = "pending")]
    Pending,
    #[serde(alias = "waiting")]
    Waiting,
    #[serde(alias = "waiting_failed")]
    WaitingFailed,
    #[serde(alias = "blocked")]
    Blocked,
    #[serde(alias = "blocked_failed")]
    BlockedFailed,
    #[serde(alias = "unblocked")]
    Unblocked,
    #[serde(alias = "unblocked_failed")]
    UnblockedFailed,
    #[serde(alias = "limiting")]
    Limiting,
    #[serde(alias = "limited")]
    Limited,
    #[serde(alias = "scheduled")]
    Scheduled,
    #[serde(alias = "assigned")]
    Assigned,
    #[serde(alias = "accepted")]
    Accepted,
    #[serde(alias = "running")]
    Running,
    #[serde(alias = "finished")]
    Finished,
    #[serde(alias = "canceling")]
    Canceling,
    #[serde(alias = "canceled")]
    Canceled,
    #[serde(alias = "timing_out")]
    TimingOut,
    #[serde(alias = "timed_out")]
    TimedOut,
    #[serde(alias = "skipped")]
    Skipped,
    #[serde(alias = "broken")]
    Broken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub username: Option<String>,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creator {
    pub id: String,
    pub graphql_id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderSettings {
    pub trigger_mode: String,
    pub build_pull_requests: Option<bool>,
    pub pull_request_branch_filter_enabled: Option<bool>,
    pub skip_builds_for_existing_commits: Option<bool>,
    pub skip_pull_request_builds_for_existing_commits: Option<bool>,
    pub build_pull_request_ready_for_review: Option<bool>,
    pub build_pull_request_labels_changed: Option<bool>,
    pub build_pull_request_forks: Option<bool>,
    pub prefix_pull_request_fork_branch_names: Option<bool>,
    pub build_branches: Option<bool>,
    pub build_tags: Option<bool>,
    pub cancel_deleted_branch_builds: Option<bool>,
    pub publish_commit_status: Option<bool>,
    pub publish_commit_status_per_step: Option<bool>,
    pub separate_pull_request_statuses: Option<bool>,
    pub publish_blocked_as_pending: Option<bool>,
    pub use_step_key_as_commit_status: Option<bool>,
    pub filter_enabled: Option<bool>,
    pub repository: Option<String>,
    pub pull_request_branch_filter_configuration: Option<String>,
    pub filter_condition: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub settings: ProviderSettings,
    pub webhook_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Step {
    #[serde(rename = "type")]
    pub step_type: String,
    pub name: String,
    pub command: String,
    pub artifact_paths: Option<String>,
    pub branch_configuration: Option<String>,
    pub env: HashMap<String, String>,
    pub timeout_in_minutes: Option<u64>,
    pub agent_query_rules: Vec<String>,
    pub concurrency: Option<u64>,
    pub parallelism: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: String,
    pub graphql_id: String,
    pub url: String,
    pub web_url: String,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub repository: String,
    pub cluster_id: Option<String>,
    pub branch_configuration: Option<String>,
    pub default_branch: Option<String>,
    pub skip_queued_branch_builds: bool,
    pub skip_queued_branch_builds_filter: Option<String>,
    pub cancel_running_branch_builds: bool,
    pub cancel_running_branch_builds_filter: Option<String>,
    pub allow_rebuilds: bool,
    pub provider: Provider,
    pub builds_url: String,
    pub badge_url: String,
    pub created_by: Creator,
    pub created_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
    pub env: Option<HashMap<String, String>>,
    pub scheduled_builds_count: u64,
    pub running_builds_count: u64,
    pub scheduled_jobs_count: u64,
    pub running_jobs_count: u64,
    pub waiting_jobs_count: u64,
    pub visibility: String,
    pub tags: Option<String>,
    pub configuration: Option<String>,
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: String,
    pub base: String,
    pub repository: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RebuiltFrom {
    pub id: String,
    pub number: u64,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: String,
    pub graphql_id: String,
    pub url: String,
    pub web_url: String,
    pub number: u64,
    pub state: BuildState,
    pub blocked: bool,
    pub blocked_state: String,
    pub message: String,
    pub commit: String,
    pub branch: String,
    pub tag: Option<String>,
    pub env: HashMap<String, String>,
    pub source: String,
    pub author: Option<Author>,
    pub creator: Option<Creator>,
    pub created_at: DateTime<Utc>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub meta_data: HashMap<String, String>,
    pub pull_request: Option<PullRequest>,
    pub rebuilt_from: Option<RebuiltFrom>,
    pub pipeline: Pipeline,
    pub jobs: Vec<Job>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub name: Option<String>,
    pub state: Option<JobState>,
    pub web_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub content: String,
    pub header_times: Option<Vec<u64>>,
    pub size: u64,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub graphql_id: String,
    pub url: String,
    pub web_url: String,
    pub name: String,
    pub slug: String,
    pub agents_url: String,
    pub emojis_url: String,
    pub created_at: DateTime<Utc>,
    pub pipelines_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub url: String,
    pub web_url: String,
    pub name: String,
    pub connection_state: ConnectionState,
    pub ip_address: String,
    pub hostname: String,
    pub user_agent: String,
    pub version: String,
    pub creator: Option<Creator>,
    pub created_at: DateTime<Utc>,
    pub job: Option<Job>,
    pub last_job_finished_at: Option<DateTime<Utc>>,
    pub priority: u64,
    pub meta_data: Vec<String>,
}

/// A `Result` alias where the `Err` case is `reqwest::Error`
pub type Result<T> = std::result::Result<T, reqwest::Error>;
