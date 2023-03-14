use crate::any_map::AnyMap;
use crate::formula_result::FormulaResults;

type CallbackRow = fn(rec: &mut AnyMap, formula_results: &mut FormulaResults);

#[derive(Debug)]
pub struct Summaries {
    summary_cache: AnyMap
}

impl Summaries {
    pub fn new() -> Self {
        Self {
            summary_cache: AnyMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.summary_cache.set(String::from("budget"), AnyMap::generate_cache());
        self.summary_cache.set(String::from("row"), AnyMap::generate_cache());
    }

    pub fn get_mut<T: 'static>(&mut self, has_type: String, date: String, key: String) -> Option<&mut T> {
        if !self.summary_cache.has::<AnyMap>(has_type.to_string()) {
            return None;
        }
        return self.summary_cache.get_mut::<AnyMap>(has_type.to_string());
    }
    
    pub fn set<T: 'static>(&mut self, has_type: String, date: String, key: String, value: T) {
        self.summary_cache.set::<T>(key.to_string(), value);
    }

    pub fn get_type_data_keys(&mut self, has_type: String, date: String) -> Vec<String> {
        return self.summary_cache.get_mut::<AnyMap>(has_type.to_string()).unwrap().get_mut::<AnyMap>(date.to_string()).unwrap().get_values().keys().cloned().collect::<Vec<String>>();
    }

    pub fn each_uncalculated_for_row(&mut self, has_type: String, date: String, keys: Vec<String>, c: CallbackRow, formula_results: &mut FormulaResults) {
        for key in keys {
            let rec = Self::get_mut::<AnyMap>(self, has_type.to_string(), date.to_string(), key.to_string()).unwrap();
            if !rec.get::<bool>("calculated".to_string()).is_none() && *rec.get::<bool>("calculated".to_string()).unwrap() {
                continue;
            }
            (c)(rec, formula_results);
        }
    }
}