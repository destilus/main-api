#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::curatorships::model::NewPost;
use crate::curatorships::model::Post;

use crate::schema::posts;
use crate::schema::posts::dsl::*;

pub fn show_posts(connection: &PgConnection) -> QueryResult<Vec<Post>> {
    //posts.filter(published.eq(true))
    posts.load::<Post>(&*connection)
}

pub fn count_posts(connection: &PgConnection) -> String {
    let u = posts
        .limit(5)
        .load::<Post>(&*connection)
        .map(|post| post.len());
    match u {
        Ok(qty) => format!("mock posts qty: {}", qty),
        Err(_) => String::from("Error getting mocks quantity"),
    }
}

pub fn create_post(new_post: NewPost, conn: &PgConnection) -> QueryResult<Post> {
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
}

pub fn update_post(post_id: i32, post: Post, connection: &PgConnection) -> QueryResult<Post> {
    diesel::update(posts::table.find(post_id))
        .set(&post)
        .get_result(connection)
}

pub fn get_post(post_id: i32, connection: &PgConnection) -> QueryResult<Post> {
    posts::table.find(post_id).get_result::<Post>(connection)
}

pub fn delete_post(post_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(posts::table.find(post_id)).execute(connection)
}
