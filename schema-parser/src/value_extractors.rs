use serde_json::{Map, Value};

use crate::extract_string_from_value;
use crate::default_value::DefaultValue;

pub fn extract_datetime_from_node(node: &Map<String, Value>) -> DefaultValue {
    let timezone = extract_string_from_value(node, "timezone");
    let format = extract_string_from_value(node, "format");
    let default = extract_string_from_value(node, "default_value");

    DefaultValue::Datetime { format, timezone, default }
}
