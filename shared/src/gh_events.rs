use serde::{Serialize, Deserialize};
use serde;
use serde_json;

pub type Events = Vec<Event>;

#[derive(Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub event_type: EventType,

    #[serde(rename = "actor")]
    pub actor: Actor,

    #[serde(rename = "repo")]
    pub repo: EventRepo,

    #[serde(rename = "payload")]
    pub payload: Payload,

    #[serde(rename = "public")]
    pub public: bool,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "org")]
    pub org: Actor,
}

#[derive(Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "login")]
    login: String,

    #[serde(rename = "display_login")]
    display_login: Option<String>,

    #[serde(rename = "gravatar_id")]
    gravatar_id: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "avatar_url")]
    avatar_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    #[serde(rename = "action")]
    action: Option<Action>,

    #[serde(rename = "issue")]
    issue: Option<Issue>,

    #[serde(rename = "comment")]
    comment: Option<Comment>,

    #[serde(rename = "push_id")]
    push_id: Option<i64>,

    #[serde(rename = "size")]
    size: Option<i64>,

    #[serde(rename = "distinct_size")]
    distinct_size: Option<i64>,

    #[serde(rename = "ref")]
    payload_ref: Option<String>,

    #[serde(rename = "head")]
    head: Option<String>,

    #[serde(rename = "before")]
    before: Option<String>,

    #[serde(rename = "commits")]
    commits: Option<Vec<Commit>>,

    #[serde(rename = "pull_request")]
    pull_request: Option<PayloadPullRequest>,

    #[serde(rename = "review")]
    review: Option<Review>,

    #[serde(rename = "number")]
    number: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "issue_url")]
    issue_url: Option<String>,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "user")]
    user: User,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "author_association")]
    author_association: AuthorAssociation,

    #[serde(rename = "body")]
    body: String,

    #[serde(rename = "performed_via_github_app")]
    performed_via_github_app: Option<serde_json::Value>,

    #[serde(rename = "pull_request_review_id")]
    pull_request_review_id: Option<i64>,

    #[serde(rename = "diff_hunk")]
    diff_hunk: Option<String>,

    #[serde(rename = "path")]
    path: Option<String>,

    #[serde(rename = "position")]
    position: Option<i64>,

    #[serde(rename = "original_position")]
    original_position: Option<i64>,

    #[serde(rename = "commit_id")]
    commit_id: Option<String>,

    #[serde(rename = "original_commit_id")]
    original_commit_id: Option<String>,

    #[serde(rename = "pull_request_url")]
    pull_request_url: Option<String>,

    #[serde(rename = "_links")]
    links: Option<CommentLinks>,

    #[serde(rename = "start_line")]
    start_line: Option<serde_json::Value>,

    #[serde(rename = "original_start_line")]
    original_start_line: Option<i64>,

    #[serde(rename = "start_side")]
    start_side: Option<Side>,

    #[serde(rename = "line")]
    line: Option<i64>,

    #[serde(rename = "original_line")]
    original_line: Option<i64>,

    #[serde(rename = "side")]
    side: Option<Side>,

    #[serde(rename = "in_reply_to_id")]
    in_reply_to_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct CommentLinks {
    #[serde(rename = "self")]
    links_self: Comments,

    #[serde(rename = "html")]
    html: Comments,

    #[serde(rename = "pull_request")]
    pull_request: Comments,
}

#[derive(Serialize, Deserialize)]
pub struct Comments {
    #[serde(rename = "href")]
    href: String,
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
    user_type: UserType,

    #[serde(rename = "site_admin")]
    site_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Commit {
    #[serde(rename = "sha")]
    sha: String,

    #[serde(rename = "author")]
    author: Author,

    #[serde(rename = "message")]
    message: Option<String>,

    #[serde(rename = "distinct")]
    distinct: bool,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    #[serde(rename = "email")]
    email: String,

    #[serde(rename = "name")]
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Issue {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "repository_url")]
    repository_url: String,

    #[serde(rename = "labels_url")]
    labels_url: String,

    #[serde(rename = "comments_url")]
    comments_url: String,

    #[serde(rename = "events_url")]
    events_url: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "number")]
    number: i64,

    #[serde(rename = "title")]
    title: String,

    #[serde(rename = "user")]
    user: User,

    #[serde(rename = "labels")]
    labels: Vec<LabelElement>,

    #[serde(rename = "state")]
    state: IssueState,

    #[serde(rename = "locked")]
    locked: bool,

    #[serde(rename = "assignee")]
    assignee: Option<User>,

    #[serde(rename = "assignees")]
    assignees: Vec<User>,

    #[serde(rename = "milestone")]
    milestone: Option<serde_json::Value>,

    #[serde(rename = "comments")]
    comments: i64,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "closed_at")]
    closed_at: Option<String>,

    #[serde(rename = "author_association")]
    author_association: AuthorAssociation,

    #[serde(rename = "active_lock_reason")]
    active_lock_reason: Option<serde_json::Value>,

    #[serde(rename = "pull_request")]
    pull_request: Option<IssuePullRequest>,

    #[serde(rename = "body")]
    body: String,

    #[serde(rename = "performed_via_github_app")]
    performed_via_github_app: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct LabelElement {
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
pub struct IssuePullRequest {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "diff_url")]
    diff_url: String,

    #[serde(rename = "patch_url")]
    patch_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct PayloadPullRequest {
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
    state: IssueState,

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
    closed_at: Option<String>,

    #[serde(rename = "merged_at")]
    merged_at: Option<String>,

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
    labels: Vec<LabelElement>,

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
    head: Head,

    #[serde(rename = "base")]
    base: Base,

    #[serde(rename = "_links")]
    links: PullRequestLinks,

    #[serde(rename = "author_association")]
    author_association: AuthorAssociation,

    #[serde(rename = "active_lock_reason")]
    active_lock_reason: Option<serde_json::Value>,

    #[serde(rename = "merged")]
    merged: Option<bool>,

    #[serde(rename = "mergeable")]
    mergeable: Option<serde_json::Value>,

    #[serde(rename = "rebaseable")]
    rebaseable: Option<serde_json::Value>,

    #[serde(rename = "mergeable_state")]
    mergeable_state: Option<String>,

    #[serde(rename = "merged_by")]
    merged_by: Option<User>,

    #[serde(rename = "comments")]
    comments: Option<i64>,

    #[serde(rename = "review_comments")]
    review_comments: Option<i64>,

    #[serde(rename = "maintainer_can_modify")]
    maintainer_can_modify: Option<bool>,

    #[serde(rename = "commits")]
    commits: Option<i64>,

    #[serde(rename = "additions")]
    additions: Option<i64>,

    #[serde(rename = "deletions")]
    deletions: Option<i64>,

    #[serde(rename = "changed_files")]
    changed_files: Option<i64>,
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
    repo: BaseRepo,
}

#[derive(Serialize, Deserialize)]
pub struct BaseRepo {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "name")]
    name: PurpleName,

    #[serde(rename = "full_name")]
    full_name: String,

    #[serde(rename = "private")]
    private: bool,

    #[serde(rename = "owner")]
    owner: User,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "description")]
    description: Option<String>,

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
    homepage: Option<String>,

    #[serde(rename = "size")]
    size: i64,

    #[serde(rename = "stargazers_count")]
    stargazers_count: i64,

    #[serde(rename = "watchers_count")]
    watchers_count: i64,

    #[serde(rename = "language")]
    language: Option<String>,

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
    license: Option<License>,

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
    spdx_id: SpdxId,

    #[serde(rename = "url")]
    url: Option<String>,

    #[serde(rename = "node_id")]
    node_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Head {
    #[serde(rename = "label")]
    label: String,

    #[serde(rename = "ref")]
    head_ref: String,

    #[serde(rename = "sha")]
    sha: String,

    #[serde(rename = "user")]
    user: User,

    #[serde(rename = "repo")]
    repo: BaseRepo,
}

#[derive(Serialize, Deserialize)]
pub struct PullRequestLinks {
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
pub struct Review {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "user")]
    user: User,

    #[serde(rename = "body")]
    body: Option<serde_json::Value>,

    #[serde(rename = "commit_id")]
    commit_id: String,

    #[serde(rename = "submitted_at")]
    submitted_at: String,

    #[serde(rename = "state")]
    state: ReviewState,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "pull_request_url")]
    pull_request_url: String,

    #[serde(rename = "author_association")]
    author_association: AuthorAssociation,

    #[serde(rename = "_links")]
    links: ReviewLinks,
}

#[derive(Serialize, Deserialize)]
pub struct ReviewLinks {
    #[serde(rename = "html")]
    html: Comments,

    #[serde(rename = "pull_request")]
    pull_request: Comments,
}

#[derive(Serialize, Deserialize)]
pub struct EventRepo {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "url")]
    url: String,
}

// https://docs.github.com/en/developers/webhooks-and-events/github-event-types
#[derive(Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "CommitCommentEvent")]
    CommitCommentEvent,

    #[serde(rename = "CreateEvent")]
    CreateEvent,

    #[serde(rename = "DeleteEvent")]
    DeleteEvent,

    #[serde(rename = "ForkEvent")]
    ForkEvent,

    #[serde(rename = "GollumEvent")]
    GollumEvent,

    #[serde(rename = "IssueCommentEvent")]
    IssueCommentEvent,

    #[serde(rename = "IssuesEvent")]
    IssuesEvent,

    #[serde(rename = "MemberEvent")]
    MemberEvent,

    #[serde(rename = "PushEvent")]
    PushEvent,

    #[serde(rename = "PullRequestEvent")]
    PullRequestEvent,

    #[serde(rename = "PullRequestReviewCommentEvent")]
    PullRequestReviewCommentEvent,

    #[serde(rename = "PullRequestReviewEvent")]
    PullRequestReviewEvent,

    #[serde(rename = "ReleaseEvent")]
    ReleaseEvent,

    #[serde(rename = "SponsorshipEvent")]
    SponsorshipEvent,

    #[serde(rename = "WatchEvent")]
    WatchEvent,
}

#[derive(Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "closed")]
    Closed,

    #[serde(rename = "created")]
    Created,

    #[serde(rename = "opened")]
    Opened,

    #[serde(rename = "started")]
    Started,
}

#[derive(Serialize, Deserialize)]
pub enum AuthorAssociation {
    #[serde(rename = "CONTRIBUTOR")]
    Contributor,

    #[serde(rename = "MEMBER")]
    Member,

    #[serde(rename = "NONE")]
    None,
}

#[derive(Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "LEFT")]
    Left,

    #[serde(rename = "RIGHT")]
    Right,
}

#[derive(Serialize, Deserialize)]
pub enum UserType {
    #[serde(rename = "Bot")]
    Bot,

    #[serde(rename = "Organization")]
    Organization,

    #[serde(rename = "User")]
    User,
}

#[derive(Serialize, Deserialize)]
pub enum IssueState {
    #[serde(rename = "closed")]
    Closed,

    #[serde(rename = "open")]
    Open,
}

#[derive(Serialize, Deserialize)]
pub enum LicenseNodeId {
    #[serde(rename = "MDc6TGljZW5zZTEz")]
    MDc6TGljZw5ZZtEz,

    #[serde(rename = "MDc6TGljZW5zZTA=")]
    MDc6TGljZw5ZZta,
}

#[derive(Serialize, Deserialize)]
pub enum SpdxId {
    #[serde(rename = "MIT")]
    Mit,

    #[serde(rename = "NOASSERTION")]
    Noassertion,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleName {
    #[serde(rename = "nixpkgs")]
    Nixpkgs,
}

#[derive(Serialize, Deserialize)]
pub enum ReviewState {
    #[serde(rename = "approved")]
    Approved,

    #[serde(rename = "commented")]
    Commented,

    #[serde(rename = "changes_requested")]
    ChangesRequested,
}
