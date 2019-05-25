use serde_json::{Map, Value};

use crate::default_value::DefaultValue;
use crate::entity_map::EntityMap;
use crate::extract_string_from_value;

pub fn extract_datetime_from_node(node: &Map<String, Value>) -> DefaultValue {
    let timezone = extract_string_from_value(node, "timezone");
    let format = extract_string_from_value(node, "format");
    let default = extract_string_from_value(node, "default_value");

    DefaultValue::Datetime { format, timezone, default }
}

pub fn extract_default_value_from_node(node: &Map<String, Value>, entity_map: &EntityMap)
                                       -> DefaultValue {
    let relation = extract_string_from_value(node, "relation");
    let fieldname = extract_string_from_value(node, "fieldname");

    entity_map.get(relation.as_str(), fieldname.as_str())
        .expect(&format!("failed to find {}.{} in entity map", relation, fieldname))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_extract_default_value_from_node() {
        let mut em = EntityMap::empty();
        em.put("foo", &json!({
            "name": "alpha",
            "category": "beta",
            "id": 1112
        }));

        let node: Value = json!({
            "kind": "relationship",
            "relation": "foo",
            "fieldname": "id"
        });

        let d = extract_default_value_from_node(node.as_object().unwrap(), &em);
        match d {
            DefaultValue::Number(n) => assert_eq!(n, 1112),
            _ => panic!()
        }
    }
}
