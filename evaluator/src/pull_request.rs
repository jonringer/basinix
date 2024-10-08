
use basinix_shared::error::Result;
use basinix_shared::github::pull_request::PullRequest;
use basinix_shared::types::{BuildRequest, GlobalConfig};

use log::{debug, info};
use reqwest::blocking::Client;

use std::process::Command;
use std::io::{Read, BufRead, BufReader, BufWriter, Write};

use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::fs::File;

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
            &format!("{}:refs/basinix/{}", base_ref, base_ref)
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
            &format!("pull/{}/head:refs/basinix/pull/{}", pr_number, pr_number)
        ])
        .status()
        .unwrap_or_else(|_| panic!("Unable to fetch PR #{} for nixpkgs", pr_number));
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
                worktree_dest_dir.to_str().unwrap(),
                rev
            ])
            .status()
            .expect("Unable to create worktree");
    } else {
        info!(target: LOG_TARGET, "Skipping creating a worktree for {}, already exists", worktree_dest_dir.to_str().unwrap());
    }
}

fn three_words(s: String) -> Option<(String, String, String)> {
    let mut words = s.split_whitespace();
    let first = words.next()?;
    let second = words.next()?;
    let third = words.next()?;
    Some((first.to_owned(), second.to_owned(), third.to_owned()))
}

/// The original outputs.txt will contain `<attr> <drv> <outpath>`, split those into their own separate files
fn split_output_to_separate_files(src_file: impl Read, prefix: &str, worktree_dir: &Path) {
    let first_words = BufReader::new(src_file)
        .lines()
        .filter_map(|line| line.map(three_words).ok().unwrap_or(None));

    let attrs_file_str = format!("{}/{}_{}", worktree_dir.display(), prefix, "attrs.txt");
    let drvs_file_str = format!("{}/{}_{}", worktree_dir.display(), prefix, "drvs.txt");
    let output_paths_file_str = format!("{}/{}_{}", worktree_dir.display(), prefix, "output_paths.txt");

    let mut attrs_file = BufWriter::new(File::create(attrs_file_str).expect("Unable to open attrs_file"));
    let _drvs_file = File::create(drvs_file_str);
    let _output_paths_file = File::create(output_paths_file_str);

    for (attr, _drv, _outpath) in first_words {
        let _sanitized_attr = attr.strip_suffix(".x86_64-linux").unwrap_or(&attr);
        attrs_file.write_all(attr.as_bytes()).unwrap();
        attrs_file.write_all("\n".as_bytes()).unwrap();
    }
}

pub fn eval_pr(config: &GlobalConfig, _build_sender: std::sync::mpsc::Sender<BuildRequest>, pr_number: u64, base_revs: &mut std::collections::HashMap<String, String>) -> Result<u32> {

    let pr_info: PullRequest = get_pr_response(pr_number)?;

    // This is less awkward than using the PathBuf `push` paradigm
    let base_path = format!("{}/{}", config.worktree_dir.to_str().unwrap(), &pr_info.base.base_ref);
    let base_worktree_dir = std::path::Path::new(&base_path);

    let head_path = format!("{}/{}", config.worktree_dir.to_str().unwrap(), &pr_info.head.sha);
    let head_worktree_dir = Path::new(&head_path);
    let head_drv_outputs_str = format!("{}/{}", &head_path, "outputs.txt");
    let head_drv_outputs = Path::new(&head_drv_outputs_str);
    let head_changed_drv_outputs_str = format!("{}/{}", &head_path, "changed_drvs.txt");
    let head_changed_drv_outputs = Path::new(&head_changed_drv_outputs_str);
    let head_changed_attr_outputs_str = format!("{}/{}", &head_path, "changed_attrs.txt");
    let _head_changed_attr_outputs = Path::new(&head_changed_attr_outputs_str);
    let head_old_drv_outputs_str = format!("{}/{}", &head_path, "old_drvs.txt");
    let head_old_drv_outputs = Path::new(&head_old_drv_outputs_str);
    let head_old_attr_outputs_str = format!("{}/{}", &head_path, "old_drvs.txt");
    let _head_old_attr_outputs = Path::new(&head_old_drv_outputs_str);
    let head_added_attr_outputs_str = format!("{}/{}", &head_path, "added_attrs.txt");
    let head_added_attr_outputs = Path::new(&head_added_attr_outputs_str);
    let head_removed_attr_outputs_str = format!("{}/{}", &head_path, "removed_attrs.txt");
    let head_removed_attr_outputs = Path::new(&head_removed_attr_outputs_str);
    let head_base_rev_str = format!("{}/{}", &head_path, "head_revision.txt");
    let head_base_rev = Path::new(&head_base_rev_str);

    // create worktree for base branch if missing
    // Check if we need to do any work
    // TODO: make sha check less ugly
    if &pr_info.base.sha.as_str() == &base_revs.get(&pr_info.base.base_ref).unwrap_or(&"0".to_string()) {
        info!(target: LOG_TARGET, "Skipping checkout of {} branch, sha is the same", &pr_info.base.base_ref);
    } else {
        let base_rev = create_base_worktree_if_missing(config.nixpkgs_dir.as_path(), &pr_info.base.base_ref);
        base_revs.insert(pr_info.base.base_ref.to_string(), base_rev);
    }

    // create worktree for head branch if missing
    fetch_pr(config.nixpkgs_dir.as_path(), pr_number);
    create_worktree_if_missing(
        config.nixpkgs_dir.as_path(),
        head_worktree_dir,
        &pr_info.head.sha);
    create_worktree_if_missing(
        config.nixpkgs_dir.as_path(),
        base_worktree_dir,
        &pr_info.base.base_ref);

    let (base_rev, base_drv_outputs): (String, PathBuf) = query_base_outpaths(config, base_worktree_dir, &pr_info.base.base_ref)?;

    // query outpaths
    if !head_drv_outputs.exists() {
        query_pr_outpaths(config, head_worktree_dir, head_drv_outputs, &pr_info.base.base_ref, &base_rev)?;
    }

    // create changed derivations file
    if !head_changed_drv_outputs.exists() {
        let output_file = File::create(&head_changed_drv_outputs).unwrap_or_else(|_| panic!("Unable to write to {}", &head_changed_drv_outputs_str));
        Command::new("comm")
            .args(&[
                "-13",
                base_drv_outputs.to_str().unwrap(),
                head_drv_outputs.to_str().unwrap()
            ])
            .stdout(output_file)
            .status()
            .unwrap();

        // also write the changed attrs to a separate file
        let changed_drvs_file = std::fs::File::open(&head_changed_drv_outputs).unwrap_or_else(|_| panic!("Unable to read to {}", &head_changed_drv_outputs_str));
        split_output_to_separate_files(&changed_drvs_file, "changed", head_worktree_dir);
    } else {
        info!(target: LOG_TARGET, "Skipping creation of {}, already exists", &head_changed_drv_outputs_str);
    }

    // create old derivations file, these will be used to determine changed builds and regressions
    if !head_old_drv_outputs.exists() {
        let output_file = File::create(&head_old_drv_outputs).unwrap_or_else(|_| panic!("Unable to write to {}", &head_old_drv_outputs_str));
        Command::new("comm")
            .args(&[
                "-23",
                base_drv_outputs.to_str().unwrap(),
                head_drv_outputs.to_str().unwrap()
            ])
            .stdout(output_file)
            .status()
            .unwrap();
    } else {
        info!(target: LOG_TARGET, "Skipping creation of {}, already exists", &head_changed_drv_outputs_str);
    }

    if !head_added_attr_outputs.exists() {
        let output_file = File::create(&head_added_attr_outputs).unwrap_or_else(|_| panic!("Unable to write to {}", &head_added_attr_outputs_str));
        Command::new("comm")
            .args(&[
                "-13",
                &head_old_attr_outputs_str,
                &head_changed_attr_outputs_str
            ])
            .stdout(output_file)
            .status()
            .unwrap();
    } else {
        info!(target: LOG_TARGET, "Skipping creation of {}, already exists", &head_added_attr_outputs_str);
    }
    let head_added_attr_outputs_file = BufReader::new(File::open(&head_added_attr_outputs_str).expect("Could not open file"));
    let head_added_attr_outputs_lines = head_added_attr_outputs_file.lines().count() as u32;

    if !head_removed_attr_outputs.exists() {
        let output_file = File::create(&head_removed_attr_outputs).unwrap_or_else(|_| panic!("Unable to write to {}", &head_removed_attr_outputs_str));
        Command::new("comm")
            .args(&[
                "-23",
                &head_old_attr_outputs_str,
                &head_changed_attr_outputs_str
            ])
            .stdout(output_file)
            .status()
            .unwrap();
    } else {
        info!(target: LOG_TARGET, "Skipping creation of {}, already exists", &head_removed_attr_outputs_str);
    }

    // can be used later to determine if results aren't stale
    if !head_base_rev.exists() {
        let mut output_file = File::create(&head_base_rev).unwrap_or_else(|_| panic!("Unable to write to {}", &head_base_rev_str));
        output_file.write(pr_info.base.base_ref.as_bytes())
            .expect("failed to write base_rev.");
    } else {
        info!(target: LOG_TARGET, "Skipping creation of {}, already exists", &head_base_rev_str);
    }

    let head_added_attr_outputs_file = BufReader::new(File::open(&head_added_attr_outputs_str).expect("Could not open file"));
    for maybe_line in head_added_attr_outputs_file.lines() {
        if let Ok(line) = maybe_line {
            let mut parts = line.split_whitespace();
            let attr_path_with_platform = parts.next().unwrap();
            let (attr, platform) = get_attr_parts(attr_path_with_platform);
            let drv = parts.next().unwrap();
            let br = BuildRequest {
                platform: platform.to_string(),
                rev: pr_info.head.sha.to_string(),
                attr: attr.to_string(),
                drv: drv.to_string(),
                build_count: head_added_attr_outputs_lines
            };

            _build_sender.send(br).expect(&format!("Unable to submit build request for {} on sha:{}", &attr, &pr_info.head.sha));
        };
    }

    // TODO: determine number of changed builds from new_drvs file
    Ok(head_added_attr_outputs_lines)
}

/// Splits an attr path into it's respective parts
/// E.g. `python3Packages.requests.x86_64-linux` -> (python3Packages.requests, x86_64-linux)
pub fn get_attr_parts (full_path: &str) -> (String, String) {
    let mut parts = full_path.split(".");

    let mut attr_path = parts.next().unwrap().to_owned();
    let mut temp = parts.next().unwrap();

    // peek at the next value, then consume the previous value accordingly
    if let Some(part) = parts.next() {
        attr_path.push_str(".");
        attr_path.push_str(temp);
        temp = part;
    }

    (attr_path.clone(), temp.to_string())
}

pub fn generate_build_requests (drv_file_path: &Path, _build_sender: Sender<BuildRequest>) {
    let drv_file = File::open(drv_file_path).unwrap_or_else(|_| panic!("Unable to open {}", &drv_file_path.to_str().unwrap()));

    for line in BufReader::new(drv_file).lines() {
        if let Ok(str_part) = line {
            println!("{}", str_part);
        }
    }
}

fn query_base_outpaths(config: &GlobalConfig, worktree_dir: &Path, base_ref: &str) -> Result<(String, PathBuf)> {
    // reset directory, to avoid a previous run
    Command::new("git")
        .current_dir(worktree_dir)
        .args(&[
            "checkout"
            ,"--"
            ,"."
        ])
        .status()?;

    // bring changes from base branch into worktree dir
    Command::new("git")
        .current_dir(worktree_dir)
        .args(&[
            "pull"
            ,"origin"
            ,base_ref
        ])
        .status()?;

    let base_rev: String = std::str::from_utf8(&Command::new("git")
        .current_dir(worktree_dir)
        .args(&[
            "rev-parse"
            ,"HEAD"
        ])
        .output()?
        .stdout)?
        .trim()
        .to_owned();

    let output_paths_file_str = format!("{}/{}.outputs.txt", worktree_dir.display(), &base_rev);
    let output_paths_file = Path::new(&output_paths_file_str);

    query_outpaths(config, worktree_dir, output_paths_file)?;

    Ok((base_rev, output_paths_file.to_path_buf()))
}

pub fn query_pr_outpaths(config: &GlobalConfig, worktree_dir: &Path, output_paths_file: &Path, base_ref: &str, base_rev: &str) -> Result<()> {
    // reset directory, to avoid a previous run
    debug!("Running command: PWD={} git checkout -- .", worktree_dir.display());
    Command::new("git")
        .current_dir(worktree_dir)
        .args(&[
            "checkout"
            ,"--"
            ,"."
        ])
        .status()?;

    // ensure that the commit will be available
    debug!("Running command: PWD={} git fetch origin {}", worktree_dir.display(), base_ref);
    Command::new("git")
        .current_dir(worktree_dir)
        .args(&[
            "fetch"
            ,"origin"
            ,base_ref
        ])
        .status()
        .expect("failed to complete git fetch.");
    // bring changes from base branch into worktree dir
    debug!("Running command: PWD={} git merge --no-commit --no-ff {}", worktree_dir.display(), base_rev);
    Command::new("git")
        .current_dir(worktree_dir)
        .args(&[
            "merge"
            ,"--no-commit"
            ,"--no-ff"
            ,base_rev
        ])
        .status()
        .expect("git pull merge failed.");

    query_outpaths(config, worktree_dir, output_paths_file)?;
    Ok(())
}

pub fn query_outpaths(config: &GlobalConfig, worktree_dir: &Path, output_paths_file: &Path) -> Result<()> {
    let output_file = File::create(&output_paths_file).unwrap_or_else(|_| panic!("Unable to write to {}", &output_paths_file.to_str().unwrap()));
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

    info!(target: LOG_TARGET, "Evaluating derivations for: {}", &worktree_dir.display());
    debug!(target: LOG_TARGET, "Running command: nix-env {}", nix_env_args.join(" "));
    let cmd1 = Command::new("nix-env")
        .current_dir(worktree_dir)
        .stdout(std::process::Stdio::piped())
        .args(nix_env_args)
        .spawn()?;

    // tests.nixos-functions and tests.trivial just create noise
    let filter_noisy_drvs_cmd = Command::new("grep")
        .current_dir(worktree_dir)
        .stdin(cmd1.stdout.unwrap())
        .stdout(std::process::Stdio::piped())
        .args(&[ "-Fv", "-e", "tests.nixos-function", "-e", "tests.trivial"])
        .spawn()?;

    Command::new("sort")
        .current_dir(worktree_dir)
        .stdin(filter_noisy_drvs_cmd.stdout.unwrap())
        .stdout(output_file)
        .status()?;

    debug!(target: LOG_TARGET, "Finished running nix-env for {}.", worktree_dir.display());
    Ok(())
}

pub fn get_pr_response(pr_number: u64) -> Result<PullRequest> {
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
    let body = request.send()?.text()?;
    Ok(serde_json::from_str::<PullRequest>(&body)?)
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
