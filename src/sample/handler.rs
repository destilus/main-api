// use std::env;

// use diesel::result::Error;
// use rocket::http::Status;
// use rocket::response::status;
// use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample;
// use crate::sample::model::NewPost;
// use crate::sample::model::Post;

#[get("/")]
pub fn all_posts(connection: DbConn) -> String {
    sample::repository::mock_posts(&connection)
    // sample::repository::show_posts(&connection)
    //     .map(|post| Json(post))
    //     .map_err(|error| error_status(error))
}
