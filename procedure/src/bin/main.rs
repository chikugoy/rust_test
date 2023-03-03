extern crate diesel;

use csv;
use std::error::Error;
use std::time::Instant;
use std::path::Path;
use thousands::Separable;

use aws_config::meta::region::RegionProviderChain;
use aws_smithy_http::byte_stream::ByteStream;

use self::models::*;
use diesel::prelude::*;
use procedure::*;

fn write_to_csv(path: &str, results: Vec<procedure::models::Pav>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(&[
        "id",
        "target_type",
    ])?;

    for rec in results {
        writer.write_record(&[
            &rec.id.to_string(),
            &rec.target_type.to_string(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

async fn upload_to_s3(path: &str) {
    let bucket_name: String = "cabernet-performance-rds-s3-test".to_string();

    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1"); // 利用しているリージョンの設定
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_s3::Client::new(&config);

    let body = ByteStream::from_path(Path::new(path)).await.unwrap();

    let resp = client.put_object()
        .bucket(&bucket_name)
        .key("test.csv")
        .body(body)
        .send()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    use self::schema::pavs::dsl::*;
    let connection = &mut establish_connection();

    let mut start = Instant::now();
    let results = pavs
        .load::<Pav>(connection)
        .expect("Error loading pavs");
    let mut end = start.elapsed();

    println!("pavs count {}", results.len().separate_with_commas());
    println!("pavs get: {}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    const PATH: &str = "./output/test.csv";

    start = Instant::now();
    if let Err(e) = write_to_csv(PATH, results) {
        eprintln!("{}", e)
    }
    end = start.elapsed();
    println!("csv output: {}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    start = Instant::now();
    upload_to_s3(PATH).await;
    end = start.elapsed();
    println!("csv s3 upload: {}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);
}