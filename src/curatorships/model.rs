#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::curatorships;
use crate::schema::posts;
use bigdecimal::BigDecimal;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "curatorships"]
pub struct NewCuratorship<'a> {
    pub curator_id: Uuid,
    pub defied_curatorship_id: Uuid,
    pub title: &'a str,
    pub subtitle: &'a str,
    pub hero_image_url: &'a str,
    pub curr_status: &'a str,
    pub category: &'a str,
    pub frequency: &'a str,
    pub exclusivity: &'a str,
    pub single_price: Option<BigDecimal>,
    pub price_currency: Option<&'a str>, // change to enum
    pub previews_count: i16,
    pub priority_order: Option<&'a str>, // change to enum
    pub verified: bool,
    pub published_at: Option<SystemTime>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize)]
pub struct NewCuratorshipDto<'a> {
    pub curator_id: &'a str,
    pub defied_curatorship_id: &'a str,
    pub metadata: CuratorshipMetadata<'a>,
    pub title: &'a str,
    pub subtitle: &'a str,
    pub hero_image_url: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct CuratorshipMetadata<'a> {
    pub title: &'a str,
    pub subtitle: Option<&'a str>,
    pub text: Option<&'a str>,
    pub hero_image: &'a str,
}

// channel_id -> Uuid,
// curator_bet_id -> Nullable<Uuid>,
// title -> Varchar,
// subtitle -> Nullable<Varchar>,
// hero_image_url -> Varchar,
// body -> Nullable<Text>,
// curr_status -> Varchar,
// category -> Varchar,
// frequency -> Varchar,
// exclusivity -> Varchar,
// single_price -> Nullable<Numeric>,
// price_currency -> Nullable<Varchar>,
// previews_count -> Int2,
// priority_order -> Varchar,
// verified -> Bool,
// published_at -> Nullable<Timestamp>,
// created_at -> Timestamp,
// updated_at -> Timestamp,
