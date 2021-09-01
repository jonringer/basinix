//! Manages priority and backlog of in-queued drv builds
//! 
//! Maintains a priority heap of builds-to-be-built, as well
//! as maintains state synchronization with database.

use basinix_shared::types::{BuildRequest, GlobalConfig};
use std::sync::atomic::{AtomicU64, Ordering};
use log::{error, info};

const build_count: AtomicU64 = AtomicU64::new(0);

const LOG_TARGET: &str = "basinix::server::build_manager";

pub fn spawn_build_manager (config: GlobalConfig, recv: std::sync::mpsc::Receiver<BuildRequest>) {
    //TODO: use rayon threadpool
    let sleep_duration = std::time::Duration::from_secs(1);

    loop {
        let request = recv.recv()
            .expect("Build request channel was closed, unable to build any more derivations");

        // wait until enough builds finish
        while build_count.load(Ordering::SeqCst) >= config.parallel_builds {
            std::thread::sleep(sleep_duration);
        }

        info!(target: LOG_TARGET, "building {}", &request.drv);

        let new_config = config.clone();
        std::thread::Builder::new()
            .name(format!("Builder for {}", &request.drv))
            .spawn(move || {
                build_drv(request, new_config);
            })
            .unwrap();
    }
}

fn build_drv(request: BuildRequest, config: GlobalConfig) {
    let worktree_dir_str = format!("{}/{}", config.worktree_dir.to_str().unwrap(), request.rev);
    let worktree_dir = std::path::Path::new(&worktree_dir_str);

    // A given drv might be part of the build closure, but might not be able
    // exposed through an attr. E.g. a drv declared in a let block
    // Use the attr to "hydrate" the store drv
    std::process::Command::new("nix-instantiate")
        .current_dir(&worktree_dir)
        .args(&[ "-A", &request.attr]);

    // Perform the "build"
    // TODO: Use cgroups to properly sandbox by threads
    info!(target: LOG_TARGET, "starting build for attr: {}, drv: {}", &request.attr, &request.drv);
    std::process::Command::new("nix-store")
        .current_dir(&worktree_dir)
        .args(&[ "-r", &request.drv, "--cores", &config.cores_per_build.to_string() ]);
    info!(target: LOG_TARGET, "finished build for attr: {}, drv: {}", &request.attr, &request.drv);
    
    build_count.fetch_sub(1, Ordering::SeqCst);

    // TODO: emit that build succeeced
}