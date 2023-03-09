extern crate diesel;

use diesel::prelude::*;
use thousands::Separable;

use self::models::*;
use self::any_map::*;
use self::calculated_cache::*;
use self::record_cache::*;
use calc_summary::*;

fn make_cache(target_dates: [&str;12], calculated_cache: &mut Calculated, record_cache: &mut Records) {
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

    for rec in results {
        let row_date: &String = &rec.date.format("%Y-%m-%d").to_string();
        let target_budget_id = if !rec.budget_id.is_none() { rec.budget_id.expect("").to_string() } else { "".to_string() };

        if !budget_row.get::<AnyMap>(row_date.to_string()).unwrap().has::<AnyMap>(target_budget_id.clone()) {
            // FIXME: 冗長なので改善したい
            let mut budget_row_data = AnyMap::new();
            budget_row_data.set("row_id".to_string(), i64::from(-1));
            budget_row_data.set("date".to_string(), rec.date);
            budget_row_data.set("value".to_string(), f64::from(0));
            budget_row_data.set("calculated".to_string(), true);
            budget_row.get_mut::<AnyMap>(row_date.to_string()).unwrap().set::<Vec<AnyMap>>(target_budget_id.clone(), vec![budget_row_data]);

            if rec.calculated.unwrap() {
                calculated_cache.set::<f64>("row".to_string(), row_date.to_string(), rec.row_id.to_string(), rec.value.unwrap());
                if !rec.is_not_aggregate.unwrap() {
                    let target_row = budget_row.get_mut::<AnyMap>(row_date.to_string()).unwrap().get_mut::<Vec<AnyMap>>(target_budget_id.clone()).unwrap();
                    let calced_add = calc::add(&target_row[0].get::<f64>("value".to_string()).unwrap(), &rec.value.unwrap());
                    target_row[0].set::<f64>("value".to_string(), calced_add)
                }
            } else {
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
                row_data.set("value".to_string(), rec.value.unwrap());
                row_data.set("calculated".to_string(), rec.calculated.unwrap());

                record_cache.set::<AnyMap>("row".to_string(), row_date.to_string(), rec.row_id.to_string(), row_data);
            }
        }
    }

    println!("FINISH cache row");

    // println!("{}", row_date.to_string());
    // println!("{}", target_budget_id.clone());

    println!("START cache budget");

    let results = tmp_budget_pavs
        .load::<TmpBudgetPav>(connection)
        .expect("Error loading tmp row budgets");
    println!("tmp row budgets count {}", results.len().separate_with_commas());

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

        let target_value = if rec.budget.is_none() { rec.forecast.unwrap() } else { rec.budget.unwrap() };

        if rec.calculated.unwrap() {
            calculated_cache.set::<f64>("budget".to_string(), row_date.to_string(), rec.budget_id.to_string(), target_value.clone());

            let target_row = record_cache.get_mut::<Vec<AnyMap>>("account_unit".to_string(), row_date.to_string(), aukey.to_string()).unwrap();
            let calced_add = calc::add(target_row[0].get::<f64>("value".to_string()).unwrap(), &target_value.clone());
            target_row[0].set::<f64>("value".to_string(), calced_add);
        } else {
            let mut account_unit_data = AnyMap::new();
            account_unit_data.set("budget_id".to_string(), rec.budget_id);
            account_unit_data.set("date".to_string(), rec.date);
            account_unit_data.set("budget".to_string(), rec.budget);
            account_unit_data.set("forecast".to_string(), rec.forecast);
            account_unit_data.set("achievement".to_string(), rec.achievement);
            account_unit_data.set("calculated".to_string(), rec.calculated);
            let target_row = record_cache.get_mut::<Vec<AnyMap>>("account_unit".to_string(), row_date.to_string(), aukey.to_string()).unwrap();
            target_row.push(account_unit_data);

            // let rows = budget_row.get::<AnyMap>(row_date.to_string()).unwrap().get::<Vec<AnyMap>>(rec.budget_id.to_string()).unwrap();
            let mut budget_data = AnyMap::new();
            budget_data.set("budget_id".to_string(), rec.budget_id);
            budget_data.set("date".to_string(), rec.date);
            budget_data.set("budget".to_string(), rec.budget);
            budget_data.set("forecast".to_string(), rec.forecast);
            budget_data.set("achievement".to_string(), rec.achievement);
            budget_data.set("calculated".to_string(), rec.calculated);
            // budget_data.set("rows".to_string(), rows.clone());
            record_cache.set::<AnyMap>("budget".to_string(), row_date.to_string(), rec.budget_id.to_string(), budget_data);
        }
        break;
    }

    println!("FINISH cache budget");

    println!("START cache account");

    let results = tmp_accounts
        .load::<TmpAccount>(connection)
        .expect("Error loading tmp row accounts");
    println!("tmp row accounts count {}", results.len().separate_with_commas());

    for rec in results {
        for profit_id in rec.profit_id_array.clone().unwrap().as_array().unwrap() {
            for target_date in target_dates {
                if !record_cache.has::<Vec<AnyMap>>("profit".to_string(), target_date.to_string(), profit_id.to_string()) {
                    record_cache.set::<Vec<AnyMap>>("profit".to_string(), target_date.to_string(), profit_id.to_string(), vec![]);
                }
                let target_row = record_cache.get_mut::<Vec<AnyMap>>("profit".to_string(), target_date.to_string(), profit_id.to_string()).unwrap();
                let mut account_data = AnyMap::new();
                account_data.set("account_id".to_string(), rec.account_id);
                account_data.set("category_id".to_string(), rec.category_id);
                account_data.set("coefficient".to_string(), rec.coefficient);
                account_data.set("group_id_array".to_string(), rec.group_id_array.clone());
                account_data.set("profit_id_array".to_string(), rec.profit_id_array.clone());
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
                account_data.set("account_id".to_string(), rec.account_id);
                account_data.set("category_id".to_string(), rec.category_id);
                account_data.set("coefficient".to_string(), rec.coefficient);
                account_data.set("group_id_array".to_string(), rec.group_id_array.clone());
                account_data.set("profit_id_array".to_string(), rec.profit_id_array.clone());
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
            account_data.set("account_id".to_string(), rec.account_id);
            account_data.set("category_id".to_string(), rec.category_id);
            account_data.set("coefficient".to_string(), rec.coefficient);
            account_data.set("group_id_array".to_string(), rec.group_id_array.clone());
            account_data.set("profit_id_array".to_string(), rec.profit_id_array.clone());
            target_row.push(account_data);

            let mut account_data = AnyMap::new();
            account_data.set("account_id".to_string(), rec.account_id);
            account_data.set("category_id".to_string(), rec.category_id);
            account_data.set("coefficient".to_string(), rec.coefficient);
            account_data.set("group_id_array".to_string(), rec.group_id_array.clone());
            account_data.set("profit_id_array".to_string(), rec.profit_id_array.clone());
            record_cache.set::<Vec<AnyMap>>("account".to_string(), target_date.to_string(), target_account_id.to_string(), vec![account_data]);
        }
    }

    println!("FINISH cache account");
}

fn main() {
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

    let mut calculated_cache = Calculated::new();
    calculated_cache.initialize();

    let mut record_cache = Records::new();
    record_cache.initialize();

    make_cache(target_dates, &mut calculated_cache, &mut record_cache);

    // record_cache.each_uncalculated()
}