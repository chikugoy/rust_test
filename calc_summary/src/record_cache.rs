use std::any::*;

use crate::any_map::AnyMap;
use crate::formula_result::FormulaResults;

type Callback = fn(depth: i32, rec: &mut AnyMap, formula_results: &mut FormulaResults) -> f64;

#[derive(Debug)]
pub struct Records {
    record_cache: AnyMap
}

impl Records {
    pub fn new() -> Self {
        Self {
            record_cache: AnyMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.record_cache.set(String::from("profit"), AnyMap::generate_cache());
        self.record_cache.set(String::from("category"), AnyMap::generate_cache());
        self.record_cache.set(String::from("group"), AnyMap::generate_cache());
        self.record_cache.set(String::from("account"), AnyMap::generate_cache());
        self.record_cache.set(String::from("account_unit"), AnyMap::generate_cache());
        self.record_cache.set(String::from("budget"), AnyMap::generate_cache());
        self.record_cache.set(String::from("row"), AnyMap::generate_cache());
    }

    pub fn has<T: 'static>(&mut self, has_type: String, date: String, key: String) -> bool {
        // FIXME: trait を使って record_cache と共通化できないか？
        if !self.record_cache.has::<AnyMap>(has_type.to_string()) {
            return false;
        }
        if !self.record_cache.get::<AnyMap>(has_type.to_string()).unwrap().has::<AnyMap>(date.to_string()) {
            return false;
        }
        if !self.record_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().has::<T>(key.to_string()) {
            return false;
        }
        return true;
    }

    // FIXME: 'static のおまじないがなぜ必要か？
    pub fn get_mut<T: 'static>(&mut self, has_type: String, date: String, key: String) -> Option<&mut T> {
        if !self.record_cache.has::<AnyMap>(has_type.to_string()) {
            return None;
        }
        if !self.record_cache.get::<AnyMap>(has_type.to_string()).unwrap().has::<AnyMap>(date.to_string()) {
            return None;
        }
        if !self.record_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().has::<T>(key.to_string()) {
            return None;
        }
        return self.record_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().get_mut::<T>(key.to_string());
    }
    
    pub fn set<T: 'static>(&mut self, has_type: String, date: String, key: String, value: T) {
        // let mut data_map = AnyMap::new();
        // data_map.set::<T>(key.to_string(), value);
        // let mut type_map = AnyMap::new();
        // type_map.set::<AnyMap>(date.to_string(), data_map);
        // self.record_cache.set::<AnyMap>(has_type.to_string(), type_map);
        // return;
        if !self.record_cache.has::<AnyMap>(has_type.to_string()) {
            let mut data_map = AnyMap::new();
            data_map.set::<T>(key.to_string(), value);
            let mut type_map = AnyMap::new();
            type_map.set::<AnyMap>(date.to_string(), data_map);
            self.record_cache.set::<AnyMap>(has_type.to_string(), type_map);
            println!("step1");
            return;
        }
        if !self.record_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().has::<AnyMap>(date.to_string()){
            let mut data_map = AnyMap::new();
            data_map.set::<T>(key.to_string(), value);
            self.record_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().set::<AnyMap>(date.to_string(), data_map);
            println!("step2");
            return;
        }
        self.record_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().set::<T>(key.to_string(), value);
    }

    pub fn get_type_data_keys(&mut self, has_type: String, date: String) -> Vec<String> {
        return self.record_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().get_values().keys().cloned().collect::<Vec<String>>();
    }

    pub fn each_uncalculated(&mut self, has_type: String, date: String, keys: Vec<String>, c: Callback, formula_results: &mut FormulaResults) {
        for key in keys {
            let rec = self.record_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().get_mut::<AnyMap>(key.to_string()).unwrap();
            if !rec.get::<bool>("calculated".to_string()).is_none() && *rec.get::<bool>("calculated".to_string()).unwrap() {
                continue;
            }
            (c)(i32::from(100), rec, formula_results);
        }
    }
}