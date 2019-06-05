use std::collections::HashMap;

use serde_json::{Map, Value};

pub trait ValueExt {
    fn get_string(&self, key: &str) -> String;
}

impl ValueExt for Value {
    fn get_string(&self, key: &str) -> String {
        self.get(key)
            .expect(&format!("missing key {}", key))
            .as_str()
            .expect(&format!("{} is not a string", key))
            .to_string()
    }
}
