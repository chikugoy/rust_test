// @generated automatically by Diesel CLI.

diesel::table! {
    // cabernet_zakky.pavs (id) {
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

diesel::table! {
    // cabernet_zakky.pavs (id) {
    cabernet_chikugo_20221207.pavs_insert_test (id) {
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


diesel::table! {
    cabernet_chikugo_20221207.tmp_accounts (id) {
        id -> Int8,
        account_id -> Nullable<Uuid>,
        category_id -> Nullable<Int4>,
        coefficient -> Nullable<Int4>,
        group_id_array -> Nullable<Jsonb>,
        profit_id_array -> Nullable<Jsonb>,
    }
}

diesel::table! {
    cabernet_chikugo_20221207.tmp_budget_pavs (id) {
        id -> Int8,
        budget_id -> Int8,
        identity_id -> Int8,
        account_id -> Nullable<Uuid>,
        unit_id -> Nullable<Int8>,
        data_set_id -> Nullable<Int8>,
        date -> Date,
        budget -> Nullable<Float8>,
        forecast -> Nullable<Float8>,
        achievement -> Nullable<Float8>,
        calculated -> Nullable<Bool>,
    }
}

diesel::table! {
    cabernet_chikugo_20221207.tmp_row_pavs (id) {
        id -> Int8,
        row_id -> Int8,
        budget_id -> Nullable<Int8>,
        identity_id -> Nullable<Int8>,
        account_id -> Nullable<Uuid>,
        unit_id -> Nullable<Int8>,
        data_set_id -> Nullable<Int8>,
        date -> Date,
        formula_json -> Nullable<Jsonb>,
        value -> Nullable<Float8>,
        calculated -> Nullable<Bool>,
        is_not_aggregate -> Nullable<Bool>,
    }
}