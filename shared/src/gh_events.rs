use serde::{Serialize, Deserialize};
use serde;
use serde_json;

type Events = Vec<Event>;

#[derive(Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "type")]
    event_type: EventType,

    #[serde(rename = "actor")]
    actor: Actor,

    #[serde(rename = "repo")]
    repo: EventRepo,

    #[serde(rename = "payload")]
    payload: Payload,

    #[serde(rename = "public")]
    public: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "org")]
    org: Actor,
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
    path: Option<Path>,

    #[serde(rename = "position")]
    position: Option<i64>,

    #[serde(rename = "original_position")]
    original_position: Option<i64>,

    #[serde(rename = "commit_id")]
    commit_id: Option<String>,

    #[serde(rename = "original_commit_id")]
    original_commit_id: Option<OriginalCommitId>,

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
    message: String,

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
    color: Color,

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
    label: LabelEnum,

    #[serde(rename = "ref")]
    base_ref: Ref,

    #[serde(rename = "sha")]
    sha: Sha,

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
    node_id: RepoNodeId,

    #[serde(rename = "name")]
    name: PurpleName,

    #[serde(rename = "full_name")]
    full_name: FullNameEnum,

    #[serde(rename = "private")]
    private: bool,

    #[serde(rename = "owner")]
    owner: User,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "description")]
    description: Description,

    #[serde(rename = "fork")]
    fork: bool,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "forks_url")]
    forks_url: String,

    #[serde(rename = "keys_url")]
    keys_url: KeysUrl,

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
    blobs_url: BlobsUrl,

    #[serde(rename = "git_tags_url")]
    git_tags_url: GitTagsUrl,

    #[serde(rename = "git_refs_url")]
    git_refs_url: GitRefsUrl,

    #[serde(rename = "trees_url")]
    trees_url: TreesUrl,

    #[serde(rename = "statuses_url")]
    statuses_url: StatusesUrl,

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
    commits_url: CommitsUrl,

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
    issues_url: IssuesUrl,

    #[serde(rename = "pulls_url")]
    pulls_url: PullsUrl,

    #[serde(rename = "milestones_url")]
    milestones_url: String,

    #[serde(rename = "notifications_url")]
    notifications_url: String,

    #[serde(rename = "labels_url")]
    labels_url: LabelsUrl,

    #[serde(rename = "releases_url")]
    releases_url: ReleasesUrl,

    #[serde(rename = "deployments_url")]
    deployments_url: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "pushed_at")]
    pushed_at: String,

    #[serde(rename = "git_url")]
    git_url: GitUrl,

    #[serde(rename = "ssh_url")]
    ssh_url: SshUrl,

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
    language: Language,

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
    default_branch: Ref,
}

#[derive(Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "key")]
    key: Key,

    #[serde(rename = "name")]
    name: LicenseName,

    #[serde(rename = "spdx_id")]
    spdx_id: SpdxId,

    #[serde(rename = "url")]
    url: Option<String>,

    #[serde(rename = "node_id")]
    node_id: LicenseNodeId,
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
    name: FullNameEnum,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "IssueCommentEvent")]
    IssueCommentEvent,

    #[serde(rename = "PullRequestEvent")]
    PullRequestEvent,

    #[serde(rename = "PullRequestReviewCommentEvent")]
    PullRequestReviewCommentEvent,

    #[serde(rename = "PullRequestReviewEvent")]
    PullRequestReviewEvent,

    #[serde(rename = "PushEvent")]
    PushEvent,
}

#[derive(Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "closed")]
    Closed,

    #[serde(rename = "created")]
    Created,

    #[serde(rename = "opened")]
    Opened,
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
pub enum OriginalCommitId {
    #[serde(rename = "af19299b6545d973f957e724450052214163f060")]
    Af19299B6545D973F957E724450052214163F060,

    #[serde(rename = "cda6ed115d23f6518211306157a0366c49de00e8")]
    Cda6Ed115D23F6518211306157A0366C49De00E8,

    #[serde(rename = "86106f042b5e30ebb816febaa2a03998ca31e207")]
    The86106F042B5E30Ebb816Febaa2A03998Ca31E207,
}

#[derive(Serialize, Deserialize)]
pub enum Path {
    #[serde(rename = "nixos/modules/system/boot/stage-1.nix")]
    NixosModulesSystemBootStage1Nix,

    #[serde(rename = "pkgs/development/tools/hindsight/default.nix")]
    PkgsDevelopmentToolsHindsightDefaultNix,

    #[serde(rename = "pkgs/servers/code-server/default.nix")]
    PkgsServersCodeServerDefaultNix,
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
pub enum Color {
    #[serde(rename = "b60205")]
    B60205,

    #[serde(rename = "ddeecc")]
    Ddeecc,

    #[serde(rename = "e6e6e6")]
    E6E6E6,

    #[serde(rename = "ededed")]
    Ededed,

    #[serde(rename = "eeffee")]
    Eeffee,

    #[serde(rename = "fbca04")]
    Fbca04,

    #[serde(rename = "fef2c0")]
    Fef2C0,

    #[serde(rename = "009800")]
    The009800,

    #[serde(rename = "84b6eb")]
    The84B6Eb,
}

#[derive(Serialize, Deserialize)]
pub enum IssueState {
    #[serde(rename = "closed")]
    Closed,

    #[serde(rename = "open")]
    Open,
}

#[derive(Serialize, Deserialize)]
pub enum Ref {
    #[serde(rename = "master")]
    Master,
}

#[derive(Serialize, Deserialize)]
pub enum LabelEnum {
    #[serde(rename = "NixOS:master")]
    NixOsMaster,
}

#[derive(Serialize, Deserialize)]
pub enum BlobsUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/git/blobs{/sha}")]
    HttpsApiGithubComReposChknoNixpkgsGitBlobsSha,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/git/blobs{/sha}")]
    HttpsApiGithubComReposDelrothNixpkgsGitBlobsSha,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/git/blobs{/sha}")]
    HttpsApiGithubComReposDguentherNixpkgsGitBlobsSha,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/git/blobs{/sha}")]
    HttpsApiGithubComReposGovanifYNixpkgsGitBlobsSha,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/git/blobs{/sha}")]
    HttpsApiGithubComReposNixOsNixpkgsGitBlobsSha,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/git/blobs{/sha}")]
    HttpsApiGithubComReposProfpatschNixpkgsGitBlobsSha,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/git/blobs{/sha}")]
    HttpsApiGithubComReposRyneeverettNixpkgsGitBlobsSha,
}

#[derive(Serialize, Deserialize)]
pub enum CommitsUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/commits{/sha}")]
    HttpsApiGithubComReposChknoNixpkgsCommitsSha,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/commits{/sha}")]
    HttpsApiGithubComReposDelrothNixpkgsCommitsSha,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/commits{/sha}")]
    HttpsApiGithubComReposDguentherNixpkgsCommitsSha,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/commits{/sha}")]
    HttpsApiGithubComReposGovanifYNixpkgsCommitsSha,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/commits{/sha}")]
    HttpsApiGithubComReposNixOsNixpkgsCommitsSha,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/commits{/sha}")]
    HttpsApiGithubComReposProfpatschNixpkgsCommitsSha,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/commits{/sha}")]
    HttpsApiGithubComReposRyneeverettNixpkgsCommitsSha,
}

#[derive(Serialize, Deserialize)]
pub enum Description {
    #[serde(rename = "Nix Packages collection")]
    NixPackagesCollection,
}

#[derive(Serialize, Deserialize)]
pub enum FullNameEnum {
    #[serde(rename = "chkno/nixpkgs")]
    ChknoNixpkgs,

    #[serde(rename = "delroth/nixpkgs")]
    DelrothNixpkgs,

    #[serde(rename = "dguenther/nixpkgs")]
    DguentherNixpkgs,

    #[serde(rename = "GovanifY/nixpkgs")]
    GovanifYNixpkgs,

    #[serde(rename = "NixOS/nixpkgs")]
    NixOsNixpkgs,

    #[serde(rename = "Profpatsch/nixpkgs")]
    ProfpatschNixpkgs,

    #[serde(rename = "ryneeverett/nixpkgs")]
    RyneeverettNixpkgs,
}

#[derive(Serialize, Deserialize)]
pub enum GitRefsUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/git/refs{/sha}")]
    HttpsApiGithubComReposChknoNixpkgsGitRefsSha,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/git/refs{/sha}")]
    HttpsApiGithubComReposDelrothNixpkgsGitRefsSha,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/git/refs{/sha}")]
    HttpsApiGithubComReposDguentherNixpkgsGitRefsSha,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/git/refs{/sha}")]
    HttpsApiGithubComReposGovanifYNixpkgsGitRefsSha,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/git/refs{/sha}")]
    HttpsApiGithubComReposNixOsNixpkgsGitRefsSha,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/git/refs{/sha}")]
    HttpsApiGithubComReposProfpatschNixpkgsGitRefsSha,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/git/refs{/sha}")]
    HttpsApiGithubComReposRyneeverettNixpkgsGitRefsSha,
}

#[derive(Serialize, Deserialize)]
pub enum GitTagsUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/git/tags{/sha}")]
    HttpsApiGithubComReposChknoNixpkgsGitTagsSha,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/git/tags{/sha}")]
    HttpsApiGithubComReposDelrothNixpkgsGitTagsSha,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/git/tags{/sha}")]
    HttpsApiGithubComReposDguentherNixpkgsGitTagsSha,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/git/tags{/sha}")]
    HttpsApiGithubComReposGovanifYNixpkgsGitTagsSha,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/git/tags{/sha}")]
    HttpsApiGithubComReposNixOsNixpkgsGitTagsSha,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/git/tags{/sha}")]
    HttpsApiGithubComReposProfpatschNixpkgsGitTagsSha,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/git/tags{/sha}")]
    HttpsApiGithubComReposRyneeverettNixpkgsGitTagsSha,
}

#[derive(Serialize, Deserialize)]
pub enum GitUrl {
    #[serde(rename = "git://github.com/chkno/nixpkgs.git")]
    GitGithubComChknoNixpkgsGit,

    #[serde(rename = "git://github.com/delroth/nixpkgs.git")]
    GitGithubComDelrothNixpkgsGit,

    #[serde(rename = "git://github.com/dguenther/nixpkgs.git")]
    GitGithubComDguentherNixpkgsGit,

    #[serde(rename = "git://github.com/GovanifY/nixpkgs.git")]
    GitGithubComGovanifYNixpkgsGit,

    #[serde(rename = "git://github.com/NixOS/nixpkgs.git")]
    GitGithubComNixOsNixpkgsGit,

    #[serde(rename = "git://github.com/Profpatsch/nixpkgs.git")]
    GitGithubComProfpatschNixpkgsGit,

    #[serde(rename = "git://github.com/ryneeverett/nixpkgs.git")]
    GitGithubComRyneeverettNixpkgsGit,
}

#[derive(Serialize, Deserialize)]
pub enum IssuesUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/issues{/number}")]
    HttpsApiGithubComReposChknoNixpkgsIssuesNumber,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/issues{/number}")]
    HttpsApiGithubComReposDelrothNixpkgsIssuesNumber,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/issues{/number}")]
    HttpsApiGithubComReposDguentherNixpkgsIssuesNumber,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/issues{/number}")]
    HttpsApiGithubComReposGovanifYNixpkgsIssuesNumber,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/issues{/number}")]
    HttpsApiGithubComReposNixOsNixpkgsIssuesNumber,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/issues{/number}")]
    HttpsApiGithubComReposProfpatschNixpkgsIssuesNumber,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/issues{/number}")]
    HttpsApiGithubComReposRyneeverettNixpkgsIssuesNumber,
}

#[derive(Serialize, Deserialize)]
pub enum KeysUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/keys{/key_id}")]
    HttpsApiGithubComReposChknoNixpkgsKeysKeyId,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/keys{/key_id}")]
    HttpsApiGithubComReposDelrothNixpkgsKeysKeyId,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/keys{/key_id}")]
    HttpsApiGithubComReposDguentherNixpkgsKeysKeyId,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/keys{/key_id}")]
    HttpsApiGithubComReposGovanifYNixpkgsKeysKeyId,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/keys{/key_id}")]
    HttpsApiGithubComReposNixOsNixpkgsKeysKeyId,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/keys{/key_id}")]
    HttpsApiGithubComReposProfpatschNixpkgsKeysKeyId,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/keys{/key_id}")]
    HttpsApiGithubComReposRyneeverettNixpkgsKeysKeyId,
}

#[derive(Serialize, Deserialize)]
pub enum LabelsUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/labels{/name}")]
    HttpsApiGithubComReposChknoNixpkgsLabelsName,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/labels{/name}")]
    HttpsApiGithubComReposDelrothNixpkgsLabelsName,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/labels{/name}")]
    HttpsApiGithubComReposDguentherNixpkgsLabelsName,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/labels{/name}")]
    HttpsApiGithubComReposGovanifYNixpkgsLabelsName,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/labels{/name}")]
    HttpsApiGithubComReposNixOsNixpkgsLabelsName,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/labels{/name}")]
    HttpsApiGithubComReposProfpatschNixpkgsLabelsName,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/labels{/name}")]
    HttpsApiGithubComReposRyneeverettNixpkgsLabelsName,
}

#[derive(Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "Nix")]
    Nix,
}

#[derive(Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "mit")]
    Mit,

    #[serde(rename = "other")]
    Other,
}

#[derive(Serialize, Deserialize)]
pub enum LicenseName {
    #[serde(rename = "MIT License")]
    MitLicense,

    #[serde(rename = "Other")]
    Other,
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
pub enum RepoNodeId {
    #[serde(rename = "MDEwOlJlcG9zaXRvcnk0NTQyNzE2")]
    MdEwOlJlcG9ZaXRvcnk0NtQyNzE2,

    #[serde(rename = "MDEwOlJlcG9zaXRvcnk1NzgwMTM0OA==")]
    MdEwOlJlcG9ZaXRvcnk1NzgwMtm0Oa,

    #[serde(rename = "MDEwOlJlcG9zaXRvcnkxNTI4NTAxMzI=")]
    MdEwOlJlcG9ZaXRvcnkxNti4NtAxMzI,

    #[serde(rename = "MDEwOlJlcG9zaXRvcnkxOTk3NDU4MTA=")]
    MdEwOlJlcG9ZaXRvcnkxOTk3Ndu4Mta,

    #[serde(rename = "MDEwOlJlcG9zaXRvcnkyMzY2NTQ3NTU=")]
    MdEwOlJlcG9ZaXRvcnkyMzY2Ntq3Ntu,

    #[serde(rename = "MDEwOlJlcG9zaXRvcnkyOTA5ODc0Ng==")]
    MdEwOlJlcG9ZaXRvcnkyOta5ODc0Ng,

    #[serde(rename = "MDEwOlJlcG9zaXRvcnkzMDQ1MDI1MDM=")]
    MdEwOlJlcG9ZaXRvcnkzMdq1Mdi1Mdm,
}

#[derive(Serialize, Deserialize)]
pub enum PullsUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/pulls{/number}")]
    HttpsApiGithubComReposChknoNixpkgsPullsNumber,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/pulls{/number}")]
    HttpsApiGithubComReposDelrothNixpkgsPullsNumber,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/pulls{/number}")]
    HttpsApiGithubComReposDguentherNixpkgsPullsNumber,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/pulls{/number}")]
    HttpsApiGithubComReposGovanifYNixpkgsPullsNumber,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/pulls{/number}")]
    HttpsApiGithubComReposNixOsNixpkgsPullsNumber,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/pulls{/number}")]
    HttpsApiGithubComReposProfpatschNixpkgsPullsNumber,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/pulls{/number}")]
    HttpsApiGithubComReposRyneeverettNixpkgsPullsNumber,
}

#[derive(Serialize, Deserialize)]
pub enum ReleasesUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/releases{/id}")]
    HttpsApiGithubComReposChknoNixpkgsReleasesId,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/releases{/id}")]
    HttpsApiGithubComReposDelrothNixpkgsReleasesId,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/releases{/id}")]
    HttpsApiGithubComReposDguentherNixpkgsReleasesId,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/releases{/id}")]
    HttpsApiGithubComReposGovanifYNixpkgsReleasesId,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/releases{/id}")]
    HttpsApiGithubComReposNixOsNixpkgsReleasesId,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/releases{/id}")]
    HttpsApiGithubComReposProfpatschNixpkgsReleasesId,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/releases{/id}")]
    HttpsApiGithubComReposRyneeverettNixpkgsReleasesId,
}

#[derive(Serialize, Deserialize)]
pub enum SshUrl {
    #[serde(rename = "git@github.com:chkno/nixpkgs.git")]
    GitGithubComChknoNixpkgsGit,

    #[serde(rename = "git@github.com:delroth/nixpkgs.git")]
    GitGithubComDelrothNixpkgsGit,

    #[serde(rename = "git@github.com:dguenther/nixpkgs.git")]
    GitGithubComDguentherNixpkgsGit,

    #[serde(rename = "git@github.com:GovanifY/nixpkgs.git")]
    GitGithubComGovanifYNixpkgsGit,

    #[serde(rename = "git@github.com:NixOS/nixpkgs.git")]
    GitGithubComNixOsNixpkgsGit,

    #[serde(rename = "git@github.com:Profpatsch/nixpkgs.git")]
    GitGithubComProfpatschNixpkgsGit,

    #[serde(rename = "git@github.com:ryneeverett/nixpkgs.git")]
    GitGithubComRyneeverettNixpkgsGit,
}

#[derive(Serialize, Deserialize)]
pub enum StatusesUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/statuses/{sha}")]
    HttpsApiGithubComReposChknoNixpkgsStatusesSha,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/statuses/{sha}")]
    HttpsApiGithubComReposDelrothNixpkgsStatusesSha,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/statuses/{sha}")]
    HttpsApiGithubComReposDguentherNixpkgsStatusesSha,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/statuses/{sha}")]
    HttpsApiGithubComReposGovanifYNixpkgsStatusesSha,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/statuses/{sha}")]
    HttpsApiGithubComReposNixOsNixpkgsStatusesSha,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/statuses/{sha}")]
    HttpsApiGithubComReposProfpatschNixpkgsStatusesSha,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/statuses/{sha}")]
    HttpsApiGithubComReposRyneeverettNixpkgsStatusesSha,
}

#[derive(Serialize, Deserialize)]
pub enum TreesUrl {
    #[serde(rename = "https://api.github.com/repos/chkno/nixpkgs/git/trees{/sha}")]
    HttpsApiGithubComReposChknoNixpkgsGitTreesSha,

    #[serde(rename = "https://api.github.com/repos/delroth/nixpkgs/git/trees{/sha}")]
    HttpsApiGithubComReposDelrothNixpkgsGitTreesSha,

    #[serde(rename = "https://api.github.com/repos/dguenther/nixpkgs/git/trees{/sha}")]
    HttpsApiGithubComReposDguentherNixpkgsGitTreesSha,

    #[serde(rename = "https://api.github.com/repos/GovanifY/nixpkgs/git/trees{/sha}")]
    HttpsApiGithubComReposGovanifYNixpkgsGitTreesSha,

    #[serde(rename = "https://api.github.com/repos/NixOS/nixpkgs/git/trees{/sha}")]
    HttpsApiGithubComReposNixOsNixpkgsGitTreesSha,

    #[serde(rename = "https://api.github.com/repos/Profpatsch/nixpkgs/git/trees{/sha}")]
    HttpsApiGithubComReposProfpatschNixpkgsGitTreesSha,

    #[serde(rename = "https://api.github.com/repos/ryneeverett/nixpkgs/git/trees{/sha}")]
    HttpsApiGithubComReposRyneeverettNixpkgsGitTreesSha,
}

#[derive(Serialize, Deserialize)]
pub enum Sha {
    #[serde(rename = "a5969bcd238cf8584512bb1e354172f75da375ca")]
    A5969Bcd238Cf8584512Bb1E354172F75Da375Ca,

    #[serde(rename = "c58219fff6890b2a2cb801cdfef29d4f6729b9ef")]
    C58219Fff6890B2A2Cb801Cdfef29D4F6729B9Ef,

    #[serde(rename = "2a9f33600cd8aa3c72fdcdbcc74bdd8096b81ab3")]
    The2A9F33600Cd8Aa3C72Fdcdbcc74Bdd8096B81Ab3,

    #[serde(rename = "4de9a335f970fd3652ece992cd981559761d98a1")]
    The4De9A335F970Fd3652Ece992Cd981559761D98A1,
}

#[derive(Serialize, Deserialize)]
pub enum ReviewState {
    #[serde(rename = "commented")]
    Commented,
}
