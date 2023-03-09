use std::collections::HashMap;

struct Calculated {
    calculated_cache: HashMap<String, HashMap<String, HashMap<String, String>>>
}

impl Calculated {
    fn initialize(&mut self, generate_cache: HashMap<String, HashMap<String, String>>) {
        self.calculated_cache = HashMap::new();
        self.calculated_cache.insert(String::from("profit"), generate_cache.clone());
        self.calculated_cache.insert(String::from("category"), generate_cache.clone());
        self.calculated_cache.insert(String::from("group"), generate_cache.clone());
        self.calculated_cache.insert(String::from("account"), generate_cache.clone());
        self.calculated_cache.insert(String::from("budget"), generate_cache.clone());
        self.calculated_cache.insert(String::from("row"), generate_cache.clone());
    }

    fn has(&mut self, has_type: &String, date: &String, key: &String) -> bool {
        if self.calculated_cache.get(has_type) == None {
            return false;
        }
        if self.calculated_cache.get(has_type).unwrap().get(date) == None {
            return false;
        }
        if self.calculated_cache.get(has_type).unwrap().get(date).unwrap().get(key) == None {
            return false;
        }
        return true;
    }

    fn get(&mut self, has_type: &String, date: &String, key: &String) -> Option<&String> {
        if self.calculated_cache.get(has_type) == None {
            return None;
        }
        if self.calculated_cache.get(has_type).unwrap().get(date) == None {
            return None;
        }
        if self.calculated_cache.get(has_type).unwrap().get(date).unwrap().get(key) == None {
            return None;
        }
        return self.calculated_cache.get(has_type).unwrap().get(date).unwrap().get(key);
    }

    fn set(&mut self, has_type: &String, date: &String, key: &String, value: &String) {
        if self.calculated_cache.get(has_type) == None {
            self.calculated_cache.insert(has_type.to_string(), HashMap::new());
        }
        if self.calculated_cache.get(has_type).unwrap().get(date) == None {
            self.calculated_cache.get_mut(has_type).unwrap().insert(date.to_string(), HashMap::new());
        }
        self.calculated_cache.get_mut(has_type).unwrap().get_mut(date).unwrap().insert(key.to_string(), value.to_string());
    }
}
