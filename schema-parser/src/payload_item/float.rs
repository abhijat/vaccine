use serde_json::Value;

use crate::payload_item::{ItemType, PayloadItem};
use crate::random_values::generate_random_float;
use crate::value_extractors::ValueExt;

pub struct FloatItem {
    name: String,
    default_value: f64,
}

impl FloatItem {
    pub fn new(v: &Value) -> Self {
        FloatItem {
            name: v.get_string("name"),
            default_value: v["default_value"].as_f64().expect("default_value is not float!"),
        }
    }
}

impl PayloadItem for FloatItem {
    fn default_value(&self) -> (String, Value) {
        (self.name.clone(), json!(self.default_value))
    }

    fn random_value(&self) -> (String, Value) {
        (self.name.clone(), json!(generate_random_float()))
    }

    fn item_type(&self) -> ItemType {
        ItemType::Float
    }
}


#[cfg(test)]
mod float_item {
    use super::*;

    fn create_float_item() -> FloatItem {
        let v: Value = serde_json::from_str(r#"{
            "default_value": 1.234,
            "name": "myName"
        }"#).unwrap();
        FloatItem::new(&v)
    }

    #[test]
    fn float_item_creation() {
        let f = create_float_item();
        assert_eq!(f.name, "myName");
        assert_eq!(f.default_value, 1.234);
    }

    #[test]
    fn float_item_default_value() {
        let f = create_float_item();
        let (name, value) = f.default_value();
        assert_eq!(name, "myName");
        assert_eq!(value, 1.234);
    }

    #[test]
    fn float_item_random_value() {
        let (name, value): (String, Value) = create_float_item().random_value();
        assert_eq!(name, "myName");
        assert!(value.is_f64());
    }
}
