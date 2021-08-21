use serde::{Deserialize, Serialize};

// For nixpkgs base branches. E.g. master, staging, or staging-next
#[derive(sqlx::FromRow, Debug, PartialEq, Serialize, Deserialize)]
struct Branch {
    id: u64,
    title: String,
}

// Build status of a derivation
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum BuildStatus {
    Unknown,
    Queued,
    InProgress,
    Failed,
    Success,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Serialize, Deserialize)]
struct Commit {
    rev_hash: String,
    branch_id: Option<u64>,
    pull_request_id: Option<u64>,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Serialize, Deserialize)]
struct Drv {
    id: u64,
    drv_path: String,
    attr: String, // An attr path which can be used to "hydra" the drv
    previous_drv: Option<String>,
    platform_id: u32,
    commit_rev_hash: String,
    build_status_id: BuildStatus,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Serialize, Deserialize)]
struct Platform {
    id: u32,
    platform: String,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Serialize, Deserialize)]
struct PullRequest {
    id: u64, // same as pull request number
}
