use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum NixOSBranch {
    Master,
    Staging,
    StagingNext,
    Other(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NixOSBranchErr(());

impl FromStr for NixOSBranch {
    type Err = NixOSBranchErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "master" => Ok(NixOSBranch::Master),
            "staging" => Ok(NixOSBranch::Staging),
            "staging-next" => Ok(NixOSBranch::StagingNext),
            _ => Ok(NixOSBranch::Other(s.to_owned())),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PushInfo {
    pub push_ref: String,
    pub before: String,
    pub head: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PullRequestInfo {
    pub number: u64,
    pub base_branch: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Message {
    EvalPush(PushInfo),
    EvalPullRequest(PullRequestInfo),
    PullRequestClosed(u64),
}