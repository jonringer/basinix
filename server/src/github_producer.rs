use std::sync::mpsc::Sender;
use basinix_shared::eval_events::EvalRequest;
use basinix_shared::gh_events::Event;
use serde_json::Deserializer;
use std::thread::sleep;
use std::time::Duration;
use reqwest::blocking::{Client,Request,RequestBuilder};

pub fn produce_github_pr_events(gh_sender: Sender<EvalRequest>) {
    println!("here");
    let sleep_duration = Duration::from_secs(1);

    let request_client = Client::new();
    gh_sender.send(EvalRequest::PullRequest(1, vec!["abcdef".to_string()])).unwrap();
    loop {
        let request = request_client
            .get("https://api.github.com/repos/nixos/nixpkgs/events")
            .header("User-Agent", "reqwest")
            .header("Accept", "application/vnd.github.v3+json");

        match request.send() {
            Ok(response) => {
                let json = &response.text().unwrap();
                println!("response from gh: {}", json);
                let values: Vec<Event> = serde_json::from_str(json).unwrap();
                println!("success! {} values", values.len());
            },
            Err(err) => {
                println!("Error! :(");
            }
        }
        sleep(sleep_duration);
    }
}
