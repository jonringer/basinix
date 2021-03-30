use basinix_shared::eval_events::EvalRequest;
use basinix_shared::github::PullRequest;
use log::{error,debug,info};
use reqwest::blocking::{Client,Response,Request,RequestBuilder};
use chrono::Local;
use std::collections::HashSet;

const LOG_TARGET: &str = "basinix::server::github_polling";

pub fn get_pr_info(pr_number: u64) {
    let request_client = Client::new();

    let mut request = request_client
        .get("https://api.github.com/repos/nixos/nixpkgs/events?per_page=100")
        .header("User-Agent", "reqwest")
        .header("Accept", "application/vnd.github.v3+json");

    if let Ok(github_token) = std::env::var("GITHUB_TOKEN") {
        debug!(target: LOG_TARGET, "Using github token for querying pull request #{}", pr_number);
        request = request.header("Authorization", format!("token {}", github_token));
    }

    info!(target: LOG_TARGET, "Querying PR #{}", pr_number);
    match request.send() {
        Ok(response) => {
            let pr_info = serialize_repsonse(response);
        },
        Err(err) => {
            error!("Error attempting to contact github: {}", err);
        }
    }
}

fn serialize_response(response: Response) -> PullRequest {
    match response.text() {
        Ok(body) => {
            match serde_json::from_str::<PullRequest>(&body) {
                Ok(parsed_json) => {
                    parsed_json
                }
                Err(err) => {
                    error!(target: LOG_TARGET, "Unable to parse response from github to json: {:?}", err);

                    let mut tmpfile = std::env::temp_dir();
                    tmpfile.push("basinix");
                    tmpfile.push("failed_json_parse");
                    std::fs::create_dir_all(&tmpfile.as_path());
                    tmpfile.push(format!("{}.txt", Local::now().to_rfc3339()));
                    let tmppath = tmpfile.as_path();

                    error!(target: LOG_TARGET, "Writing contents to {}", &tmpfile.display());
                    std::fs::write(&tmppath, body.as_bytes()).unwrap();
                }
            }
        },
        Err(err) => {
            error!("Unable to parse response from github: {:?}", err);
        }
    }
    return PullRequest::new();
}
