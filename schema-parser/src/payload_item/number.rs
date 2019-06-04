use serde_json::Value;
use crate::value_extractors::ValueExt;
use crate::random_values::generate_random_number;
use crate::payload_item::{PayloadItem, ItemType};

pub struct NumberItem {
    name: String,
    default_value: i64,
}

impl NumberItem {
    pub fn new(v: &Value) -> Self {
        NumberItem {
            name: v.get_string("name"),
            default_value: v["default_value"].as_i64().unwrap(),
        }
    }
}

impl PayloadItem for NumberItem {
    fn default_value(&self) -> (String, Value) {
        (self.name.clone(), json!(self.default_value))
    }

    fn random_value(&self) -> (String, Value) {
        (self.name.clone(), json!(generate_random_number()))
    }

    fn item_type(&self) -> ItemType {
        ItemType::Number
    }
}


#[cfg(test)]
mod number_item {
    use super::*;

    fn create_number_item() -> NumberItem {
        let v: Value = serde_json::from_str(r#"{
            "default_value": 1,
            "name": "myName"
        }"#).unwrap();
        NumberItem::new(&v)
    }

    #[test]
    fn number_item_creation() {
        let n = create_number_item();
        assert_eq!(n.name, "myName");
        assert_eq!(n.default_value, 1);
    }

    #[test]
    fn number_item_default_value() {
        let n = create_number_item();
        let (name, value): (String, Value) = n.default_value();
        assert_eq!(name, "myName");
        assert_eq!(value, json!(1));
    }

    #[test]
    fn number_item_random_value() {
        let (name, value): (String, Value) = create_number_item().random_value();
        assert_eq!(name, "myName");
        assert!(value.is_i64());
    }
}
