use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use std::io::Write;
use log::info;

lazy_static! {
    static ref OUTPATHS_EXPRESSION: &'static str = include_str!("./outpaths.nix");
}

const LOG_TARGET: &str = "basinix::shared::types";

#[derive(Serialize, Deserialize, Debug)]
#[derive(derive_more::From)]
// should match entries in up.sql
pub enum PRStatus {
  Unknown = 1,
  Queued = 2,
  InProgress = 3,
  Closed = 4,
  Success = 5,
  Failed = 6,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NixOSBranch {
    Master,
    Staging,
    StagingNext,
    Other(String),
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
pub enum Arch {
    X86,
    X86_64,
    Aarch64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum System {
    Linux,
    Darwin,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BuildRequest {
    pub platform: String, // e.g. Platform { arch = X86_64, system = Linux }
    pub rev: String,        // Git commit sha
    pub attr: String,       // Attr to "hydrate" the drv
    pub drv: String,        // e.g. /nix/store/e1qr....-package.drv
    pub build_count: u32,   // to determine build priority
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

// TODO: Message is a terrible name
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Message {
    EvalPush(PushInfo),
    EvalPullRequest(PullRequestInfo),
    PullRequestClosed(u64),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct FileConfig {
    pub cache_dir: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GlobalConfig {
    pub cores_per_build: u32,
    pub parallel_builds: u64,
    pub cache_dir: PathBuf,
    pub nixpkgs_dir: PathBuf,
    pub worktree_dir: PathBuf,
    pub outpaths_expression_path: PathBuf,
}

fn create_dir(path: &std::path::Path) {
     info!(target: LOG_TARGET, "Creating {}", &path.to_str().unwrap());
     std::fs::create_dir_all(&path)
        .expect(&format!("Unable to create directory {}", &path.display()));
}

impl GlobalConfig {
    /// Read option supplied by user, and create a RO config
    pub fn new(config: &FileConfig) -> GlobalConfig {
        let cache_dir = match &config.cache_dir {
            Some(dir) => std::path::Path::new(&dir.replace("//","/")).to_path_buf(),
            None => {
                // use $XDG_CACHE_DIR/basinix otherwise
                let mut dir = std::path::PathBuf::new();
                dir.push(&dirs::cache_dir().unwrap().to_str().unwrap().to_owned().replace("//","/"));
                dir.push("basinix");
                dir
            }
        };

        let nixpkgs_dir = {
            let mut dir = cache_dir.to_path_buf();
            dir.push("nixpkgs");
            dir
        };

        let worktree_dir = {
            let mut dir = cache_dir.to_path_buf();
            dir.push("worktrees");
            dir
        };

        let outpaths_expression_path = {
            let mut dir = cache_dir.to_path_buf();
            dir.push("out-paths.nix");
            dir
        };

        if !outpaths_expression_path.exists() {
            info!(target: LOG_TARGET, "Creating {}", &outpaths_expression_path.to_str().unwrap());
            std::fs::create_dir_all(
                &outpaths_expression_path.parent().unwrap())
                .expect(&format!("Unable to create directory for: {}", &outpaths_expression_path.display()));
            let mut f = std::fs::File::create(&outpaths_expression_path)
                .expect(&format!("Unable to create file at location: {}", &outpaths_expression_path.to_str().unwrap()));
            f.write_all(OUTPATHS_EXPRESSION.as_bytes())
                .expect(&format!("Unable to write to location: {}", &outpaths_expression_path.to_str().unwrap()));
        }

        if !worktree_dir.exists() {
            create_dir(&worktree_dir);
        }

        GlobalConfig {
            // TODO: allow for coures and build number to be configured
            // TODO: allow for coures and build number to be configured per host
            cores_per_build: 2,
            parallel_builds: 64,
            cache_dir,
            nixpkgs_dir,
            worktree_dir,
            outpaths_expression_path,
        }
    }
}
