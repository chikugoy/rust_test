use std::any::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AnyMap {
    pub values: HashMap<String, Box<dyn Any>>,
}

impl AnyMap {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set<T>(&mut self, k: String, v: T)
        where
            T: Any,
    {
        self.values.insert(k, Box::new(v));
    }

    pub fn get<T>(&mut self, key: String) -> Option<&T>
        where
            T: Any,
    {
        self.values.get(&key).and_then(|v| v.downcast_ref::<T>())
    }

    pub fn get_mut<T>(&mut self, key: String) -> Option<&mut T>
        where
            T: Any,
    {
        self.values.get_mut(&key).and_then(|v| v.downcast_mut::<T>())
    }

    pub fn get_values(&mut self) -> &HashMap<String, Box<dyn Any>>
    {
        return &self.values;
    }

    pub fn has<T>(&self, key: String) -> bool
        where
            T: Any,
    {
        let val = self.values.get(&key).and_then(|v| v.downcast_ref::<T>());
        if val.is_none() {
            return false;
        }
        return true;
    }

    // FIXME: AnyMapにこの関数もたせるのは違うかも
    pub fn generate_cache() -> AnyMap {
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
            generate_cache.set::<AnyMap>(v.to_string().clone(), AnyMap::new());
        }

        return generate_cache;
    }
}
