// @generated automatically by Diesel CLI.

diesel::table! {
    cabernet_chikugo_20221207.pavs (id) {
        id -> Int8,
        target_type -> Varchar,
        target_id -> Int8,
        date -> Date,
        value -> Float8,
        other_values -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        owner_id -> Nullable<Int8>,
    }
}
