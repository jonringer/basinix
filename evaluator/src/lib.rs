use basinix_shared::types::Message::{self, EvalPush, EvalPullRequest, PullRequestClosed};
use log::{debug, error, info};
use std::sync::mpsc::Receiver;

const LOG_TARGET: &str = "basinix::evaluator";

pub fn eval_events(recv: Receiver<Message>) {

    debug!(target: LOG_TARGET, "Starting evaluator loop");
    loop {
        match recv.recv() {
            Ok(EvalPush(push_info)) => {
                info!("Evaluating push from {} to {}", &push_info.before, &push_info.head);
            },
            Ok(EvalPullRequest(pr)) => {
                info!("Evaluating pull request: {}", pr.number);
            },
            Ok(PullRequestClosed(pr)) => {
                info!("Pull request was closed: {}", pr);
            },
            Err(e) => {
                error!("Failed to receive evaluation event: {:?}", e)
            }
        }
    }
}