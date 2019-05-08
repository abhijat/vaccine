use serde_json::{Map, Value};

#[derive(Debug)]
pub enum DefaultValue {
    String(String),
    Number(i64),
    Boolean(bool),
    Mapping(Value),
}

impl DefaultValue {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            DefaultValue::String(s) => json!(s),
            DefaultValue::Number(i) => json!(i),
            DefaultValue::Boolean(b) => json!(b),
            DefaultValue::Mapping(m) => m.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub kind: String,
    pub default_value: DefaultValue,
}

impl Component {
    pub fn new(v: &Value) -> Self {
        let v = v.as_object().expect("input is not object!");
        let name = Self::extract_string(v, "name");
        let kind = Self::extract_string(v, "kind");
        let default_value = Self::extract_default_value(v, "default_value")
            .expect("failed to extract default value");
        Component { name, kind, default_value }
    }

    pub fn payload(&self) -> (String, Value) {
        (self.name.clone(), self.default_value.to_json())
    }

    fn extract_string(v: &Map<String, Value>, key: &str) -> String {
        v.get(key).expect("missing key name")
            .as_str().expect("name is not a string")
            .to_string()
    }

    pub fn extract_default_value(v: &Map<String, serde_json::Value>, key: &str)
                                 -> Option<DefaultValue> {
        let v = v.get(key).expect(&format!("could not extract {} from map", key));

        match v {
            v if v.is_string() =>
                Some(DefaultValue::String(v.as_str().unwrap().to_string())),
            v if v.is_number() =>
                Some(DefaultValue::Number(v.as_i64().unwrap())),
            v if v.is_boolean() =>
                Some(DefaultValue::Boolean(v.as_bool().unwrap())),
            v if v.is_object() =>
                Some(DefaultValue::Mapping(Self::extract_default_values_from_child_nodes(v))),
            _ => None,
        }
    }

    fn extract_default_values_from_child_nodes(v: &Value) -> Value {
        let schema_node = v.as_object()
            .expect("node is not object!")
            .get("schema")
            .expect("could not GET schema from node");

        let mut output: Value = json! {{}};
        for schema_element in schema_node.as_array()
            .expect("schema node is not array") {
            let component = Self::new(schema_element);
            output[component.name] = component.default_value.to_json();
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::component::*;

    #[test]
    fn create_component() {
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
mod extract_value_tests {
    use serde_json::Value;

    use super::*;
    use super::DefaultValue::*;

    #[test]
    fn extract_string_from_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn", "kind": "string", "default_value": "ss"}"#)
            .unwrap();
        let v = Component::extract_default_value(&v.as_object().unwrap(), "default_value");
        assert!(v.is_some());
        match v.unwrap() {
            String(s) => assert_eq!(&s, "ss"),
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn extract_number_from_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn", "kind": "number", "default_value": 10000}"#)
            .unwrap();
        let v = Component::extract_default_value(&v.as_object().unwrap(), "default_value");
        assert!(v.is_some());
        match v.unwrap() {
            Number(n) => assert_eq!(n, 10000),
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn extract_boolean_from_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn", "kind": "boolean", "default_value": false}"#)
            .unwrap();
        let v = Component::extract_default_value(&v.as_object().unwrap(), "default_value");
        assert!(v.is_some());
        match v.unwrap() {
            Boolean(b) => assert!(!b),
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn extract_object_from_payload() {
        let v: Value = serde_json::from_str(r#"{"name": "sn",
          "kind": "mapping",
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
