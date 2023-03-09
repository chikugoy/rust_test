extern crate diesel;

use csv;
use std::error::Error;
use std::time::Instant;
use std::path::Path;
use diesel::prelude::*;
use thousands::Separable;
use aws_config::meta::region::RegionProviderChain;
use aws_smithy_http::byte_stream::ByteStream;

use self::models::*;
use procedure::*;

fn write_to_csv(path: &str, results: Vec<procedure::models::Pav>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(&[
        "id",
        "target_type",
        "owner_id",
    ])?;

    for rec in results {
        writer.write_record(&[
            &rec.id.to_string(),
            &rec.target_type.to_string(),
            &rec.owner_id.expect("null").to_string(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

async fn upload_to_s3(path: &str) {
    let bucket_name: String = "cabernet-performance-rds-s3-test".to_string();

    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_s3::Client::new(&config);

    let body = ByteStream::from_path(Path::new(path)).await.unwrap();

    client.put_object()
        .bucket(&bucket_name)
        .key("test.csv")
        .body(body)
        .send()
        .await
        .unwrap();
}

// pub struct PavInsertTest {
//     pub id: i64,
//     pub target_type: String,
//     pub target_id: i64,
//     pub date: NaiveDate,
//     pub value: f64,
//     pub other_values: serde_json::Value,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
//     pub owner_id: Option<i64>,
// }

fn insert_pavs(results: Vec<procedure::models::Pav>) {
    use self::schema::pavs_insert_test::dsl::*;
    for rec in results {
        let pav = procedure::models::PavInsertTest {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        let query = diesel::insert_into(pavs_insert_test).values(pav);
        println!("{}", diesel::debug_query::<procedure::models::PavInsertTest, _>(&query));
    }
}

#[tokio::main]
async fn main() {
    use self::schema::pavs::dsl::*;
    let connection = &mut establish_connection();

    let mut start = Instant::now();
    let results = pavs
        // .filter(owner_id.eq(10))
        // .limit(5)
        .load::<Pav>(connection)
        .expect("Error loading pavs");
    let mut end = start.elapsed();

    println!("pavs count {}", results.len().separate_with_commas());
    println!("pavs get: {}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    const PATH: &str = "./output/test.csv";

    // start = Instant::now();
    // if let Err(e) = write_to_csv(PATH, results) {
    //     eprintln!("{}", e)
    // }
    // end = start.elapsed();
    // println!("csv output: {}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);
    //
    // start = Instant::now();
    // upload_to_s3(PATH).await;
    // end = start.elapsed();
    // println!("csv s3 upload: {}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    start = Instant::now();
    insert_pavs(results);
    end = start.elapsed();
    println!("pavs insert: {}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);
}