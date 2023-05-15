use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Input
pub struct DeploymentsFetcherParams {
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

// Output
#[derive(Debug, Clone)]
pub struct CommitItem {
    pub sha: String,
    pub message: String,
    pub resource_path: String,
    pub committed_at: DateTime<Utc>,
    pub creator_login: String,
}

#[derive(Debug, Clone)]
pub struct RepositoryInfo {
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum CommitOrRepositoryInfo {
    Commit(CommitItem),
    RepositoryInfo(RepositoryInfo),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeploymentInfo {
    GithubDeployment { id: String },
    HerokuRelease { id: String, version: u64 },
}

#[derive(Debug, Clone)]
pub struct DeploymentItem {
    pub info: DeploymentInfo,
    pub head_commit: CommitItem,
    pub base: CommitOrRepositoryInfo,
    pub creator_login: String,
    pub deployed_at: DateTime<Utc>,
}

// Errors
#[derive(Debug, Error)]
pub enum DeploymentsFetcherError {
    #[error("Create API client error")]
    CreateAPIClientError(#[source] anyhow::Error),
    #[error("Fetch deployments error")]
    FetchError(#[source] anyhow::Error),
    #[error("Get commit error")]
    CommitIsNotFound(#[source] anyhow::Error),
    #[error("Cannot get repository created at")]
    GetRepositoryCreatedAtError(#[source] anyhow::Error),
    #[error("Fetch deployments result is empty list")]
    DeploymentsFetcherResultIsEmptyList(#[source] anyhow::Error),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

// Workflow
#[async_trait]
pub trait DeploymentsFetcher {
    async fn fetch(
        &self,
        params: DeploymentsFetcherParams,
    ) -> Result<Vec<DeploymentItem>, DeploymentsFetcherError>;
}
