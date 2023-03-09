use std::any::*;

use crate::any_map::AnyMap;

type Callback = fn(map: &AnyMap);

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

    pub fn has(&mut self, has_type: String, date: String, key: String) -> bool {
        // FIXME: trait を使って record_cache と共通化できないか？
        if !self.record_cache.has::<AnyMap>(has_type.to_string()) {
            return false;
        }
        if !self.record_cache.has::<AnyMap>(format!("{}{}{}", has_type.to_string(), "-", date.to_string())) {
            return false;
        }

        if !self.record_cache.has::<AnyMap>(format!("{}{}{}{}{}", has_type.to_string(), "-", date.to_string(), "-", key.to_string())) {
            return false;
        }
        return true;
    }

    pub fn get<T: 'static>(&mut self, has_type: String, date: String, key: String) -> Option<&T> {
        if !self.record_cache.has::<AnyMap>(has_type.to_string()) {
            return None;
        }
        if !self.record_cache.has::<AnyMap>(format!("{}{}{}", has_type.to_string(), "-", date.to_string())) {
            return None;
        }
        if !self.record_cache.has::<T>(format!("{}{}{}{}{}", has_type.to_string(), "-", date.to_string(), "-", key.to_string())) {
            return None;
        }
        return self.record_cache.get::<T>(format!("{}{}{}{}{}", has_type.to_string(), "-", date.to_string(), "-", key.to_string()));
    }

    pub fn get_mut<T: 'static>(&mut self, has_type: String, date: String, key: String) -> Option<&mut T> {
        if !self.record_cache.has::<AnyMap>(has_type.to_string()) {
            return None;
        }
        if !self.record_cache.has::<AnyMap>(format!("{}{}{}", has_type.to_string(), "-", date.to_string())) {
            return None;
        }
        if !self.record_cache.has::<T>(format!("{}{}{}{}{}", has_type.to_string(), "-", date.to_string(), "-", key.to_string())) {
            return None;
        }
        return self.record_cache.get_mut::<T>(format!("{}{}{}{}{}", has_type.to_string(), "-", date.to_string(), "-", key.to_string()));
    }

    pub fn set<T: 'static>(&mut self, has_type: String, date: String, key: String, value: T) {
        if !self.record_cache.has::<AnyMap>(has_type.to_string()) {
            self.record_cache.set::<AnyMap>(has_type.to_string(), AnyMap::new());
        }
        if !self.record_cache.has::<AnyMap>(format!("{}{}{}", has_type.to_string(), "-", date.to_string())) {
            self.record_cache.set::<AnyMap>(format!("{}{}{}", has_type.to_string(), "-", date.to_string()), AnyMap::new());
        }
        self.record_cache.set::<T>(format!("{}{}{}{}{}", has_type.to_string(), "-", date.to_string(), "-", key.to_string()), value);
    }

    pub fn each_uncalculated(&mut self, has_type: String, date: String, c: Callback) {
        let records = &self.record_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().values;
        for (key, value) in &*records {
            let data = &value.downcast_ref::<AnyMap>().unwrap();
            println!("{} / {:?}", key, data);
            // if *data.get_mut::<bool>("calculated".to_string()).unwrap() {
            //     (c)(data)
            // }
        }
    }
}
