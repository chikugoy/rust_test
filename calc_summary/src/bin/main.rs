extern crate diesel;

use csv;
use std::error::Error;
use std::time::Instant;
use std::path::Path;
use std::collections::HashMap;
use std::any::*;

use diesel::prelude::*;
use thousands::Separable;
use aws_config::meta::region::RegionProviderChain;
use aws_smithy_http::byte_stream::ByteStream;

use self::models::*;
use self::any_map::*;
use calc_summary::*;

fn get_generate_cache() -> AnyMap {
    let target_dates = [
        "2022-04-01",
        "2022-05-01",
        "2022-06-01",
        "2022-07-01",
        "2022-08-01",
        "2022-09-01",
        "2022-10-01",
        "2022-11-01",
        "2022-12-01",
        "2023-01-01",
        "2023-02-01",
        "2023-03-01",
    ];

    // FIXME: rustではHashMapにHashMapを入れ子にするようなコードは余り筋がよくないかも
    // 何でも入るHashMapを実装している人はいたが、安定版ではない features機能を使っている
    // http://benfalk.com/blog/2022/02/27/rust-hashmap-to-store-anything/
    let mut generate_cache = AnyMap::new();
    for v in target_dates.iter() {
        generate_cache.set(v.to_string(), AnyMap::new());
    }

    return generate_cache;
}

fn make_cache(generate_cache: &AnyMap) {
    let mut selects: Vec<String> = vec![];
    let mut budget_row = generate_cache.clone();

    println!("START cache row");

    use self::schema::tmp_row_pavs::dsl::*;
    let connection = &mut establish_connection();

    let results = tmp_row_pavs
        .load::<TmpRowPav>(connection)
        .expect("Error loading tmp row pavs");
    println!("pavs count {}", results.len().separate_with_commas());

    for rec in results {
        let row_date: &String = &rec.date.format("%Y-%m-%d").to_string();
        println!("{}", row_date);
        if !budget_row.get::<AnyMap>(row_date.to_string()).unwrap().has::<AnyMap>(rec.budget_id.expect("").to_string()) {
            let mut budget_row_data = AnyMap::new();
            budget_row_data.set("row_id".to_string(), i64::from(-1));
            budget_row_data.set("date".to_string(), rec.date);
            budget_row_data.set("calculated".to_string(), true);
            budget_row.get::<AnyMap>(row_date.to_string()).unwrap().set::<&AnyMap>(rec.budget_id.expect("").to_string(), &mut budget_row_data);
            // println!("budget_row_data {:?}", budget_row_data);
            // println!("budget_row {:?}", budget_row);
        }

        break;
    }


    // pub struct TmpRowPav {
    //     pub id: i64,
    //     pub row_id: i64,
    //     pub budget_id: Option<i64>,
    //     pub identity_id: Option<i64>,
    //     pub account_id: Option<Uuid>,
    //     pub unit_id: Option<i64>,
    //     pub data_set_id: Option<i64>,
    //     pub date: NaiveDate,
    //     pub formula_json: Option<serde_json::Value>,
    //     pub value: Option<f64>,
    //     pub calculated: Option<bool>,
    //     pub is_not_aggregate: Option<bool>,
    // }

    // selects = [
    //     'id',
    //     'row_id',
    //     'budget_id',
    //     'unit_id',
    // 'COALESCE(value, 0.0) AS value',
    // `TO_CHAR(date, '${DATE_FORMAT}') AS date`,
    //          'formula_json',
    //          'is_not_aggregate',
    //          'calculated',
    //          ];
    // for (var r of plv8.execute(`SELECT ${selects.join(',')} FROM tmp_row_pavs`)) {
    //     var date = r.date;
    //     if (!budget_row[date].has(r.budget_id)) budget_row[date].set(r.budget_id, [{ row_id: -1, date: date, value: 0.0, calculated: true }]);
    //
    //     if (r.calculated) {
    //         calculated_ids.set('row', date, r.row_id, r.value);
    //         if (!r.is_not_aggregate) budget_row[date].get(r.budget_id)[0].value = calc.add(budget_row[date].get(r.budget_id)[0].value, r.value);
    //     } else {
    //         if (!r.is_not_aggregate) budget_row[date].get(r.budget_id).push(r);
    //         records.set('row', date, r.row_id, r);
    //     }
    // }
    // logger.print('FINISH cache row');
    //
    // logger.print('START cache budget');
    // selects = [
    //     'id',
    //     'budget_id',
    //     'account_id',
    //     'unit_id',
    //     'budget',
    // 'forecast AS value',
    // `TO_CHAR(date, '${DATE_FORMAT}') AS date`,
    //          'calculated',
    //          ];
    // for (var r of plv8.execute(`SELECT ${selects.join(',')} FROM tmp_budget_pavs b`)) {
    //     // 上書き値を考慮した値を取得するseen_outside_valueを定義しておく
    //     Object.defineProperty(r, 'seen_outside_value', { get: function () { return this.budget === null ? this.value : this.budget; } });
    //     var date = r.date;
    //     var aukey = [r.account_id, r.unit_id];
    //     if (!records.has('account_unit', date, aukey)) records.set('account_unit', date, aukey, [{ budget_id: -1, date: date, value: 0.0, calculated: true }]);
    //
    //     if (r.calculated) {
    //         calculated_ids.set('budget', date, r.budget_id, r.seen_outside_value);
    //         records.get('account_unit', date, aukey)[0].value = calc.add(records.get('account_unit', date, aukey)[0].value, r.seen_outside_value);
    //     } else {
    //         records.get('account_unit', date, aukey).push(r);
    //         r.rows = budget_row[date].get(r.budget_id);
    //         records.set('budget', date, r.budget_id, r);
    //     }
    // }
    // logger.print('FINISH cache budget');
    //
    // logger.print('START cache account');
    // for (var r of plv8.execute(`SELECT * FROM tmp_accounts a`)) {
    //     for (var date of target_dates) {
    //         for (var p of r.profit_id_array) {
    //             if (!records.has('profit', date, p)) records.set('profit', date, p, []);
    //             records.get('profit', date, p).push(r);
    //         }
    //         for (var n of r.group_id_array) {
    //             if (!records.has('group', date, n)) records.set('group', date, n, []);
    //             records.get('group', date, n).push(r);
    //         }
    //         if (!records.has('category', date, r.category_id)) records.set('category', date, r.category_id, []);
    //         records.get('category', date, r.category_id).push(r);
    //         records.set('account', date, r.account_id, [r]);
    //     }
    // }
    // logger.print('FINISH cache account');

}

#[tokio::main]
async fn main() {
    // use self::schema::pavs::dsl::*;
    // let connection = &mut establish_connection();
    //
    // let mut start = Instant::now();
    // let results = pavs
    //     // .filter(owner_id.eq(10))
    //     // .limit(5)
    //     .load::<Pav>(connection)
    //     .expect("Error loading pavs");
    // let mut end = start.elapsed();


    // // vec
    // let mut vec = vec![];
    // let d1 = vec![];
    // let d2 = vec![];
    //
    // vec.push(d1);
    // vec.push(d2);
    //
    // vec[0].push(1);
    // vec[0].push(2);
    // vec[0].push(3);
    // vec[1].push(5);
    // vec[1].push(6);
    //
    // println!("d1: {:?}",vec[0]);
    // println!("d2: {:?}",vec[1]);
    //
    // println!("d1[2]:{}",vec[0][2]);


    // // hash
    // let mut calculated_cache = HashMap::new();
    // let mut calculated_cache_data = HashMap::new();
    //
    // calculated_cache_data.insert(String::from("2022-01-01"), String::from("test_data1"));
    // calculated_cache.insert(String::from("type1"), calculated_cache_data);
    //
    // println!("{:?}", calculated_cache);
    // println!("{:?}", calculated_cache.get(&String::from("type1")).unwrap().get(&String::from("2022-01-01")).unwrap());
    // _generate_cache.get_mut(&String::from("2022-04-01")).unwrap().insert(String::from("type1"), String::from("type11"));

    let mut generate_cache = get_generate_cache();

    make_cache(&generate_cache);




    //
    // println!("pavs count {}", results.len().separate_with_commas());
    // println!("pavs get: {}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);
    //
    // const PATH: &str = "./output/test.csv";
    //
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
}