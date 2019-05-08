#[macro_use]
extern crate serde_json;

use serde_json::{Map, Value};

mod component;
mod rest_endpoint;
mod random_values;

pub fn extract_string_from_value(v: &Map<String, Value>, key: &str) -> String {
    v.get(key)
        .expect("missing key name")
        .as_str()
        .expect("name is not a string")
        .to_string()
}
