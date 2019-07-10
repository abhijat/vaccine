use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde_json::Value;

use crate::rest_endpoint::Endpoint;
use rest_client::AuthType;

#[derive(Debug)]
pub struct Session {
    pub root_url: String,
    pub auth_type: Option<AuthType>,
    pub endpoints: HashMap<String, Endpoint>,
    pub created: HashMap<String, Value>,
}

impl Session {
    pub fn from_file(path: &str) -> Self {
        let mut file = File::open(path).expect(&format!("failed to open {}", path));

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).expect("failed to read file");

        let value = serde_json::from_str(buffer.as_str())
            .expect(&format!("failed to read json from {}", path));

        Self::new(&value)
    }

    pub fn new(v: &Value) -> Self {

        // Make sure the types match
        assert!(v.is_object());
        assert!(v["endpoints"].is_array());

        // Build up the endpoints
        let endpoints: HashMap<String, Endpoint> = v["endpoints"].as_array()
            .unwrap()
            .iter()
            .map(|v| Endpoint::new(v))
            .map(|e| (e.name.clone(), e))
            .collect();

        let auth_type: Option<AuthType> = if let Some(v) = v.get("auth_type") {
            serde_json::from_value(v.clone()).ok()
        } else {
            None
        };

        Session {
            endpoints,
            created: HashMap::new(),
            root_url: v["root_url"].as_str().expect("root_url missing or not string").to_string(),
            auth_type
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_json() {
        let data = r#"{
            "endpoints": [
                { "name": "a",
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
        ] },
        { "name": "b",
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
        ] }
            ]
        }"#;

        let v: Value = serde_json::from_str(data).unwrap();
        let config = Session::new(&v);
        assert_eq!(config.endpoints.len(), 2);
    }
}
