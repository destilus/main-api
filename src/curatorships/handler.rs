// use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::connection::DbConn;
use crate::curatorships;
use crate::curatorships::model::NewPost;
use crate::curatorships::model::Post;
use crate::curatorships::model::{NewCuratorship, NewCuratorshipDto, NewCuratorshipItem};

#[get("/posts")]
pub fn all_posts(connection: DbConn) -> Result<Json<Vec<Post>>, Status> {
    curatorships::repository::show_posts(&connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[get("/count")]
pub fn count_posts(connection: DbConn) -> String {
    curatorships::repository::count_posts(&connection)
    // curatorships::repository::show_posts(&connection)
    //     .map(|post| Json(post))
    //     .map_err(|error| error_status(error))
}

#[post("/posts", format = "application/json", data = "<new_curatorship_dto>")]
pub fn create_curatorship(
    connection: DbConn,
    new_curatorship_dto: Json<NewCuratorshipDto<'_>>,
) -> Result<status::Created<Json<String>>, Status> {
    // curatorships::repository::create_curatorship(new_curatorship_dto.into_inner(), &connection)
    //     .map(|post| post_created(post))
    //     .map_err(|error| error_status(error))

    todo!()
}

#[get("/posts/<id>")]
pub fn get_post(id: i32, connection: DbConn) -> Result<Json<Post>, Status> {
    curatorships::repository::get_post(id, &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[put("/posts/<id>", format = "application/json", data = "<post>")]
pub fn update_post(id: i32, post: Json<Post>, connection: DbConn) -> Result<Json<Post>, Status> {
    curatorships::repository::update_post(id, post.into_inner(), &connection)
        .map(|post| Json(post))
        .map_err(|error| error_status(error))
}

#[delete("/posts/<id>")]
pub fn delete_post(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    curatorships::repository::delete_post(id, &connection)
        .map(|_| status::NoContent)
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
