#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::curatorship_items;
use crate::schema::curatorships;
use crate::schema::posts;
use bigdecimal::BigDecimal;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub curator_id: Option<Uuid>,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub curator_id: Option<Uuid>,
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Curatorship {
    pub id: Uuid,
    pub curator_id: Uuid,
    pub defied_curatorship_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
    pub curator_bet_id: Option<Uuid>,
    pub title: String,
    pub subtitle: String,
    pub hero_image_url: String,
    pub body: String,
    pub curr_status: String,
    pub category: String,
    pub exclusivity: String,
    pub priority_order: String,         // change to enum
    pub price_currency: Option<String>, // change to enum
    pub single_price: Option<BigDecimal>,
    pub verified: bool,
    pub previews_count: i16,
    pub published_at: Option<SystemTime>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct CuratorshipItem {
    pub id: Uuid,
    pub curatorship_id: Uuid,
    pub priority_order: i16,
    pub external_ref: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub hero_image_url: Option<String>,
    pub body: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Serialize)]
pub struct CompleteCuratorship {
    pub curatorship: Curatorship,
    pub curatorship_items: Vec<CuratorshipItem>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "curatorships"]
pub struct NewCuratorship<'a> {
    pub curator_id: Uuid,
    pub defied_curatorship_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
    pub curator_bet_id: Option<Uuid>,
    pub title: &'a str,
    pub subtitle: &'a str,
    pub hero_image_url: &'a str,
    pub body: &'a str,
    pub curr_status: &'a str,
    pub category: &'a str,
    pub exclusivity: &'a str,
    pub priority_order: &'a str,         // change to enum
    pub price_currency: Option<&'a str>, // change to enum
    pub single_price: Option<BigDecimal>,
    pub previews_count: i16,
    pub verified: bool,
    pub published_at: Option<SystemTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "curatorship_items"]
pub struct NewCuratorshipItem<'a> {
    pub curatorship_id: Uuid,
    pub priority_order: i16,
    pub external_ref: Option<&'a str>,
    pub title: &'a str,
    pub subtitle: Option<&'a str>,
    pub hero_image_url: Option<&'a str>,
    pub body: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct NewCuratorshipDto<'a> {
    pub curator_id: &'a str,
    pub defied_curatorship_id: Option<&'a str>,
    pub channel_id: Option<&'a str>,
    pub curator_bet_id: Option<&'a str>,
    pub metadata: CuratorshipMetadata<'a>,
    pub curr_status: &'a str,
    pub category: &'a str,
    pub frequency: &'a str,
    pub exclusivity: &'a str,
    pub price_currency: Option<&'a str>, // change to enum
    pub single_price: Option<BigDecimal>,
    pub previews_count: i16,
    pub priority_order: &'a str, // change to enum
    pub verified: bool,
    pub items: Vec<NewCutarshipItemDto<'a>>,
}

#[derive(Serialize, Deserialize)]
pub struct NewCutarshipItemDto<'a> {
    pub external_ref: Option<&'a str>,
    pub priority: i16,
    pub metadata: CuratorshipItemMetadata<'a>,
}

#[derive(Serialize, Deserialize)]
pub struct CuratorshipMetadata<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub text: &'a str,
    pub hero_image: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct CuratorshipItemMetadata<'a> {
    pub title: &'a str,
    pub subtitle: Option<&'a str>,
    pub text: &'a str,
    pub hero_image: Option<&'a str>,
}
