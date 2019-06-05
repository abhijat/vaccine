use serde_json::Value;

use crate::payload_item::{ItemType, PayloadItem};
use crate::random_values::generate_random_boolean;
use crate::value_extractors::ValueExt;

pub struct BooleanItem {
    name: String,
    default_value: bool,
}

impl BooleanItem {
    pub fn new(v: &Value) -> Self {
        BooleanItem {
            name: v.get_string("name"),
            default_value: v["default_value"].as_bool().expect("default_value is not boolean"),
        }
    }
}

impl PayloadItem for BooleanItem {
    fn default_value(&self) -> (String, Value) {
        (self.name.clone(), Value::from(self.default_value))
    }

    fn random_value(&self) -> (String, Value) {
        (self.name.clone(), Value::from(generate_random_boolean()))
    }

    fn item_type(&self) -> ItemType {
        ItemType::Boolean
    }
}

#[cfg(test)]
mod boolean_item {
    use super::*;

    fn create_boolean_item() -> BooleanItem {
        let v = serde_json::from_str(r#"{
            "name": "foobar_flag",
            "default_value": false
        }"#).unwrap();
        BooleanItem::new(&v)
    }

    #[test]
    fn test_boolean_item_creation() {
        let b = create_boolean_item();
        assert_eq!(b.name, "foobar_flag");
        assert_eq!(b.default_value, false);
    }

    #[test]
    fn test_boolean_item_default_value() {
        let b = create_boolean_item();
        let (name, default) = b.default_value();
        assert_eq!(name, "foobar_flag");
        assert_eq!(default, Value::from(false));
    }

    #[test]
    fn test_boolean_item_random_value() {
        let (name, value) = create_boolean_item().random_value();
        assert_eq!(name, "foobar_flag");
        assert!(value.is_boolean());
    }
}
