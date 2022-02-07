// use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::connection::DbConn;
use crate::sample;
use crate::sample::model::NewPost;
use crate::sample::model::Post;

#[get("/")]
pub fn all_posts(connection: DbConn) -> String {
    sample::repository::mock_posts(&connection)
    // sample::repository::show_posts(&connection)
    //     .map(|post| Json(post))
    //     .map_err(|error| error_status(error))
}

#[post("/posts", format = "application/json", data = "<new_post>")]
pub fn create_post(
    connection: DbConn,
    new_post: Json<NewPost<'_>>,
) -> Result<status::Created<Json<Post>>, Status> {
    sample::repository::create_post(new_post.into_inner(), &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

fn post_created(post: Post) -> status::Created<Json<Post>> {
    println!("here final");
    status::Created::new(format!(
        "{host}:{port}/post/{id}",
        host = "localhost",
        port = "8000",
        id = post.id
    ))
    .body(Json(post))
}
