use rocket;

use crate::connection;
use crate::sample;

pub async fn create_routes() {
    rocket::build()
        .manage(connection::get_pool())
        .mount("/posts", routes![sample::handler::all_posts])
        .launch()
        .await
        .ok();
}
