use std::any::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AnyMap {
    values: HashMap<String, Box<dyn Any>>,
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

    pub fn get<T>(&self, key: String) -> Option<&T>
        where
            T: Any,
    {
        self.values.get(&key).and_then(|v| v.downcast_ref::<T>())
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
}

// FIXME: 共通化したい
pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
