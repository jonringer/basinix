use std::os::unix::net::UnixListener;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Listening on socket");
    std::fs::create_dir_all("/run/user/1000/basinix/");
    let listener = UnixListener::bind("/run/user/1000/basinix/basinix.sock")?;
    match listener.accept() {
        Ok((socket, addr)) => println!("Got a client: {:?}", addr),
        Err(e) => println!("accept function failed: {:?}", e),
    }
    Ok(())
}
