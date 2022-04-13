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

use super::model::{Curatorship, CuratorshipItem};
use std::time::SystemTime;

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

#[post("/posts", format = "application/json", data = "<new_post>")]
pub fn create_post(
    connection: DbConn,
    new_post: Json<NewPost<'_>>,
) -> Result<status::Created<Json<Post>>, Status> {
    let data = new_post.into_inner();
    let post = data;
    // let post = NewPost {
    //     curator_id: None,
    //     body: data.body,
    //     title: data.title,
    // };
    curatorships::repository::create_post(post, &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_status(error))
}

#[post(
    "/curatorships",
    format = "application/json",
    data = "<new_curatorship_dto>"
)]
pub fn create_curatorship(
    connection: DbConn,
    new_curatorship_dto: Json<NewCuratorshipDto<'_>>,
) -> Result<status::Created<Json<CuratorshipItem>>, Status> {
    let dto = new_curatorship_dto.into_inner();
    // todo: unwrap somente para teste
    // Options colocados como None por enquanto
    // let channel_id_temp = Uuid::parse_str(dto.channel_id).unwrap();
    // let curator_bet_id_temp = None;
    let curator_id_temp = Uuid::parse_str(dto.curator_id).unwrap();
    // let defied_curatorship_id_temp = Uuid::parse_str(dto.defied_curatorship_id).unwrap();
    let new_curatorship = NewCuratorship {
        channel_id: None,
        curator_bet_id: None,
        curator_id: curator_id_temp,
        defied_curatorship_id: None,
        body: dto.metadata.text,
        category: dto.category,
        curr_status: dto.curr_status,
        exclusivity: dto.exclusivity,
        subtitle: dto.metadata.subtitle,
        title: dto.metadata.title,
        hero_image_url: dto.metadata.hero_image,
        previews_count: dto.previews_count,
        priority_order: dto.priority_order,
        price_currency: dto.price_currency,
        single_price: dto.single_price,
        verified: dto.verified,
        published_at: Some(SystemTime::now()),
    };

    let mut new_curatorship_items: Vec<NewCuratorshipItem> = dto
        .items
        .iter()
        .map(|item| NewCuratorshipItem {
            curatorship_id: Uuid::new_v4(),
            priority_order: item.priority,
            external_ref: item.external_ref,
            title: item.metadata.title,
            subtitle: item.metadata.subtitle,
            hero_image_url: item.metadata.hero_image,
            body: item.metadata.text,
        })
        .collect();

    let first_item = new_curatorship_items
        .pop()
        .expect("expected at leat one item");

    curatorships::repository::create_curatorship(new_curatorship, first_item, &connection)
        .map(|curatorship| curatorship_item_created(curatorship))
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

fn curatorship_item_created(
    curatorship_item: CuratorshipItem,
) -> status::Created<Json<CuratorshipItem>> {
    println!("here final");
    status::Created::new(format!(
        "{host}:{port}/curatorship-item/{id}",
        host = "localhost",
        port = "8000",
        id = curatorship_item.id
    ))
    .body(Json(curatorship_item))
}
