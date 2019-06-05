use serde_json::{Map, Value};

use crate::component::Component;
use crate::extract_string_from_value;
use crate::payload_item::{payload_item_from_json, PayloadItem};
use crate::random_values::random_elements;

#[derive(Debug)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
    pub requires: Vec<String>,
    pub components: Vec<Box<dyn PayloadItem>>,
}

impl Endpoint {
    pub fn new(v: &Value) -> Self {
        let v = v.as_object().expect("payload is not object");
        let name = extract_string_from_value(v, "name");
        let url = extract_string_from_value(v, "url");
        let requires: Vec<String> = v.get("requires").expect("missing `requires`")
            .as_array().expect("`requires` is not an array")
            .iter()
            .map(|v| v.as_str().expect("non string in requires field").to_string())
            .collect();

        let components = v.get("schema")
            .expect("missing `schema`")
            .as_array()
            .expect("`schema` is not an array")
            .iter()
            .map(payload_item_from_json)
            .collect::<Vec<Box<dyn PayloadItem>>>();

        Endpoint { name, url, requires, components }
    }

    pub fn default_payload(&self) -> Value {
        let m = self.components.iter()
            .map(|c| c.default_value())
            .collect::<Map<String, Value>>();
        Value::from(m)
    }

    pub fn randomized_payload(&self) -> Value {
        let m = random_elements(&self.components)
            .map(|c| c.random_value())
            .collect::<Map<String, Value>>();
        Value::from(m)
    }
}

#[cfg(test)]
mod public_api {
    use serde_json::Value;

    use crate::default_value::DefaultValue;

    use super::*;

    fn create_endpoint() -> Endpoint {
        let payload = r#"{ "name": "a",
          "url": "http://localhost:8000/api/v2/house",
          "requires": [],
          "schema": [
            {
              "name": "houseType",
              "kind": "string",
              "default_value": "castle"
            },
            {
              "name": "sizeInSquareFeet",
              "kind": "number",
              "default_value": 11001100
            },
            {
              "name": "isSurroundedByAMoat",
              "kind": "boolean",
              "default_value": true
            },
            {
              "name": "startDate",
              "kind": "datetime",
              "timezone": "Asia/Kolkata",
              "format": "%Y-%m-%d %H:%M:%S",
              "default_value": "2001-01-01 11:22:33"
            },
            {
              "name": "endDate",
              "kind": "datetime",
              "timezone": "Asia/Kolkata",
              "format": "%Y-%m-%d %H:%M:%S",
              "default_value": "now + 500minutes"
            },
            {
              "name": "constructionMaterial",
              "kind": "mapping",
              "schema": [
                 { "name": "wallMaterial", "kind": "string", "default_value": "plasterOfParis" },
                 { "name": "tonnage", "kind": "number", "default_value": 100 },
                 { "name": "flammable", "kind": "boolean", "default_value": false }
               ]
            }
        ] }"#;

        let v: Value = serde_json::from_str(payload).expect("failed to parse payload");
        Endpoint::new(&v)
    }

    #[test]
    fn simple_rest_endpoint_from_json_payload() {
        let e = create_endpoint();
        assert_eq!(e.name, "a");
        assert_eq!(e.url, "http://localhost:8000/api/v2/house");
        assert_eq!(e.requires.len(), 0);
        assert_eq!(e.components.len(), 6);
    }

    #[test]
    fn assert_endpoint_components() {
        let e = create_endpoint().components;
        let (name, value) = e[0].default_value();
        assert_eq!(name.as_str(), "houseType");
        assert_eq!(value, Value::from("castle"));

        let (name, value) = e[2].default_value();
        assert_eq!(name.as_str(), "isSurroundedByAMoat");
        assert_eq!(value, Value::from(true));

        let (name, value) = e[3].default_value();
        assert_eq!(name, "startDate");
        assert_eq!(value, Value::from("2001-01-01 11:22:33"));

        let (name, value) = e[4].default_value();
        assert_eq!(name, "endDate");

        let (name, value) = e[5].default_value();
        assert_eq!(name.as_str(), "constructionMaterial");
        assert_eq!(value, json!({
            "wallMaterial": "plasterOfParis",
            "tonnage": 100,
            "flammable": false,
        }));
    }

    #[test]
    fn default_payload_generation() {
        let e = create_endpoint();
        let p = e.default_payload();
        assert_eq!(p["constructionMaterial"]["wallMaterial"], "plasterOfParis");
        assert_eq!(p["constructionMaterial"]["flammable"], false);
        assert_eq!(p["constructionMaterial"]["tonnage"], 100);
        assert_eq!(p["sizeInSquareFeet"], 11001100);
        assert_eq!(p["isSurroundedByAMoat"], true);
        assert_eq!(p["houseType"], "castle");
    }

    #[test]
    fn randomized_payload_generation() {
        let e = create_endpoint();
        let p = e.randomized_payload();

        assert!(p.is_object());
        let p = p.as_object().unwrap();

        // None of these assertions are guaranteed to run!
        p.get("sizeInSquareFeet").map(|v| assert!(v.is_number()));
        p.get("isSurroundedByAMoat").map(|v| assert!(v.is_boolean()));
        p.get("houseType").map(|v| assert!(v.is_string()));

        p.get("constructionMaterial").map(|material| {
            assert!(material.is_object());
            let material = material.as_object().unwrap();
            material.get("wallMaterial").map(|v| v.is_string());
            material.get("flammable").map(|v| v.is_boolean());
            material.get("tonnage").map(|v| v.is_number());
        });
    }
}
