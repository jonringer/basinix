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

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "diff_url")]
    diff_url: String,

    #[serde(rename = "patch_url")]
    patch_url: String,

    #[serde(rename = "issue_url")]
    issue_url: String,

    #[serde(rename = "number")]
    number: i64,

    #[serde(rename = "state")]
    state: String,

    #[serde(rename = "locked")]
    locked: bool,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "user")]
    user: User,

    #[serde(rename = "body")]
    body: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "closed_at")]
    closed_at: Option<serde_json::Value>,

    #[serde(rename = "merged_at")]
    merged_at: Option<serde_json::Value>,

    #[serde(rename = "merge_commit_sha")]
    merge_commit_sha: String,

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
    draft: bool,

    #[serde(rename = "commits_url")]
    commits_url: String,

    #[serde(rename = "review_comments_url")]
    review_comments_url: String,

    #[serde(rename = "review_comment_url")]
    review_comment_url: String,

    #[serde(rename = "comments_url")]
    comments_url: String,

    #[serde(rename = "statuses_url")]
    statuses_url: String,

    #[serde(rename = "head")]
    head: Base,

    #[serde(rename = "base")]
    base: Base,

    #[serde(rename = "_links")]
    links: Links,

    #[serde(rename = "author_association")]
    author_association: String,

    #[serde(rename = "auto_merge")]
    auto_merge: Option<serde_json::Value>,

    #[serde(rename = "active_lock_reason")]
    active_lock_reason: Option<serde_json::Value>,

    #[serde(rename = "merged")]
    merged: bool,

    #[serde(rename = "mergeable")]
    mergeable: bool,

    #[serde(rename = "rebaseable")]
    rebaseable: bool,

    #[serde(rename = "mergeable_state")]
    mergeable_state: String,

    #[serde(rename = "merged_by")]
    merged_by: Option<serde_json::Value>,

    #[serde(rename = "comments")]
    comments: i64,

    #[serde(rename = "review_comments")]
    review_comments: i64,

    #[serde(rename = "maintainer_can_modify")]
    maintainer_can_modify: bool,

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
    #[serde(rename = "label")]
    label: String,

    #[serde(rename = "ref")]
    base_ref: String,

    #[serde(rename = "sha")]
    sha: String,

    #[serde(rename = "user")]
    user: User,

    #[serde(rename = "repo")]
    repo: Repo,
}

#[derive(Serialize, Deserialize)]
pub struct Repo {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "full_name")]
    full_name: String,

    #[serde(rename = "private")]
    private: bool,

    #[serde(rename = "owner")]
    owner: User,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "fork")]
    fork: bool,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "forks_url")]
    forks_url: String,

    #[serde(rename = "keys_url")]
    keys_url: String,

    #[serde(rename = "collaborators_url")]
    collaborators_url: String,

    #[serde(rename = "teams_url")]
    teams_url: String,

    #[serde(rename = "hooks_url")]
    hooks_url: String,

    #[serde(rename = "issue_events_url")]
    issue_events_url: String,

    #[serde(rename = "events_url")]
    events_url: String,

    #[serde(rename = "assignees_url")]
    assignees_url: String,

    #[serde(rename = "branches_url")]
    branches_url: String,

    #[serde(rename = "tags_url")]
    tags_url: String,

    #[serde(rename = "blobs_url")]
    blobs_url: String,

    #[serde(rename = "git_tags_url")]
    git_tags_url: String,

    #[serde(rename = "git_refs_url")]
    git_refs_url: String,

    #[serde(rename = "trees_url")]
    trees_url: String,

    #[serde(rename = "statuses_url")]
    statuses_url: String,

    #[serde(rename = "languages_url")]
    languages_url: String,

    #[serde(rename = "stargazers_url")]
    stargazers_url: String,

    #[serde(rename = "contributors_url")]
    contributors_url: String,

    #[serde(rename = "subscribers_url")]
    subscribers_url: String,

    #[serde(rename = "subscription_url")]
    subscription_url: String,

    #[serde(rename = "commits_url")]
    commits_url: String,

    #[serde(rename = "git_commits_url")]
    git_commits_url: String,

    #[serde(rename = "comments_url")]
    comments_url: String,

    #[serde(rename = "issue_comment_url")]
    issue_comment_url: String,

    #[serde(rename = "contents_url")]
    contents_url: String,

    #[serde(rename = "compare_url")]
    compare_url: String,

    #[serde(rename = "merges_url")]
    merges_url: String,

    #[serde(rename = "archive_url")]
    archive_url: String,

    #[serde(rename = "downloads_url")]
    downloads_url: String,

    #[serde(rename = "issues_url")]
    issues_url: String,

    #[serde(rename = "pulls_url")]
    pulls_url: String,

    #[serde(rename = "milestones_url")]
    milestones_url: String,

    #[serde(rename = "notifications_url")]
    notifications_url: String,

    #[serde(rename = "labels_url")]
    labels_url: String,

    #[serde(rename = "releases_url")]
    releases_url: String,

    #[serde(rename = "deployments_url")]
    deployments_url: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "pushed_at")]
    pushed_at: String,

    #[serde(rename = "git_url")]
    git_url: String,

    #[serde(rename = "ssh_url")]
    ssh_url: String,

    #[serde(rename = "clone_url")]
    clone_url: String,

    #[serde(rename = "svn_url")]
    svn_url: String,

    #[serde(rename = "homepage")]
    homepage: String,

    #[serde(rename = "size")]
    size: i64,

    #[serde(rename = "stargazers_count")]
    stargazers_count: i64,

    #[serde(rename = "watchers_count")]
    watchers_count: i64,

    #[serde(rename = "language")]
    language: String,

    #[serde(rename = "has_issues")]
    has_issues: bool,

    #[serde(rename = "has_projects")]
    has_projects: bool,

    #[serde(rename = "has_downloads")]
    has_downloads: bool,

    #[serde(rename = "has_wiki")]
    has_wiki: bool,

    #[serde(rename = "has_pages")]
    has_pages: bool,

    #[serde(rename = "forks_count")]
    forks_count: i64,

    #[serde(rename = "mirror_url")]
    mirror_url: Option<serde_json::Value>,

    #[serde(rename = "archived")]
    archived: bool,

    #[serde(rename = "disabled")]
    disabled: bool,

    #[serde(rename = "open_issues_count")]
    open_issues_count: i64,

    #[serde(rename = "license")]
    license: License,

    #[serde(rename = "forks")]
    forks: i64,

    #[serde(rename = "open_issues")]
    open_issues: i64,

    #[serde(rename = "watchers")]
    watchers: i64,

    #[serde(rename = "default_branch")]
    default_branch: String,
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

    #[serde(rename = "node_id")]
    node_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "login")]
    login: String,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "avatar_url")]
    avatar_url: String,

    #[serde(rename = "gravatar_id")]
    gravatar_id: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "followers_url")]
    followers_url: String,

    #[serde(rename = "following_url")]
    following_url: String,

    #[serde(rename = "gists_url")]
    gists_url: String,

    #[serde(rename = "starred_url")]
    starred_url: String,

    #[serde(rename = "subscriptions_url")]
    subscriptions_url: String,

    #[serde(rename = "organizations_url")]
    organizations_url: String,

    #[serde(rename = "repos_url")]
    repos_url: String,

    #[serde(rename = "events_url")]
    events_url: String,

    #[serde(rename = "received_events_url")]
    received_events_url: String,

    #[serde(rename = "type")]
    user_type: Type,

    #[serde(rename = "site_admin")]
    site_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Label {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "color")]
    color: String,

    #[serde(rename = "default")]
    label_default: bool,

    #[serde(rename = "description")]
    description: Option<String>,
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
