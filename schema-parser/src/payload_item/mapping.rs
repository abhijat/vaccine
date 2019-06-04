
use serde_json::{Map, Value};

use crate::payload_item::{ItemType, payload_item_from_json, PayloadItem};
use crate::value_extractors::ValueExt;

#[derive(Debug)]
pub struct MappingItem {
    name: String,
    children: Vec<Box<dyn PayloadItem>>,
}

impl MappingItem {
    pub fn new(v: &Value) -> Self {
        let name = v.get_string("name");
        let schema = v["schema"].as_array().expect("schema is not an array");
        let children: Vec<Box<dyn PayloadItem>> = schema.iter()
            .map(|v| payload_item_from_json(&v))
            .collect();
        MappingItem { name, children }
    }
}

impl PayloadItem for MappingItem {
    fn default_value(&self) -> (String, Value) {
        let default_values = self.children
            .iter()
            .map(|child| child.default_value())
            .collect::<Map<String, Value>>();

        (self.name.clone(), Value::from(default_values))
    }

    fn random_value(&self) -> (String, Value) {
        let random_values = self.children
            .iter()
            .map(|child| child.random_value())
            .collect::<Map<String, Value>>();

        (self.name.clone(), Value::from(random_values))
    }

    fn item_type(&self) -> ItemType {
        ItemType::Mapping
    }
}

#[cfg(test)]
mod mapping_item {
    use super::*;

    fn create_map_item() -> MappingItem {
        let v: Value = serde_json::from_str(r#"{
        "name": "sn",
        "kind": "mapping",
        "schema": [
              { "name": "ss", "kind": "string", "default_value": "ss" },
              { "name": "snnum", "kind": "number", "default_value": 11111 }

         ]}"#).unwrap();
        MappingItem::new(&v)
    }

    #[test]
    fn test_mapping_item_creation() {
        let m: MappingItem = create_map_item();
        assert_eq!(m.name, "sn");
    }

    #[test]
    fn test_mapping_item_default_values() {
        let (name, value): (String, Value) = create_map_item().default_value();
        assert_eq!(name, "sn");
        assert!(value.is_object());
        assert_eq!(value["snnum"].as_i64().unwrap(), 11111);
        assert_eq!(value["ss"].as_str().unwrap(), "ss");
    }

    #[test]
    fn test_mapping_item_random_values() {
        let (name, value): (String, Value) = create_map_item().random_value();
        assert_eq!(name, "sn");
        assert!(value.is_object());
        assert!(value["snnum"].is_number());
        assert!(value["ss"].is_string());
    }
}
