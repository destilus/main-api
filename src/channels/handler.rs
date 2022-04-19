// use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::channels;
use crate::channels::model::{NewChannel, NewChannelDto};
use crate::connection::DbConn;

use super::model::Channel;
use std::time::SystemTime;

#[get("/")]
pub fn all_channels(connection: DbConn) -> Result<Json<Vec<Channel>>, Status> {
    channels::repository::show_channels(&connection)
        .map(|channel| Json(channel))
        .map_err(|error| error_status(error))
}

#[get("/count")]
pub fn count_channels(connection: DbConn) -> String {
    channels::repository::count_channels(&connection)
}

#[post("/", format = "application/json", data = "<new_channel_dto>")]
pub fn create_channel(
    connection: DbConn,
    new_channel_dto: Json<NewChannelDto<'_>>,
) -> Result<status::Created<Json<Channel>>, Status> {
    let dto = new_channel_dto.into_inner();
    let new_channel = NewChannel {
        owner_id: Uuid::parse_str(dto.owner_id).unwrap(),
        exclusivity: dto.exclusivity,
        title: dto.title,
        summary: dto.summary,
        frequency: dto.frequency,
        subscription_price: dto.subscription_price,
        price_currency: dto.price_currency,
    };
    channels::repository::create_channel(new_channel, &connection)
        .map(|channel| channel_created(channel))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_channel(id: &str, connection: DbConn) -> Result<Json<Channel>, Status> {
    let channel_uuid = Uuid::parse_str(id).unwrap();
    channels::repository::get_channel(channel_uuid, &connection)
        .map(|channel| Json(channel))
        .map_err(|error| error_status(error))
}


 #[put("/<id>", format = "application/json")]
pub fn update_channel(id: &str, connection: DbConn) -> Result<Json<Channel>, Status> {
    let channel_uuid = Uuid::parse_str(id).unwrap();
    channels::repository::update_channel(channel_uuid, &connection)
        .map(|channel| Json(channel))
        .map_err(|error| error_status(error))
} 

#[delete("/<id>")]
pub fn delete_channel(id: &str, connection: DbConn) -> Result<status::NoContent, Status> {
    let channel_uuid = Uuid::parse_str(id).unwrap();
    channels::repository::delete_channel(channel_uuid, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

fn channel_created(channel: Channel) -> status::Created<Json<Channel>> {
    println!("here final");
    status::Created::new(format!(
        "{host}:{port}/channel/{id}",
        host = "localhost",
        port = "8000",
        id = channel.id
    ))
    .body(Json(channel))
}
