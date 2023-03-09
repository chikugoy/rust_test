use diesel::prelude::*;
use chrono::NaiveDateTime;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Pav {
    pub id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub date: NaiveDate,
    pub value: f64,
    pub other_values: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub owner_id: Option<i64>,
}

#[derive(Queryable)]
#[diesel(table_name = pavs_insert_test)]
pub struct PavInsertTest {
    pub id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub date: NaiveDate,
    pub value: f64,
    pub other_values: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub owner_id: Option<i64>,
}

#[derive(Queryable)]
#[diesel(table_name = tmp_row_pavs)]
pub struct RowPav {
    pub id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub date: NaiveDate,
    pub value: f64,
    pub other_values: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub owner_id: Option<i64>,
}

#[derive(Queryable)]
pub struct TmpAccount {
    pub id: i64,
    pub account_id: Option<Uuid>,
    pub category_id: Option<i32>,
    pub coefficient: Option<i32>,
    pub group_id_array: Option<serde_json::Value>,
    pub profit_id_array: Option<serde_json::Value>,
}

#[derive(Queryable)]
pub struct TmpBudgetPav {
    pub id: i64,
    pub budget_id: i64,
    pub identity_id: i64,
    pub account_id: Option<Uuid>,
    pub unit_id: Option<i64>,
    pub data_set_id: Option<i64>,
    pub date: NaiveDate,
    pub budget: Option<f64>,
    pub forecast: Option<f64>,
    pub achievement: Option<f64>,
    pub calculated: Option<bool>,
}

#[derive(Queryable)]
pub struct TmpRowPav {
    pub id: i64,
    pub row_id: i64,
    pub budget_id: Option<i64>,
    pub identity_id: Option<i64>,
    pub account_id: Option<Uuid>,
    pub unit_id: Option<i64>,
    pub data_set_id: Option<i64>,
    pub date: NaiveDate,
    pub formula_json: Option<serde_json::Value>,
    pub value: Option<f64>,
    pub calculated: Option<bool>,
    pub is_not_aggregate: Option<bool>,
}
