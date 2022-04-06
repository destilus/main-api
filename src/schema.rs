table! {
    bets (id) {
        id -> Uuid,
        creator_id -> Uuid,
        title -> Varchar,
        summary -> Nullable<Varchar>,
        curr_status -> Varchar,
        category -> Varchar,
        bids_limit_date -> Timestamp,
        result_released_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    channels (id) {
        id -> Uuid,
        owner_id -> Uuid,
        title -> Varchar,
        summary -> Nullable<Varchar>,
        frequency -> Varchar,
        exclusivity -> Varchar,
        subscription_price -> Nullable<Numeric>,
        price_currency -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    curatorship_items (id) {
        id -> Uuid,
        curatorship_id -> Uuid,
        priority_order -> Int2,
        external_ref -> Nullable<Varchar>,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        hero_image_url -> Nullable<Varchar>,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    curatorships (id) {
        id -> Uuid,
        curator_id -> Uuid,
        defied_curatorship_id -> Nullable<Uuid>,
        channel_id -> Nullable<Uuid>,
        curator_bet_id -> Nullable<Uuid>,
        title -> Varchar,
        subtitle -> Varchar,
        hero_image_url -> Varchar,
        body -> Varchar,
        curr_status -> Varchar,
        category -> Varchar,
        exclusivity -> Varchar,
        priority_order -> Varchar,
        price_currency -> Nullable<Varchar>,
        single_price -> Nullable<Numeric>,
        verified -> Bool,
        previews_count -> Int2,
        published_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Int4,
        curator_id -> Nullable<Uuid>,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        pwd -> Varchar,
        verified -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(bets -> users (creator_id));
joinable!(channels -> users (owner_id));
joinable!(curatorship_items -> curatorships (curatorship_id));
joinable!(curatorships -> bets (curator_bet_id));
joinable!(curatorships -> channels (channel_id));
joinable!(curatorships -> users (curator_id));
joinable!(posts -> users (curator_id));

allow_tables_to_appear_in_same_query!(
    bets,
    channels,
    curatorship_items,
    curatorships,
    posts,
    users,
);
