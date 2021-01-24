use std::sync::mpsc::channel::Sender;
use eval_events::EvalRequest;
use std::thread::sleep;
use std::time::Duration;
use reqwest::{
    RequestBuilder,
    header::HeaderMap
};

fn produce_github_pr_events(gh_sender: Sender<EvalRequest>) {
    let sleep_duration = Duration::from_seconds(10);

    let request_client = RequestBuilder::new()
        .header("Accept", "application/vnd.github.v3+json")
        .get("https://api.github.com/repos/nixos/nixpkg/events");
    loop {
        sleep(sleep_duration);
    }
}
