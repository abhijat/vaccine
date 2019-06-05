use serde_json::Value;

use crate::rest_endpoint::Endpoint;

#[derive(Debug)]
pub struct Configuration {
    endpoints: Vec<Endpoint>,
}

impl Configuration {
    pub fn new(v: &Value) -> Self {

        // Make sure the types match
        assert!(v.is_object());
        assert!(v["endpoints"].is_array());

        // Build up the endpoints
        let endpoints: Vec<Endpoint> = v["endpoints"].as_array()
            .unwrap()
            .iter()
            .map(|v| Endpoint::new(v))
            .collect();
        Configuration { endpoints }
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
        let config = Configuration::new(&v);
        assert_eq!(config.endpoints.len(), 2);
    }
}
