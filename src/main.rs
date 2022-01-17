#[macro_use] extern crate rocket;

use rocket::tokio::time::{sleep, Duration};

#[get("/quick")]
fn quick_hello() -> &'static str{
    "Hello, World"
}

#[get("/long")]
fn long_hello() -> &'static str{
    // even though is a sync handler, it does not block other requests, 
    // because rocket 0.5 is async (any number of incoming connections N spread out across M threads) 
    let one_billion = 100000000;
    for _el in 0..one_billion {

    }
    "Hello, World"
}

#[get("/sleep/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("waited for {} seconds", seconds)
}

// Rust's Futures are a form of cooperative multitasking. In general, 
// Futures and async fns should only .await on operations and never block. 
// Some common examples of blocking include locking non-async mutexes, 
// joining threads, or using non-async library functions (including those in std) 
// that perform I/O.
// If a Future or async fn blocks the thread, inefficient resource usage, stalls, 
// or sometimes even deadlocks can occur.
// Sometimes there is no good async alternative for a library or operation. 
// If necessary, you can convert a synchronous operation to an async 
// one with tokio::task::spawn_blocking:
use std::io;
use rocket::tokio::task::spawn_blocking;
#[get("/")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![quick_hello, long_hello])
        .mount("/time", routes![delay])
        .mount("/blocking", routes![blocking_task])
}
