use basinix_shared::read_config;
use log::{error};
use std::os::unix::net::UnixListener;

fn main() -> std::io::Result<()> {
    let _global_config = read_config();

    println!("Listening on socket");
    std::fs::create_dir_all("/run/user/1000/basinix/")?;

    // TODO: enable some communication between rmote evaluators and main build manager
    // let (sender, receiver) = channel();
    // thread::Builder::new()
    //     .name("Evaluator".to_string())
    //     .spawn(move || {
    //         eval_events(receiver, &global_config);
    //     })
    //     .unwrap();

    let listener = UnixListener::bind("/run/user/1000/basinix/evaluator.sock")?;
    match listener.accept() {
        Ok((_socket, addr)) => {
            println!("Got a client: {:?}", addr);
        }
        Err(e) => {
            error!("accept function failed: {:?}", e)
        }
    }
    Ok(())
}
