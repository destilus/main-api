#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::channels::model::NewChannel;
use crate::channels::model::Channel;

use crate::schema::channels;
use crate::schema::channels::dsl::*;

pub fn show_channels(connection: &PgConnection) -> QueryResult<Vec<Channel>> {
    channels.load::<Channel>(&*connection)
}

pub fn count_channels(connection: &PgConnection) -> String {
    let u = channels
        .limit(5)
        .load::<Channel>(&*connection)
        .map(|channel| channel.len());
    match u {
        Ok(qty) => format!("mock posts qty: {}", qty),
        Err(_) => String::from("Error getting mocks quantity"),
    }
}

pub fn create_channel(
    new_channel: NewChannel,
    conn: &PgConnection,
) -> QueryResult<Channel> {
    diesel::insert_into(channels::table)
        .values(&new_channel)
        .get_result(conn)
}


 pub fn update_channel(channel_id: i32, channel: Channel, connection: &PgConnection) -> QueryResult<Channel> {
    diesel::update(channels::table.find(channel_id))
        .set(&channel)
        .get_result(connection)
} 

pub fn get_channel(channel_id: i32, connection: &PgConnection) -> QueryResult<Channel> {
    channels::table.find(channel_id).get_result::<Channel>(connection)
}

pub fn delete_channel(channel_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(channels::table.find(channel_id)).execute(connection)
}
