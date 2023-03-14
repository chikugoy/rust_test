extern crate diesel;

use csv;
use std::env;
use std::error::Error;
use std::time::Instant;
use diesel::prelude::*;
use thousands::Separable;

use self::models::*;
use self::any_map::*;
use self::record_cache::*;
use self::formula_result::*;
use calc_summary::*;

fn make_cache(target_dates: [&str;12], record_cache: &mut Records) {
    let mut budget_row = AnyMap::generate_cache();

    println!("START cache row");

    use self::schema::tmp_row_pavs::dsl::*;
    use self::schema::tmp_budget_pavs::dsl::*;
    use self::schema::tmp_accounts::dsl::*;
    let connection = &mut establish_connection();

    let results = tmp_row_pavs
        .load::<TmpRowPav>(connection)
        .expect("Error loading tmp row pavs");
    println!("tmp row pavs count {}", results.len().separate_with_commas());

    let start = Instant::now();

    for rec in results {
        let row_date: &String = &rec.date.format("%Y-%m-%d").to_string();
        let target_budget_id = if !rec.budget_id.is_none() { rec.budget_id.expect("").to_string() } else { "".to_string() };

        if !budget_row.get::<AnyMap>(row_date.to_string()).unwrap().has::<AnyMap>(target_budget_id.clone()) {
            let mut budget_row_data = AnyMap::new();
            budget_row_data.set("row_id".to_string(), i64::from(-1));
            budget_row_data.set("date".to_string(), rec.date);
            budget_row_data.set("value".to_string(), f64::from(0));
            budget_row_data.set("calculated".to_string(), true);
            budget_row.get_mut::<AnyMap>(row_date.to_string()).unwrap().set::<Vec<AnyMap>>(target_budget_id.clone(), vec![budget_row_data]);
        }

        if !rec.is_not_aggregate.unwrap() {
            let mut budget_row_data = AnyMap::new();
            budget_row_data.set("row_id".to_string(), rec.row_id);
            budget_row_data.set("date".to_string(), rec.date);
            budget_row_data.set("value".to_string(), rec.value.unwrap());
            budget_row_data.set("calculated".to_string(), rec.calculated.unwrap());

            let target_row = budget_row.get_mut::<AnyMap>(row_date.to_string()).unwrap().get_mut::<Vec<AnyMap>>(target_budget_id.clone()).unwrap();
            target_row.push(budget_row_data);
        }
        let mut row_data = AnyMap::new();
        row_data.set("row_id".to_string(), rec.row_id);
        row_data.set("date".to_string(), rec.date);
        row_data.set("value".to_string(), if rec.value.is_none() { f64::from(0) } else { rec.value.unwrap() } );
        row_data.set("calculated".to_string(), rec.calculated.unwrap());
        row_data.set("formula_json".to_string(), if rec.formula_json.is_none() { None } else { Some(rec.formula_json.unwrap()) } );
        row_data.set("is_not_aggregate".to_string(), rec.is_not_aggregate.unwrap());

        record_cache.set::<AnyMap>("row".to_string(), row_date.to_string(), rec.row_id.to_string(), row_data);
    }

    let end = start.elapsed();
    println!("make cache row: {}.{:01}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    println!("FINISH cache row");

    println!("START cache budget");

    let results = tmp_budget_pavs
        .load::<TmpBudgetPav>(connection)
        .expect("Error loading tmp row budgets");
    println!("tmp row budgets count {}", results.len().separate_with_commas());

    let start = Instant::now();

    for rec in results {
        let row_date: &String = &rec.date.format("%Y-%m-%d").to_string();
        let target_account_id = if !rec.account_id.is_none() { rec.account_id.expect("").to_string() } else { "".to_string() };
        let target_unit_id = if !rec.unit_id.is_none() { rec.unit_id.expect("").to_string() } else { "".to_string() };
        let aukey = target_account_id.to_string() + &"-".to_string() + &target_unit_id.to_string();

        if !record_cache.has::<Vec<AnyMap>>("account_unit".to_string(), row_date.to_string(), aukey.to_string()) {
            let mut account_unit_data = AnyMap::new();
            account_unit_data.set("budget_id".to_string(), i64::from(-1));
            account_unit_data.set("date".to_string(), rec.date);
            account_unit_data.set("value".to_string(), f64::from(0));
            account_unit_data.set("budget".to_string(), f64::from(0));
            account_unit_data.set("forecast".to_string(), f64::from(0));
            account_unit_data.set("achievement".to_string(), f64::from(0));
            account_unit_data.set("calculated".to_string(), true);
            record_cache.set::<Vec<AnyMap>>("account_unit".to_string(), row_date.to_string(), aukey.to_string(), vec![account_unit_data]);
        }

        let mut account_unit_data = AnyMap::new();
        account_unit_data.set("budget_id".to_string(), rec.budget_id);
        account_unit_data.set("date".to_string(), rec.date);
        account_unit_data.set("budget".to_string(), if rec.budget.is_none() { f64::from(0) } else {rec.budget.unwrap()});
        account_unit_data.set("forecast".to_string(), rec.forecast.unwrap());
        account_unit_data.set("achievement".to_string(), rec.achievement.unwrap());
        account_unit_data.set("calculated".to_string(), rec.calculated.unwrap());
        let target_row = record_cache.get_mut::<Vec<AnyMap>>("account_unit".to_string(), row_date.to_string(), aukey.to_string()).unwrap();
        target_row.push(account_unit_data);

        let mut budget_data = AnyMap::new();
        budget_data.set("budget_id".to_string(), rec.budget_id);
        budget_data.set("date".to_string(), rec.date);
        budget_data.set("budget".to_string(), if rec.budget.is_none() { f64::from(0) } else {rec.budget.unwrap()});
        budget_data.set("forecast".to_string(), rec.forecast.unwrap());
        budget_data.set("achievement".to_string(), rec.achievement.unwrap());
        budget_data.set("calculated".to_string(), rec.calculated.unwrap());
        record_cache.set::<AnyMap>("budget".to_string(), row_date.to_string(), rec.budget_id.to_string(), budget_data);
    }

    let end = start.elapsed();
    println!("make cache budget: {}.{:01}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    println!("FINISH cache budget");

    println!("START cache account");

    let results = tmp_accounts
        .load::<TmpAccount>(connection)
        .expect("Error loading tmp row accounts");
    println!("tmp row accounts count {}", results.len().separate_with_commas());

    let start = Instant::now();

    for rec in results {
        for profit_id in rec.profit_id_array.clone().unwrap().as_array().unwrap() {
            for target_date in target_dates {
                if !record_cache.has::<Vec<AnyMap>>("profit".to_string(), target_date.to_string(), profit_id.to_string()) {
                    record_cache.set::<Vec<AnyMap>>("profit".to_string(), target_date.to_string(), profit_id.to_string(), vec![]);
                }
                let target_row = record_cache.get_mut::<Vec<AnyMap>>("profit".to_string(), target_date.to_string(), profit_id.to_string()).unwrap();
                let mut account_data = AnyMap::new();
                account_data.set("account_id".to_string(), rec.account_id.unwrap());
                account_data.set("category_id".to_string(), rec.category_id.unwrap());
                account_data.set("coefficient".to_string(), rec.coefficient);
                account_data.set("group_id_array".to_string(), rec.group_id_array.clone().unwrap());
                account_data.set("profit_id_array".to_string(), rec.profit_id_array.clone().unwrap());
                target_row.push(account_data);
            }
        }

        for group_id in rec.group_id_array.clone().unwrap().as_array().unwrap() {
            for target_date in target_dates {
                if !record_cache.has::<Vec<AnyMap>>("group".to_string(), target_date.to_string(), group_id.to_string()) {
                    record_cache.set::<Vec<AnyMap>>("group".to_string(), target_date.to_string(), group_id.to_string(), vec![]);
                }
                let target_row = record_cache.get_mut::<Vec<AnyMap>>("group".to_string(), target_date.to_string(), group_id.to_string()).unwrap();
                let mut account_data = AnyMap::new();
                account_data.set("account_id".to_string(), rec.account_id.unwrap());
                account_data.set("category_id".to_string(), rec.category_id.unwrap());
                account_data.set("coefficient".to_string(), rec.coefficient.unwrap());
                account_data.set("group_id_array".to_string(), rec.group_id_array.clone().unwrap());
                account_data.set("profit_id_array".to_string(), rec.profit_id_array.clone().unwrap());
                target_row.push(account_data);
            }
        }

        let target_category_id = if !rec.category_id.is_none() { rec.category_id.expect("").to_string() } else { "".to_string() };
        let target_account_id = if !rec.account_id.is_none() { rec.account_id.unwrap().to_string() } else { "".to_string() };

        for target_date in target_dates {
            if !record_cache.has::<Vec<AnyMap>>("category".to_string(), target_date.to_string(), target_category_id.to_string()) {
                record_cache.set::<Vec<AnyMap>>("category".to_string(), target_date.to_string(), target_category_id.to_string(), vec![]);
            }
            let target_row = record_cache.get_mut::<Vec<AnyMap>>("category".to_string(), target_date.to_string(), target_category_id.to_string()).unwrap();
            let mut account_data = AnyMap::new();
            account_data.set("account_id".to_string(), rec.account_id.unwrap());
            account_data.set("category_id".to_string(), rec.category_id.unwrap());
            account_data.set("coefficient".to_string(), rec.coefficient.unwrap());
            account_data.set("group_id_array".to_string(), rec.group_id_array.clone().unwrap());
            account_data.set("profit_id_array".to_string(), rec.profit_id_array.clone().unwrap());
            target_row.push(account_data);

            let mut account_data = AnyMap::new();
            account_data.set("account_id".to_string(), rec.account_id.unwrap());
            account_data.set("category_id".to_string(), rec.category_id.unwrap());
            account_data.set("coefficient".to_string(), rec.coefficient.unwrap());
            account_data.set("group_id_array".to_string(), rec.group_id_array.clone().unwrap());
            account_data.set("profit_id_array".to_string(), rec.profit_id_array.clone().unwrap());
            record_cache.set::<Vec<AnyMap>>("account".to_string(), target_date.to_string(), target_account_id.to_string(), vec![account_data]);
        }
    }

    let end = start.elapsed();
    println!("make cache account: {}.{:01}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    println!("FINISH cache account");
}

fn calculate_budget_rec(records: &mut Vec<AnyMap>, formula_results: &mut FormulaResults, target_date: String, key: String) {
    let mut calculated_value = f64::from(0);
    let mut is_not_calculated = false;

    for rec in records {
        if rec.get::<bool>("calculated".to_string()).is_none() || *rec.get::<bool>("calculated".to_string()).unwrap() {
            continue;
        }

        is_not_calculated = true;
        calculated_value = calculated_value + (*rec.get::<f64>("forecast".to_string()).unwrap() * 1.05);
    }

    if !is_not_calculated {
        return;
    }

    let data = FormulaResultData {
        date: target_date.clone(),
        key: key.clone(),
        value: Some(calculated_value),
    };
    formula_results.push("budget".to_string(), data);
}

fn calculate_row_rec(rec: &mut AnyMap, formula_results: &mut FormulaResults, target_date: String, key: String) {
    if !rec.get::<bool>("calculated".to_string()).is_none() && *rec.get::<bool>("calculated".to_string()).unwrap() {
        return;
    }

    let calculated_value = *rec.get::<f64>("value".to_string()).unwrap() * 1.05;
    let data = FormulaResultData {
        date: target_date.clone(),
        key: key.clone(),
        value: Some(calculated_value),
    };
    formula_results.push("row".to_string(), data);
    return;
}

fn calculate(target_dates: [&str;12], record_cache: &mut Records, formula_results: &mut FormulaResults) {
    for target_date in target_dates {
        // row calculate
        let row_keys = record_cache.get_type_data_keys("row".to_string(), target_date.to_string());
        for key in row_keys {
            let rec = record_cache.get_mut::<AnyMap>("row".to_string(), target_date.to_string(), key.to_string()).unwrap();
            calculate_row_rec(rec, formula_results, target_date.to_string(), key.to_string());
        }

        // budget calculate
        let budget_keys = record_cache.get_type_data_keys("account_unit".to_string(), target_date.to_string());
        for key in budget_keys {
            let records = record_cache.get_mut::<Vec<AnyMap>>("account_unit".to_string(), target_date.to_string(), key.to_string()).unwrap();
            if records.len() == 0 { continue }
            calculate_budget_rec(records, formula_results, target_date.to_string(), key.to_string());
        }
    }
}

fn calculate_summary(target_dates: [&str;12], record_cache: &mut Records, formula_results: &mut FormulaResults) {
    // ====================================
    // row calculate
    let mut row_summary_cache = AnyMap::new();

    for target_date in target_dates {
        let row_keys = record_cache.get_type_data_keys("row".to_string(), target_date.to_string());
        for key in row_keys {
            let rec = record_cache.get_mut::<AnyMap>("row".to_string(), target_date.to_string(), key.to_string()).unwrap();
            if !row_summary_cache.has::<f64>(key.to_string()) {
                row_summary_cache.set::<f64>(key.to_string(), *rec.get::<f64>("value".to_string()).unwrap());
            } else {
                let value = row_summary_cache.get::<f64>(key.to_string()).unwrap() + *rec.get::<f64>("value".to_string()).unwrap();
                row_summary_cache.set::<f64>(key.to_string(), value);
            }
        }
    }

    let summary_keys = row_summary_cache.get_values().keys().cloned().collect::<Vec<String>>();
    for key in summary_keys {
        let data = FormulaResultData {
            date: "".to_string(),
            key: key.clone(),
            value: Some(*row_summary_cache.get::<f64>(key.to_string()).unwrap() * 1.05),
        };

        formula_results.push("row".to_string(), data);
    }

    // ====================================
    // budget calculate
    let mut budget_summary_cache = AnyMap::new();

    for target_date in target_dates {
        let row_keys = record_cache.get_type_data_keys("account_unit".to_string(), target_date.to_string());
        for key in row_keys {
            let records = record_cache.get_mut::<Vec<AnyMap>>("account_unit".to_string(), target_date.to_string(), key.to_string()).unwrap();
            for rec in records {
                if !budget_summary_cache.has::<f64>(key.to_string()) {
                    budget_summary_cache.set::<f64>(key.to_string(), *rec.get::<f64>("forecast".to_string()).unwrap());
                } else {
                    let value = budget_summary_cache.get::<f64>(key.to_string()).unwrap() + *rec.get::<f64>("forecast".to_string()).unwrap();
                    budget_summary_cache.set::<f64>(key.to_string(), value);
                }
            }
        }
    }

    let summary_keys = budget_summary_cache.get_values().keys().cloned().collect::<Vec<String>>();
    for key in summary_keys {
        let data = FormulaResultData {
            date: "".to_string(),
            key: key.clone(),
            value: Some(*budget_summary_cache.get::<f64>(key.to_string()).unwrap() * 1.05),
        };

        formula_results.push("budget".to_string(), data);
    }
}

/// csv output.
///
/// # Arguments
///
/// * `path` - output file path
/// * `results` - output data
fn write_to_csv(path: String, results: Vec<FormulaResultData>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path(&path)?;
    writer.write_record(&[
        "date",
        "key",
        "value",
    ])?;

    for rec in results {
        writer.write_record(&[
            &rec.date.to_string(),
            &rec.key.to_string(),
            &rec.value.expect("null").to_string(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn main() {
    // ==============================================================
    // make cache
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut is_summary = false;
    if args.len() > 1 && args[1] == "summary" {
        is_summary = true;
        println!("yes summary");
    } else {
        println!("not summary");
    }

    let target_dates = [
        "2021-02-01",
        "2021-03-01",
        "2021-04-01",
        "2021-05-01",
        "2021-06-01",
        "2021-07-01",
        "2021-08-01",
        "2021-09-01",
        "2021-10-01",
        "2021-11-01",
        "2021-12-01",
        "2022-01-01",
    ];

    let mut record_cache = Records::new();
    record_cache.initialize();

    make_cache(target_dates, &mut record_cache);

    let end = start.elapsed();
    println!("make cache: {}.{:01}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    // ==============================================================
    // calculate
    let start = Instant::now();

    let mut formula_results = FormulaResults::new();

    if !is_summary {
        calculate(target_dates, &mut record_cache, &mut formula_results);
    } else {
        calculate_summary(target_dates, &mut record_cache, &mut formula_results);
    }

    let end = start.elapsed();
    println!("calculate: {}.{:01}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    // ==============================================================
    // csv output
    let start = Instant::now();

    const BASE_PATH: &str = "./output/";

    if let Err(e) = write_to_csv(BASE_PATH.to_string() + "rows.csv", formula_results.row) {
        eprintln!("{}", e)
    }
    if let Err(e) = write_to_csv(BASE_PATH.to_string() + "budgets.csv", formula_results.budget) {
        eprintln!("{}", e)
    }
    let end = start.elapsed();
    println!("csv output : {}.{:01}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);
}
