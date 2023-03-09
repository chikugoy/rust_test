use std::any::*;

use crate::any_map::AnyMap;

#[derive(Debug)]
pub struct Calculated {
    calculated_cache: AnyMap
}

impl Calculated {
    pub fn new() -> Self {
        Self {
            calculated_cache: AnyMap::new(),
        }
    }
    pub fn initialize(&mut self) {
        self.calculated_cache.set(String::from("profit"), AnyMap::generate_cache());
        self.calculated_cache.set(String::from("category"), AnyMap::generate_cache());
        self.calculated_cache.set(String::from("group"), AnyMap::generate_cache());
        self.calculated_cache.set(String::from("account"), AnyMap::generate_cache());
        self.calculated_cache.set(String::from("budget"), AnyMap::generate_cache());
        self.calculated_cache.set(String::from("row"), AnyMap::generate_cache());
    }

    pub fn has<T: 'static>(&mut self, has_type: String, date: String, key: String) -> bool {
        // FIXME: trait を使って record_cache と共通化できないか？
        if !self.calculated_cache.has::<AnyMap>(has_type.to_string()) {
            return false;
        }
        if !self.calculated_cache.get::<AnyMap>(has_type.to_string()).unwrap().has::<AnyMap>(date.to_string()) {
            return false;
        }
        if !self.calculated_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().has::<T>(key.to_string()) {
            return false;
        }
        return true;
    }

    // FIXME: 'static のおまじないがなぜ必要か？
    pub fn get_mut<T: 'static>(&mut self, has_type: String, date: String, key: String) -> Option<&mut T> {
        if !self.calculated_cache.has::<AnyMap>(has_type.to_string()) {
            return None;
        }
        if !self.calculated_cache.get::<AnyMap>(has_type.to_string()).unwrap().has::<AnyMap>(date.to_string()) {
            return None;
        }
        if !self.calculated_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().has::<T>(key.to_string()) {
            return None;
        }
        return self.calculated_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().get_mut::<T>(key.to_string());
    }

    pub fn set<T: 'static>(&mut self, has_type: String, date: String, key: String, value: T) {
        if !self.calculated_cache.has::<AnyMap>(has_type.to_string()) {
            self.calculated_cache.set::<AnyMap>(has_type.to_string(), AnyMap::new());
            return;
        }
        if !self.calculated_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().has::<AnyMap>(date.to_string()){
            self.calculated_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().set::<AnyMap>(date.to_string(), AnyMap::new());
        }
        self.calculated_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().set::<T>(key.to_string(), value);
    }

}
