#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::channels;
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
pub struct Channel {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub frequency: String,
    pub exclusivity: String,
    pub subscription_price: Option<BigDecimal> ,
    pub price_currency: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "channels"]
pub struct NewChannel<'a> {
    pub owner_id: Uuid,
    pub title: &'a str,
    pub summary: Option<&'a str>,
    pub frequency: &'a str,
    pub exclusivity: &'a str,
    pub subscription_price: Option<BigDecimal>,
    pub price_currency: Option<&'a str>,
}

#[derive(Serialize, Deserialize)]
pub struct NewChannelDto<'a> {
    pub owner_id: &'a str,
    pub title: &'a str,
    pub summary: Option<&'a str>,
    pub frequency: &'a str,
    pub exclusivity: &'a str,
    pub subscription_price: Option<BigDecimal>,
    pub price_currency: Option<&'a str>,
}