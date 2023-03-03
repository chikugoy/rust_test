use diesel::prelude::*;
use chrono::NaiveDateTime;
use chrono::NaiveDate;

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