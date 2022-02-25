use rocket;

use crate::connection;
use crate::curatorships;

pub async fn create_routes() {
    rocket::build()
        .manage(connection::get_pool())
        .mount(
            "/curatorships",
            routes![
                curatorships::handler::all_posts,
                curatorships::handler::count_posts,
                curatorships::handler::create_post,
                curatorships::handler::update_post,
                curatorships::handler::get_post,
                curatorships::handler::delete_post
            ],
        )
        .launch()
        .await
        .ok();
}
