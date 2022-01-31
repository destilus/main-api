#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

// use crate::schema::posts;

mod api_key;
mod connection;

use api_key::ApiKey;
use dotenv::dotenv;
// use rocket::http::Status;
use crate::connection::DbConn;
use rocket::tokio::time::{sleep, Duration};

// use crate::models::Post;

#[get("/")]
fn index() -> String {
    "Hello index changed".to_string()
}

#[get("/posts")]
pub fn all_posts(connection: DbConn) -> String {
    // repository::get_post_title(&connection)
    String::from("posts")
}

#[get("/quick")]
fn quick_hello(api_key: ApiKey) -> String {
    let value = api_key;
    format!("Hello, World {}", value.0)
}

#[get("/dynamic/<name>")]
fn dynamic_int(name: usize) -> String {
    format!("dynamic usize {}", name)
}

// only /dynamic/ will not be found
// only /dynamic/something will be
#[get("/dynamic/<name>", rank = 2)]
fn dynamic(name: Option<&str>) -> String {
    let result = if let Some(el) = name { el } else { "Nobody" };

    format!("dynamic str {}", result)
}

use std::path::PathBuf;
#[get("/page/<path..>")]
fn get_page(path: PathBuf) -> &'static str {
    let _hey = path;
    "anything after page matches"
}

#[get("/foo/<_>/bar")]
fn foo_bar() -> &'static str {
    "Foo _____ bar!"
}

#[get("/long")]
fn long_hello() -> &'static str {
    // even though is a sync handler, it does not block other requests,
    // because rocket 0.5 is async (any number of incoming connections N spread out across M threads)
    let one_billion = 100000000;
    for _el in 0..one_billion {}
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
use rocket::tokio::task::spawn_blocking;
use std::io;
#[get("/")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    rocket::build()
        .manage(connection::get_pool())
        .mount("/", routes![index, all_posts])
        .mount(
            "/hello",
            routes![quick_hello, long_hello, dynamic, dynamic_int, foo_bar],
        )
        .mount("/time", routes![delay])
        .mount("/blocking", routes![blocking_task])
        .mount("/default", routes![get_page])
        .launch()
        .await
        .ok();
}
