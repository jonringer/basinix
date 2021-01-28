// A commit is the commit id
pub type Commit = String;
// E.g. #110430
pub type PullRequestNum = u32;

#[derive(Debug)]
pub enum EvalRequest {
    PullRequestOpened(PullRequestNum),
    PushEvent(PullRequestNum),
}


