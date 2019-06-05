use std::collections::HashMap;

use serde_json::Value;

use crate::default_value::DefaultValue;

// Stores named entities against the response from server
#[derive(Debug)]
pub struct EntityMap {
    pub entity_map: HashMap<String, Value>
}

impl EntityMap {
    pub fn empty() -> Self {
        EntityMap { entity_map: HashMap::new() }
    }

    pub fn get(&self, name: &str, key: &str) -> Option<DefaultValue> {
        self.entity_map
            .get(name)
            .and_then(|v|
                v.as_object()
                    .expect(&format!("Value stored against {} is not map", name))
                    .get(key))
            .and_then(|v| {
                let v = match v {
                    v if v.is_number() => DefaultValue::Number(v.as_i64().unwrap()),
                    v if v.is_string() => DefaultValue::String(v.as_str().unwrap().to_string()),
                    v if v.is_boolean() => DefaultValue::Boolean(v.as_bool().unwrap()),
                    _ => panic!(format!("{} is not convertible to default value", v))
                };
                Some(v)
            })
    }

    pub fn put(&mut self, name: &str, value: &Value) {
        self.entity_map.insert(name.to_string(), value.clone());
    }
}

#[cfg(test)]
mod tests {
    use chrono::format::Item::Fixed;

    use super::*;

    fn fixture() -> EntityMap {
        let mut em = EntityMap::empty();
        em.put("foo", &json!({
            "name": "laptop",
            "brand": "thinkpad",
            "ram_in_gb": 16,
            "efi_boot": false
        }));

        em
    }

    #[test]
    fn test_get_existing_name_existing_key() {
        let em = fixture();
        let ram = em.get("foo", "ram_in_gb");
        assert!(ram.is_some());
        match ram.unwrap() {
            DefaultValue::Number(n) => assert_eq!(n, 16),
            _ => panic!("failed")
        }
    }

    #[test]
    fn test_get_existing_name_missing_key() {
        let em = fixture();
        let foo = em.get("foo", "bar");
        assert!(foo.is_none());
    }

    #[test]
    fn test_get_missing_name() {
        let em = fixture();
        let foo = em.get("bar", "foo");
        assert!(foo.is_none());
    }
}
