use rocket;

use crate::connection;
use crate::channels;

pub async fn create_routes() {
    rocket::build()
        .manage(connection::get_pool())
        .mount(
            "/channels",
            routes![
                channels::handler::all_posts,
                channels::handler::count_posts,
                channels::handler::create_channel,
                channels::handler::create_post,
                channels::handler::update_post,
                channels::handler::get_post,
                channels::handler::delete_post
            ],
        )
        .launch()
        .await
        .ok();
}
