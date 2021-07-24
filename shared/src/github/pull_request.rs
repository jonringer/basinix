// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct PullRequest {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "number")]
    number: i64,

    #[serde(rename = "state")]
    state: String,

    #[serde(rename = "locked")]
    locked: Option<bool>,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "user")]
    user: User,

    #[serde(rename = "closed_at")]
    closed_at: Option<serde_json::Value>,

    #[serde(rename = "merged_at")]
    merged_at: Option<serde_json::Value>,

    #[serde(rename = "merge_commit_sha")]
    merge_commit_sha: Option<String>,

    #[serde(rename = "assignee")]
    assignee: Option<serde_json::Value>,

    #[serde(rename = "assignees")]
    assignees: Vec<Option<serde_json::Value>>,

    #[serde(rename = "requested_reviewers")]
    requested_reviewers: Vec<User>,

    #[serde(rename = "requested_teams")]
    requested_teams: Vec<Option<serde_json::Value>>,

    #[serde(rename = "labels")]
    labels: Vec<Label>,

    #[serde(rename = "milestone")]
    milestone: Option<serde_json::Value>,

    #[serde(rename = "draft")]
    draft: Option<bool>,

    #[serde(rename = "head")]
    pub head: Base,

    #[serde(rename = "base")]
    pub base: Base,

    #[serde(rename = "_links")]
    links: Links,

    #[serde(rename = "auto_merge")]
    auto_merge: Option<serde_json::Value>,

    #[serde(rename = "active_lock_reason")]
    active_lock_reason: Option<serde_json::Value>,

    #[serde(rename = "merged")]
    merged: Option<bool>,

    #[serde(rename = "mergeable")]
    mergeable: Option<bool>,

    #[serde(rename = "merged_by")]
    merged_by: Option<serde_json::Value>,

    #[serde(rename = "comments")]
    comments: i64,

    #[serde(rename = "review_comments")]
    review_comments: i64,

    #[serde(rename = "commits")]
    commits: i64,

    #[serde(rename = "additions")]
    additions: i64,

    #[serde(rename = "deletions")]
    deletions: i64,

    #[serde(rename = "changed_files")]
    changed_files: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Base {
    #[serde(rename = "ref")]
    pub base_ref: String,

    #[serde(rename = "sha")]
    pub sha: String,

    #[serde(rename = "user")]
    user: User,
}

#[derive(Serialize, Deserialize)]
pub struct Repo {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "owner")]
    owner: User,

    #[serde(rename = "size")]
    size: i64,

    #[serde(rename = "stargazers_count")]
    stargazers_count: i64,

    #[serde(rename = "watchers_count")]
    watchers_count: i64,

    #[serde(rename = "forks_count")]
    forks_count: i64,

    #[serde(rename = "mirror_url")]
    mirror_url: Option<serde_json::Value>,

    #[serde(rename = "open_issues_count")]
    open_issues_count: i64,

    #[serde(rename = "license")]
    license: Option<License>,

    #[serde(rename = "forks")]
    forks: i64,

    #[serde(rename = "open_issues")]
    open_issues: i64,

    #[serde(rename = "watchers")]
    watchers: i64,
}

#[derive(Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "key")]
    key: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "spdx_id")]
    spdx_id: String,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "type")]
    user_type: Type,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
    #[serde(rename = "id")]
    id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    links_self: Comments,

    #[serde(rename = "html")]
    html: Comments,

    #[serde(rename = "issue")]
    issue: Comments,

    #[serde(rename = "comments")]
    comments: Comments,

    #[serde(rename = "review_comments")]
    review_comments: Comments,

    #[serde(rename = "review_comment")]
    review_comment: Comments,

    #[serde(rename = "commits")]
    commits: Comments,

    #[serde(rename = "statuses")]
    statuses: Comments,
}

#[derive(Serialize, Deserialize)]
pub struct Comments {
    #[serde(rename = "href")]
    href: String,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Organization")]
    Organization,

    #[serde(rename = "User")]
    User,
}
