use std::sync::mpsc::Sender;
use basinix_shared::eval_events::EvalRequest;
use basinix_shared::gh_events::Event;
use serde_json::Deserializer;
use std::thread::sleep;
use std::time::Duration;
use log::{error,debug,info};
use reqwest::blocking::{Client,Response,Request,RequestBuilder};
use chrono::Local;

const LOG_TARGET: &str = "basinix::server::github_polling";

pub fn produce_github_pr_events(gh_sender: Sender<EvalRequest>) {
    let sleep_duration = Duration::from_secs(1);
    let request_client = Client::new();

    loop {
        let request = request_client
            .get("https://api.github.com/repos/nixos/nixpkgs/events?per_page=100")
            .header("User-Agent", "reqwest")
            .header("Accept", "application/vnd.github.v3+json");

        info!(target: LOG_TARGET, "Polling github activity");
        match request.send() {
            Ok(response) => {
                let events = serialize_and_filter_events(response);
            },
            Err(err) => {
                error!("Error attempting to contact github: {}", err);
            }
        }
        sleep(sleep_duration);
    }
}

fn serialize_and_filter_events(response: Response) -> Vec<Event> {
    match response.text() {
        Ok(body) => {
            match serde_json::from_str::<Vec<Event>>(&body) {
                Ok(parsed_json) => {
                    debug!(target: LOG_TARGET, "Successfully queried {} values", parsed_json.len());
                    return parsed_json;
                }
                Err(err) => {
                    error!("Unable to parse response from github to json: {:?}", err);

                    let mut tmpfile = std::env::temp_dir();
                    tmpfile.push("basinix");
                    tmpfile.push("failed_json_parse");
                    std::fs::create_dir_all(&tmpfile.as_path());
                    tmpfile.push(format!("{}.txt", Local::now().to_rfc3339()));
                    let tmppath = tmpfile.as_path();

                    error!("Writing contents to {}", &tmpfile.display());
                    std::fs::write(&tmppath, body.as_bytes());
                }
            }
        },
        Err(err) => {
            error!("Unable to parse response from github: {:?}", err);
        }
    }
    return Vec::<Event>::new();
}
