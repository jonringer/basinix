
// A commit is is the commit id
type Commit = String;
// E.g. #110430
type PullRequestNum = u32;

pub enum EvalRequest {
    PullRequest(PullRequestNum, Vec<Commit>),
}


