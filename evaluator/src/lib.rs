
use basinix_shared::types::{BuildRequest, GlobalConfig};
use basinix_shared::types::Message::{self, EvalPullRequest, EvalPush, PullRequestClosed};
use log::{debug, error, info};
use std::fs;
use std::process::Command;
use std::sync::mpsc::{Receiver, Sender};

pub mod pull_request;

const LOG_TARGET: &str = "basinix::evaluator";

fn ensure_nixpkgs_exists(config: GlobalConfig) {
    let nixpkgs_path = &config.nixpkgs_dir.as_path();
    let nixpkgs_path_str = nixpkgs_path.to_str().unwrap();

    if !nixpkgs_path.exists() {
        info!("Creating nixpkgs checkout at {}", nixpkgs_path_str);
        fs::create_dir_all(nixpkgs_path).unwrap_or_else(|_| panic!("Unable to create nixpkgs directory at: {}",
            nixpkgs_path_str));

        if !Command::new("git")
            .args(&[
                "clone",
                "git@github.com:NixOS/nixpkgs.git",
                nixpkgs_path_str,
            ])
            .status()
            .unwrap()
            .success()
        {
            error!("Failed to clone nixpkgs repo to {}", nixpkgs_path_str);
            error!("Please ensure that XDG_CACHE_DIR is write-able");
            std::process::exit(1);
        }
    }

    Command::new("git")
        .current_dir(nixpkgs_path_str)
        .args(&[
            "-c",
            "fetch.prune=false",
            "fetch",
            "origin",
        ])
        .status()
        .expect("Unable to fetch remote for nixpkgs");

    // We want to get into detached HEAD, as worktrees may already have a branch or commit checkedout
    Command::new("git")
        .current_dir(nixpkgs_path_str)
        .args(&[
            "checkout",
            "origin/master^",
        ])
        .status()
        .expect("Unable to checkout nixpkgs origin/master");
}

pub fn eval_events(recv: Receiver<Message>, build_sender: Sender<BuildRequest>, config: GlobalConfig) {
    debug!(target: LOG_TARGET, "Ensuring nixpkgs checkout");
    ensure_nixpkgs_exists(config.clone());

    // This is used to check whether or not we need to update the base branch
    // and determine all of the drv paths
    let mut base_revs: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    debug!(target: LOG_TARGET, "Starting evaluator loop");
    loop {
        match recv.recv() {
            Ok(EvalPush(push_info)) => {
                info!(
                    "Evaluating push from {} to {}",
                    &push_info.before, &push_info.head
                );
            }
            Ok(EvalPullRequest(pr)) => {
                info!("Evaluating pull request: {}", pr.number);
                pull_request::eval_pr(&config, build_sender.clone(), pr.number, &mut base_revs)
                    .expect("Failed to evaluate PR");
            }
            Ok(PullRequestClosed(pr)) => {
                info!("Pull request was closed: {}", pr);
            }
            Err(e) => {
                error!("Failed to receive evaluation event: {:?}", e)
            }
        }
    }
}
