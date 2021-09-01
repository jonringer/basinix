use warp::Filter;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use log::{debug, info};
use env_logger;
mod github_producer;
use github_producer::produce_github_pr_events;
use basinix_shared::read_config;
use basinix_shared::types::{BuildRequest, GlobalConfig, Message};
use chrono::Local;
use std::io::Write;
use basinix_evaluator::eval_events;

extern crate serde;

pub mod models;
pub mod cli;

mod build_manager;

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

    
    let m = cli::build_cli().get_matches();
    
    let global_config: GlobalConfig = read_config();
    std::mem::forget(read_config());

    debug!(target: LOG_TARGET, "Creating databse connection pool");
    let pool = basinix_shared::db::create_connection_pool().await.expect("Unable to create database pool.");
    debug!(target: LOG_TARGET, "Checking if database needs to be initialized");
    basinix_shared::db::init_database(&pool).await.expect("Unable to create database.");

    let (tx, rx) = mpsc::channel::<Message>();
    let (build_request_sender, build_request_receiver) = mpsc::channel::<BuildRequest>();

    // TODO: Allow for evaluator to exist on another machine
    info!(target: LOG_TARGET, "Starting evaluation thread");
    let eval_config = global_config.clone();
    thread::Builder::new().name("Evalutor".to_string()).spawn(move|| {
        eval_events(rx, build_request_sender, eval_config);
    }).unwrap();

    info!(target: LOG_TARGET, "Starting github polling thread");
    thread::Builder::new()
        .name("github_producer".to_string())
        .spawn(move|| {
            produce_github_pr_events(tx);
        }).unwrap();

    let bm_config = global_config.clone();
    thread::Builder::new()
        .name("build_manager".to_string())
        .spawn(move|| {
            build_manager::spawn_build_manager(bm_config, build_request_receiver);
        }).unwrap();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await
}
