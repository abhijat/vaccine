use serde_json::Value;

use crate::payload_item::ItemType;
use crate::payload_item::PayloadItem;
use crate::random_values::generate_random_string;
use crate::value_extractors::ValueExt;

pub struct StringItem {
    name: String,
    default_value: String,
}

impl StringItem {
    pub fn new(v: &Value) -> Self {
        StringItem {
            name: v.get_string("name"),
            default_value: v.get_string("default_value"),
        }
    }
}

impl PayloadItem for StringItem {
    fn default_value(&self) -> (String, Value) {
        (self.name.clone(), json!(self.default_value))
    }

    fn random_value(&self) -> (String, Value) {
        (self.name.clone(), json!(generate_random_string()))
    }

    fn item_type(&self) -> ItemType {
        ItemType::String
    }
}


#[cfg(test)]
mod string_item {
    use super::*;

    fn create_string_item() -> StringItem {
        let v: Value = serde_json::from_str(r#"{
            "default_value": "foo",
            "name": "myName"
        }"#).unwrap();
        StringItem::new(&v)
    }

    #[test]
    fn string_item_creation() {
        let s = create_string_item();
        assert_eq!(s.name, "myName");
        assert_eq!(s.default_value, "foo");
    }

    #[test]
    fn string_item_default_value() {
        let s: StringItem = create_string_item();
        let (name, value) = s.default_value();
        assert_eq!(name, "myName");
        assert_eq!(value, "foo");
    }

    #[test]
    fn string_item_random_value() {
        let s: StringItem = create_string_item();
        let (name, value) = s.random_value();
        assert_eq!(name, "myName");
        assert!(value.is_string());
        assert!(!value.as_str().unwrap().is_empty());
    }
}
