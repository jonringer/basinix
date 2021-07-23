use warp::Filter;
use std::sync::mpsc;
use std::thread;
use log::info;
use env_logger;
mod github_producer;
use github_producer::produce_github_pr_events;
use basinix_shared::types::Message;
use chrono::Local;
use std::io::Write;
use basinix_evaluator::eval_events;

#[macro_use]
extern crate diesel;
extern crate serde;

pub mod schema;
pub mod models;

const LOG_TARGET: &str = "basinix::server::main";

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(buf,
                "[{}] {:5} - {} - {}",
                Local::now().to_rfc3339(),
                record.level(),
                record.target(),
                record.args()
            )
        }).init();

    let (tx, rx) = mpsc::channel::<Message>();

    // TODO: Allow for evaluator to exist on another machine
    info!(target: LOG_TARGET, "Starting evaluation thread");
    thread::Builder::new().name("Evalutor".to_string()).spawn(move|| {
        eval_events(rx);
    }).unwrap();

    info!(target: LOG_TARGET, "Starting github polling thread");
    thread::Builder::new().name("github_producer".to_string()).spawn(move|| {
        produce_github_pr_events(tx);
    }).unwrap();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await
}

