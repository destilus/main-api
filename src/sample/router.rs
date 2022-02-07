use rocket;

use crate::connection;
use crate::sample;

pub async fn create_routes() {
    rocket::build()
        .manage(connection::get_pool())
        .mount(
            "/",
            routes![sample::handler::all_posts, sample::handler::create_post],
        )
        .launch()
        .await
        .ok();
}
