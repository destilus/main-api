use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
// use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample;
use crate::sample::model::NewPost;
use crate::sample::model::Post;

// #[get("/")]
pub fn all_posts(_connection: DbConn) -> String {
    String::from("all_posts from sample")
    // sample::repository::show_posts(&connection)
    //     .map(|post| Json(post))
    //     .map_err(|error| error_status(error))
}
