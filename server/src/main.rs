use warp::Filter;
use std::os::unix::net::{UnixStream,UnixListener};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use log::{debug, info, error};
use env_logger;
mod github_producer;
use github_producer::produce_github_pr_events;
use basinix_shared::eval_events::EvalRequest;
use chrono::Local;
use std::io::Write;

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

    let (tx, rx) = mpsc::channel::<EvalRequest>();
    info!(target: LOG_TARGET, "Starting github polling thread");
    thread::Builder::new().name("github_producer".to_string()).spawn(move|| {
        produce_github_pr_events(tx)
    });

    println!("waiting to receive");
    println!("{:?}", rx.recv().unwrap());
    let sleep_time = Duration::from_secs(20);
    thread::sleep(sleep_time);

    let stream = UnixStream::connect("/run/user/1000/basinix/basinix.sock").unwrap();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await
}

