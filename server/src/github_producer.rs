use basinix_shared::github::repo_events::{Action, Event, EventType};
use basinix_shared::types::Message::{self, EvalPullRequest, EvalPush, PullRequestClosed};
use basinix_shared::types::{PullRequestInfo, PushInfo};
use chrono::Local;
use log::{debug, error, info};
use reqwest::blocking::{Client, Response};
use std::collections::HashSet;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

const LOG_TARGET: &str = "basinix::server::github_polling";

fn gh_event_to_eval_event(gh_event: Event) -> Option<Message> {
    match gh_event.event_type {
        EventType::PullRequestEvent => match gh_event.payload.action.unwrap() {
            Action::Closed => Some(PullRequestClosed(gh_event.payload.number.unwrap())),
            _ => Some(EvalPullRequest(PullRequestInfo {
                number: gh_event.payload.number.unwrap(),
                base_branch: gh_event.payload.pull_request.unwrap().base.base_ref,
            })),
        },
        EventType::PushEvent => Some(EvalPush(PushInfo {
            push_ref: gh_event.payload.payload_ref.unwrap(),
            before: gh_event.payload.before.unwrap(),
            head: gh_event.payload.head.unwrap(),
        })),
        _ => None,
    }
}

pub fn produce_github_pr_events(sender: Sender<Message>) {
    let mut sleep_seconds = 5;
    let request_client = Client::new();

    let mut past_events: HashSet<u64> = HashSet::with_capacity(1000);

    loop {
        let mut request = request_client
            .get("https://api.github.com/repos/nixos/nixpkgs/events?per_page=100")
            .header("User-Agent", "reqwest")
            .header("Accept", "application/vnd.github.v3+json");

        if let Ok(github_token) = std::env::var("GITHUB_TOKEN") {
            debug!(target: LOG_TARGET, "Using github token for polling events");
            request = request.header("Authorization", format!("token {}", github_token));
        }

        info!(target: LOG_TARGET, "Polling github activity");
        match request.send() {
            Ok(response) => {
                let events = serialize_and_filter_events(response, &mut past_events);

                // Adjust polling rate based on successful retrievals
                // These rates will also only reflect push events, so the number of new
                // events may be much less than the total pay load
                if events.len() < 5 {
                    debug!(
                        target: LOG_TARGET,
                        "Only {} new events returned. Doubling current sleep time of {}s",
                        events.len(),
                        sleep_seconds
                    );
                    // TODO: Configure max wait time between polls
                    sleep_seconds = std::cmp::min(sleep_seconds * 2, 300);
                } else if events.len() > 15 {
                    debug!(
                        target: LOG_TARGET,
                        "More than 15 new events returned. Halving sleep time {}", sleep_seconds
                    );
                    // TODO: Configure minimum time between polls
                    sleep_seconds = std::cmp::max(sleep_seconds / 2, 5);
                }

                // send events to evaluator
                for event in events {
                    sender.send(gh_event_to_eval_event(event).unwrap());
                }
            }
            Err(err) => {
                error!("Error attempting to contact github: {}", err);
            }
        }
        debug!(target: LOG_TARGET, "Sleeping for {} seconds", sleep_seconds);
        sleep(Duration::from_secs(sleep_seconds));
    }
}

fn serialize_and_filter_events(response: Response, past_events: &mut HashSet<u64>) -> Vec<Event> {
    match response.text() {
        Ok(body) => {
            match serde_json::from_str::<Vec<Event>>(&body) {
                Ok(parsed_json) => {
                    debug!(
                        target: LOG_TARGET,
                        "Successfully queried {} values",
                        parsed_json.len()
                    );
                    let commit_changes = parsed_json.into_iter().filter(|event| {
                        event.event_type == EventType::PushEvent
                            || event.event_type == EventType::PullRequestEvent
                    });
                    let (old_events, new_events): (Vec<Event>, Vec<Event>) = commit_changes
                        .partition(|event| past_events.contains(&event.id.parse::<u64>().unwrap()));

                    // TODO: configure this value
                    // Only the last 100 events are really useful, using 1000 just to avoid cache
                    // churn
                    if past_events.len() > 800 {
                        debug!(target: LOG_TARGET, "Clearing old event cache");
                        past_events.clear();
                        old_events.iter().for_each(|event| {
                            past_events.insert(event.id.parse::<u64>().unwrap());
                            ()
                        });
                    }

                    new_events.iter().for_each(|event| {
                        past_events.insert(event.id.parse::<u64>().unwrap());
                        ()
                    });
                    debug!(target: LOG_TARGET, "Old events: {}", old_events.len());
                    debug!(target: LOG_TARGET, "New events: {}", new_events.len());
                    return new_events;
                }
                Err(err) => {
                    error!(
                        target: LOG_TARGET,
                        "Unable to parse response from github to json: {:?}", err
                    );

                    let mut tmpfile = std::env::temp_dir();
                    tmpfile.push("basinix");
                    tmpfile.push("failed_json_parse");
                    std::fs::create_dir_all(&tmpfile.as_path()).expect(&format!(
                        "Unable to create directory {:?}",
                        &tmpfile.as_path()
                    ));
                    tmpfile.push(format!("{}.txt", Local::now().to_rfc3339()));
                    let tmppath = tmpfile.as_path();

                    error!(
                        target: LOG_TARGET,
                        "Writing contents to {}",
                        &tmpfile.display()
                    );
                    std::fs::write(&tmppath, body.as_bytes()).unwrap();
                }
            }
        }
        Err(err) => {
            error!("Unable to parse response from github: {:?}", err);
        }
    }

    return Vec::<Event>::new();
}
