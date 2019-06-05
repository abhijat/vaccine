use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::component::Component;
use crate::default_value::DefaultValue;

pub trait ValueExt {
    fn get_string(&self, key: &str) -> String;
    fn default_value(&self) -> DefaultValue;
    fn get_datetime(&self) -> DefaultValue;
    fn child_defaults(&self) -> Value;
}

impl ValueExt for Value {
    fn get_string(&self, key: &str) -> String {
        self.get(key)
            .expect(&format!("missing key {}", key))
            .as_str()
            .expect(&format!("{} is not a string", key))
            .to_string()
    }

    fn default_value(&self) -> DefaultValue {
        match self.as_object()
            .unwrap()
            .get("default_value")
            .expect(&format!("could not extract 'default_value' from map")) {
            v if v.is_string() =>
                DefaultValue::String(v.as_str().unwrap().to_string()),
            v if v.is_number() =>
                DefaultValue::Number(v.as_i64().unwrap()),
            v if v.is_boolean() =>
                DefaultValue::Boolean(v.as_bool().unwrap()),
            v if v.is_object() =>
                DefaultValue::Mapping(v.child_defaults()),
            _ => panic!("invalid type of node"),
        }
    }

    fn get_datetime(&self) -> DefaultValue {
        let v = self.as_object().unwrap();
        DefaultValue::Datetime {
            format: self.get_string("format"),
            default: self.get_string("default_value"),
            timezone: self.get_string("timezone"),
        }
    }

    fn child_defaults(&self) -> Value {
        let m = self.as_object()
            .expect("node is not object!")
            .get("schema")
            .expect("could not GET schema from node")
            .as_array()
            .unwrap()
            .iter()
            .map(Component::new)
            .map(|component| (component.name, component.default_value.to_json()))
            .collect::<Map<String, Value>>();
        Value::from(m)
    }
}
