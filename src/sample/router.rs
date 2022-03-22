// use rocket;

// use crate::connection;
// use crate::sample;

// pub async fn create_routes() {
//     rocket::build()
//         .manage(connection::get_pool())
//         .mount(
//             "/sample",
//             routes![
//                 sample::handler::all_posts,
//                 sample::handler::count_posts,
//                 sample::handler::create_post,
//                 sample::handler::update_post,
//                 sample::handler::get_post,
//                 sample::handler::delete_post
//             ],
//         )
//         .launch()
//         .await
//         .ok();
// }
