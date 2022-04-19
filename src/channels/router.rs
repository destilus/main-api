use rocket;

use crate::connection;
use crate::channels;

pub async fn create_routes() {
    rocket::build()
        .manage(connection::get_pool())
        .mount(
            "/channels",
            routes![
                channels::handler::all_channels,
                channels::handler::count_channels,
                channels::handler::create_channel,
                channels::handler::update_channel,
                channels::handler::get_channel,
                channels::handler::delete_channel
            ],
        )
        .launch()
        .await
        .ok();
}
