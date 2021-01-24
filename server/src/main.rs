use warp::Filter;
use std::os::unix::net::{UnixStream,UnixListener};

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let stream = UnixStream::connect("/run/user/1000/basinix/basinix.sock").unwrap();

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await
}

