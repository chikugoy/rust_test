pub mod models;
pub mod schema;
pub mod calculated_cache;
pub mod record_cache;
pub mod calc;
pub mod any_map;
pub mod formula_result;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn cache_key(key: &String) -> String {
    if type_of(&key) == "" {

    }
    return key.to_string();
}

pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}


