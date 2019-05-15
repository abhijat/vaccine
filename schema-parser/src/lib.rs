#[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate chrono_tz;

use serde_json::{Map, Value};

mod component;
mod default_value;
mod rest_endpoint;
mod random_values;
mod value_extractors;
mod datetime;

pub fn extract_string_from_value(v: &Map<String, Value>, key: &str) -> String {
    v.get(key)
        .expect("missing key name")
        .as_str()
        .expect("name is not a string")
        .to_string()
}
