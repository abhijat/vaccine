use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use humantime::parse_duration;
use serde_json::{Map, Value};

use crate::datetime::{datetime_from_now, is_now};
use crate::default_value::DefaultValue;
use crate::extract_string_from_value;
use crate::random_values::*;
use crate::value_extractors::ValueExt;

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub kind: String,
    pub default_value: DefaultValue,
    pub children: Option<Vec<Component>>,
}

impl Component {
    pub fn new(v: &Value) -> Self {
        let name = v.get_string("name");
        let kind = v.get_string("kind");

        let default_value = if kind == "datetime" {
            v.get_datetime()
        } else {
            v.default_value()
        };

        let children = if kind == "mapping" {
            Some(Self::populate_children(v.as_object().unwrap()))
        } else {
            None
        };

        Component { name, kind, default_value, children }
    }

    pub fn default_payload(&self) -> (String, Value) {
        (self.name.clone(), self.default_value.to_json())
    }

    pub fn randomized_payload(&self) -> (String, Value) {
        (self.name.clone(), self.random_value())
    }

    fn random_value(&self) -> Value {
        match self.kind.as_str() {
            "string" => json!(generate_random_string()),
            "number" => json!(generate_random_number()),
            "boolean" => json!(generate_random_boolean()),
            "mapping" => self.extract_random_values_from_child_nodes(),
            "datetime" => self.default_value.random_datetime(),
            _ => panic!(format!("unknown kind {}", self.kind))
        }
    }

    pub fn populate_children(v: &Map<String, Value>) -> Vec<Component> {
        v["default_value"]["schema"]
            .as_array()
            .expect("default_value -> schema is not an array")
            .iter()
            .map(Self::new)
            .collect()
    }

    fn extract_random_values_from_child_nodes(&self) -> Value {
        self.children.as_ref()
            .map(|c| random_elements(c))
            .map(|c| c.map(|component| (component.name.clone(), component.random_value()))
                .collect::<Map<String, Value>>())
            .map(|m| Value::from(m))
            .expect("no children to generate random values from!")
    }
}

#[cfg(test)]
mod public_api {
    use crate::component::*;
    use crate::default_value::DefaultValue;

    #[test]
    fn simple_component_creation() {
        let payload = r#"{ "name": "service-name", "kind": "string", "default_value": "a-simple-service" }"#;

        let v: serde_json::Value = serde_json::from_str(payload).unwrap();
        let component = Component::new(&v);

        assert_eq!(&component.name, "service-name");
        assert_eq!(&component.kind, "string");

        match component.default_value {
            DefaultValue::String(s) => assert_eq!(s, "a-simple-service"),
            _ => panic!("unexpected value!"),
        }
    }
}

#[cfg(test)]
mod default_payload {
    use super::*;

    #[test]
    fn generate_simple_payload() {
        let payload = r#"{ "name": "service-name", "kind": "string", "default_value": "a-simple-service" }"#;
        let component = Component::new(&serde_json::from_str(payload).unwrap());
        let (key, value) = component.default_payload();

        assert_eq!(&key, "service-name");
        assert_eq!(value, json!("a-simple-service"));
    }

    #[test]
    fn generate_complex_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn", "kind": "mapping",
          "default_value": {"schema": [
              { "name": "sn", "kind": "boolean", "default_value": false },
              { "name": "ss", "kind": "string", "default_value": "ss" },
              { "name": "snnum", "kind": "number", "default_value": 11111 }
            ] } }"#).unwrap();

        let component = Component::new(&v);
        let (key, value) = component.default_payload();
        assert_eq!(&key, "sn");
        assert_eq!(value, json!({"sn": false, "ss": "ss", "snnum": 11111}));
    }
}

#[cfg(test)]
mod randomized_payload {
    use super::*;

    #[test]
    fn generate_randomized_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn", "kind": "mapping",
          "default_value": {"schema": [
              { "name": "sn", "kind": "boolean", "default_value": false },
              { "name": "ss", "kind": "string", "default_value": "ss" },
              { "name": "snnum", "kind": "number", "default_value": 11111 }
            ] } }"#).unwrap();

        let component = Component::new(&v);
        let (key, value) = component.randomized_payload();
        assert_eq!(&key, "sn");

        assert!(value.is_object());

        let value = value.as_object().unwrap();

        value.get("sn").map(|v| assert!(v.is_boolean()));
        value.get("ss").map(|v| assert!(v.is_string()));
        value.get("snnum").map(|v| assert!(v.is_number()));
    }
}

#[cfg(test)]
mod value_extraction {
    use serde_json::Value;

    use super::*;
    use super::DefaultValue::*;

    #[test]
    fn extract_string_from_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn", "kind": "string", "default_value": "ss"}"#)
            .unwrap();
        match v.default_value() {
            String(s) => assert_eq!(&s, "ss"),
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn extract_number_from_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn", "kind": "number", "default_value": 10000}"#)
            .unwrap();
        match v.default_value() {
            Number(n) => assert_eq!(n, 10000),
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn extract_boolean_from_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn", "kind": "boolean", "default_value": false}"#)
            .unwrap();
        match v.default_value() {
            Boolean(b) => assert!(!b),
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn extract_object_from_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn", "kind": "mapping",
          "default_value": {"schema": [
              { "name": "sn", "kind": "boolean", "default_value": false },
              { "name": "ss", "kind": "string", "default_value": "ss" },
              { "name": "snnum", "kind": "number", "default_value": 11111 }
            ] } }"#).unwrap();

        let component = Component::new(&v);
        assert_eq!(&component.kind, "mapping");

        match component.default_value {
            Mapping(m) => assert_eq!(m, json!({"sn": false, "ss": "ss", "snnum": 11111})),
            _ => panic!("unexpected variant"),
        }
    }
}

#[cfg(test)]
mod datetime_field {
    use chrono::{Datelike, NaiveDate, NaiveDateTime};
    use chrono_tz::Asia;

    use super::*;

    #[test]
    fn datetime_with_fixed_value() {
        let v = serde_json::from_str(r#"{
            "name": "start_date",
            "kind": "datetime",
            "default_value": "2019-01-13",
            "timezone": "Asia/Kolkata",
            "format": "%Y-%m-%d"
        }"#).unwrap();

        let c = Component::new(&v);
        let (key, value) = c.default_payload();

        assert_eq!(key, "start_date");
        assert_eq!(value, json!("2019-01-13"));
    }

    #[test]
    fn datetime_based_on_now() {
        let v = serde_json::from_str(r#"{
            "name": "start_date",
            "kind": "datetime",
            "default_value": "now + 100years",
            "timezone": "Asia/Kolkata",
            "format": "%Y-%m-%d"
        }"#).unwrap();

        let c = Component::new(&v);
        let (key, value) = c.default_payload();

        let value = value.as_str().unwrap();
        assert_eq!(key, "start_date");

        let now = Utc::now().with_timezone(&Asia::Kolkata);
        let parsed = NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap();
        assert_eq!(now.year() + 100, parsed.year());
    }
}
