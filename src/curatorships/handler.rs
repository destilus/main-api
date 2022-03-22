// use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::connection::DbConn;
use crate::curatorships;
use crate::curatorships::model::NewPost;
use crate::curatorships::model::Post;
use crate::curatorships::model::{NewCuratorship, NewCuratorshipDto, NewCuratorshipItem};

use super::model::Curatorship;

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


#[post("/curatorships", format = "application/json", data = "<new_curatorship_dto>")]
pub fn create_curatorship(
    connection: DbConn,
    new_curatorship_dto: Json<NewCuratorshipDto<'_>>,
) -> Result<status::Created<Json<Curatorship>>, Status> {
    let dto = new_curatorship_dto.into_inner();
    // todo: unwrap somente para teste
    // Options colocados como None por enquanto
    // let channel_id_temp = Uuid::parse_str(dto.channel_id).unwrap();
    // let curator_bet_id_temp = None;
    let curator_id_temp = Uuid::parse_str(dto.curator_id).unwrap();
    // let defied_curatorship_id_temp = Uuid::parse_str(dto.defied_curatorship_id).unwrap();
    let new_curatorship = NewCuratorship{
        body : dto.metadata.text,
        category: dto.category,
        channel_id: None,
        curator_bet_id: None,
        curator_id: curator_id_temp,
        curr_status: dto.curr_status,
        defied_curatorship_id: None,
        exclusivity: dto.exclusivity,
        frequency: dto.frequency, 
        hero_image_url: dto.metadata.hero_image,
        previews_count: dto.previews_count,
        price_currency: dto.price_currency,
        priority_order: dto.priority_order,
        single_price: dto.single_price,
        subtitle: dto.metadata.subtitle.unwrap(),
        title: dto.metadata.title,
        verified: dto.verified
    };
    curatorships::repository::create_curatorship(new_curatorship, &connection)
         .map(|curatorship| curatorship_created(curatorship))
         .map_err(|error| error_status(error))

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

fn curatorship_created(curatorship: Curatorship) -> status::Created<Json<Curatorship>> {
    println!("here final");
    status::Created::new(format!(
        "{host}:{port}/curatorship/{id}",
        host = "localhost",
        port = "8000",
        id = curatorship.id
    ))
    .body(Json(curatorship))
}
