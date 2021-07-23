use std::os::unix::net::UnixListener;
use std::thread;
use std::sync::mpsc::{channel, Receiver};
use log::{error, info};
use basinix_evaluator::eval_events;

fn main() -> std::io::Result<()> {
    println!("Listening on socket");
    std::fs::create_dir_all("/run/user/1000/basinix/")?;

    let (sender, receiver) = channel();
    thread::Builder::new().name("Evaluator".to_string()).spawn(move|| {
        eval_events(receiver);
    }).unwrap();

    let listener = UnixListener::bind("/run/user/1000/basinix/evaluator.sock")?;
    match listener.accept() {
        Ok((_socket, addr)) => {
            println!("Got a client: {:?}", addr);

        },
        Err(e) => {
            error!("accept function failed: {:?}", e)
        },
    }
    Ok(())
}
