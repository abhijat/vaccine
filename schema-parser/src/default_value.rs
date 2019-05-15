use serde_json::Value;

use crate::datetime::{datetime_from_now, is_now};

#[derive(Debug)]
pub enum DefaultValue {
    String(String),
    Number(i64),
    Boolean(bool),
    Mapping(Value),
    Datetime {
        format: String,
        default: String,
        timezone: String,
    },
}

impl DefaultValue {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            DefaultValue::String(s) => json!(s),
            DefaultValue::Number(i) => json!(i),
            DefaultValue::Boolean(b) => json!(b),
            DefaultValue::Mapping(m) => m.clone(),
            DefaultValue::Datetime { format, default, timezone } =>
                Self::datetime_to_json(format, default, timezone)
        }
    }

    fn datetime_to_json(format: &str, default: &str, timezone: &str) -> serde_json::Value {
        if is_now(default) {
            let d = datetime_from_now(default, timezone);
            json!(d.format(format).to_string())
        } else {
            json!(default)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn test_json_conversion_of_datetime_variant() {
        let d = DefaultValue::Datetime {
            format: "%Y-%m-%d %H::%M::%S".to_string(),
            default: "now".to_string(),
            timezone: "Asia/Kolkata".to_string()
        };

        let json_value = d.to_json();
        eprintln!("json_value = {:?}", json_value);
    }
}
