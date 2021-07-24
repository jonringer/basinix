use basinix_shared::eval_events::EvalRequest;
use basinix_shared::github::PullRequest;
use basinix_shared::types::GlobalConfig;
use chrono::Local;
use log::{debug, error, info};
use reqwest::blocking::{Client, Request, RequestBuilder, Response};
use std::collections::{HashSet, HashMap};
use std::process::Command;
use std::process::ExitStatus;
use core::future::Future;
use std::path::{Path, PathBuf};

const LOG_TARGET: &str = "basinix::evaluator::pull_request";

// Fetches upstream, then creates a worktree for the revision.
// Returns the sha of HEAD
pub fn create_base_worktree_if_missing(nixpkgs_dir: &Path, base_ref: &str) -> String {
    // need to fetch the branch which has the commits on it
    Command::new("git")
        .current_dir(nixpkgs_dir)
        .args(&[
            "-c",
            "fetch.prune=false",
            "fetch",
            "--force",
            "origin",
            base_ref
        ])
        .status()
        .expect("Unable to fetch base ref for nixpkgs"); 
    
    let base_rev: String = std::string::String::from_utf8(Command::new("git")
        .current_dir(nixpkgs_dir)
        .args(&[
            "rev-parse",
            &format!("origin/{}", base_ref)
        ])
        .output()
        .unwrap()
        .stdout)
        .expect("Unable read from rev-parse")
        .trim()
        .to_string();

    let mut worktree_dest_dir =  nixpkgs_dir.parent().unwrap().to_path_buf();
    worktree_dest_dir.push(&base_rev);

    create_worktree_if_missing(nixpkgs_dir, worktree_dest_dir.as_path(), &base_rev);

    base_rev
}

pub fn fetch_pr(nixpkgs_dir: &Path, pr_number: u64) {
    // need to fetch the branch which has the commits on it
    Command::new("git")
        .current_dir(nixpkgs_dir)
        .args(&[
            "-c",
            "fetch.prune=false",
            "fetch",
            "--force",
            "https://github.com/NixOS/nixpkgs",
            &format!("pull/{}/head", pr_number)
        ])
        .status()
        .expect(&format!("Unable to fetch PR #{} for nixpkgs", pr_number)); 
}

pub fn create_worktree_if_missing(nixpkgs_dir: &Path, worktree_dest_dir: &Path, rev: &str) {
    // checkout branch on a worktree, will be reused to build the derivations
    info!(target: LOG_TARGET, "Creating worktree at {}", worktree_dest_dir.to_str().unwrap());
    if !worktree_dest_dir.exists() {
        Command::new("git")
            .current_dir(nixpkgs_dir)
            .args(&[
                "worktree",
                "add",
                &worktree_dest_dir.to_str().unwrap(),
                rev 
            ])
            .status()
            .expect("Unable to create worktree");
    } else {
        info!(target: LOG_TARGET, "Skipping creating a worktree for {}, already exists", worktree_dest_dir.to_str().unwrap());
    }
}

pub fn eval_pr(config: &GlobalConfig, pr_number: u64, base_revs: &mut std::collections::HashMap<String, String>) {

    let pr_info = get_pr_response(pr_number)
        .and_then(|body|
            Ok(serde_json::from_str::<PullRequest>(&body).expect("Unable to serialize github pr response")
        )).unwrap();

    // This is less awkward than using the PathBuf `push` paradigm
    let base_path = format!("{}/{}", config.worktree_dir.to_str().unwrap(), &pr_info.base.base_ref);
    let base_worktree_dir = std::path::Path::new(&base_path);
    let base_drv_outputs_str = format!("{}/{}", &base_path, "outputs.txt");
    let base_drv_outputs = std::path::Path::new(&base_drv_outputs_str);

    let head_path = format!("{}/{}", config.worktree_dir.to_str().unwrap(), &pr_info.head.sha);
    let head_worktree_dir = Path::new(&head_path);
    let head_drv_outputs_str = format!("{}/{}", &head_path, "outputs.txt");
    let head_drv_outputs = Path::new(&head_drv_outputs_str);
    let head_new_drv_outputs_str = format!("{}/{}", &head_path, "new_drvs.txt");
    let head_new_drv_outputs = Path::new(&head_new_drv_outputs_str);

    // create worktree for base branch if missing
    // Check if we need to do any work
    // TODO: make sha check less ugly
    if &pr_info.base.sha.as_str() == &base_revs.get(&pr_info.base.base_ref).unwrap_or(&"0".to_string()) {
        info!(target: LOG_TARGET, "Skipping checkout of {} branch, sha is the same", &pr_info.base.base_ref);
    } else {
        let base_rev = create_base_worktree_if_missing(config.nixpkgs_dir.as_path(), &pr_info.base.base_ref);
        base_revs.insert(pr_info.base.base_ref.to_string(), base_rev.to_string());
    }

    // create worktree for head branch if missing
    fetch_pr(&config.nixpkgs_dir.as_path(), pr_number);
    create_worktree_if_missing(
        config.nixpkgs_dir.as_path(),
        &head_worktree_dir,
        &pr_info.head.sha);

    // query outpaths
    if !head_drv_outputs.exists() {
        query_outpaths(config, &head_worktree_dir, &head_drv_outputs);
    }

    if !base_drv_outputs.exists() {
        query_outpaths(config, &base_worktree_dir, &base_drv_outputs);
    }

    // create new derivations file
    if !head_new_drv_outputs.exists() {
        let output_file = std::fs::File::create(&head_new_drv_outputs).expect(
            &format!("Unable to write to {}", &head_new_drv_outputs_str)
        );
        Command::new("comm")
            .args(&[
                "-13",
                base_drv_outputs.to_str().unwrap(),
                head_drv_outputs.to_str().unwrap()
            ])
            .stdout(output_file)
            .status()
            .unwrap();
    } else {
        info!(target: LOG_TARGET, "Skipping creation of {}, already exists", &head_new_drv_outputs_str);
    }
}

pub fn query_outpaths(config: &GlobalConfig, worktree_dir: &Path, output_paths_file: &Path) -> ExitStatus {
    let output_file = std::fs::File::create(&output_paths_file).expect(
        &format!("Unable to write to {}", &output_paths_file.to_str().unwrap())
    );
    // This should create a file with the following contents:
    // <attr>.<system> <drv-path> <out-path>
    let nix_env_args = &[
        "-f",
        config.outpaths_expression_path.to_str().unwrap(),
        "-qaP",
        "--no-name",
        "--out-path",
        "--drv-path",
        "--arg",
        "checkMeta",
        "true",
        "--arg",
        "path",
        worktree_dir.to_str().unwrap()
    ];

    info!(target: LOG_TARGET, "Running command: nix-env {}", nix_env_args.join(" "));
    let cmd1 = Command::new("nix-env")
        .current_dir(worktree_dir)
        .stdout(std::process::Stdio::piped())
        .args(nix_env_args)
        .spawn().unwrap();
    
    // tests.nixos-functions and tests.trivial just create noise
    let filter_noisy_drvs_cmd = Command::new("grep")
        .current_dir(worktree_dir)
        .stdin(cmd1.stdout.unwrap())
        .stdout(std::process::Stdio::piped())
        .args(&[ "-Fv", "-e", "tests.nixos-function", "-e", "tests.trivial"])
        .spawn().unwrap();

    Command::new("sort")
        .current_dir(worktree_dir)
        .stdin(filter_noisy_drvs_cmd.stdout.unwrap())
        .stdout(output_file)
        .status()
        .expect("Unable to query derivation list")
}

pub fn get_pr_response(pr_number: u64) -> Result<String, reqwest::Error> {
    let request_client = Client::new();

    let mut request = request_client
        .get(format!("https://api.github.com/repos/nixos/nixpkgs/pulls/{}", pr_number))
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github.v3+json");

    if let Ok(github_token) = std::env::var("GITHUB_TOKEN") {
        debug!(
            target: LOG_TARGET,
            "Using github token for querying pull request #{}", pr_number
        );
        request = request.header("Authorization", format!("token {}", github_token));
    }

    info!(target: LOG_TARGET, "Querying PR #{}", pr_number);
    request.send()?.text()
    //.and_then(|body|
    //serde_json::from_str::<PullRequest>(&body))
}

// fn handle_serialization_error(body: reqwest::Body, err: serde_json::Err) {
//     error!(
//         target: LOG_TARGET,
//         "Unable to parse response from github to json: {:?}", err
//     );
// 
//     let mut tmpfile = std::env::temp_dir();
//     tmpfile.push("basinix");
//     tmpfile.push("failed_json_parse");
//     std::fs::create_dir_all(&tmpfile.as_path());
//     tmpfile.push(format!("{}.txt", Local::now().to_rfc3339()));
//     let tmp_path = tmpfile.as_path();
// 
//     error!(
//         target: LOG_TARGET,
//         "Writing contents to {}",
//         &tmpfile.display()
//     );
//     std::fs::write(&tmp_path, body.as_bytes()).unwrap();
// }
// 