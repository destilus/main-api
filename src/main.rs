#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod connection;
mod curatorships;
mod sample;
mod schema;

#[rocket::main]
async fn main() {
    dotenv().ok();
    // sample::router::create_routes().await;
    curatorships::router::create_routes().await;
}
