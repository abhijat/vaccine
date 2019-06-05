use serde_json::Value;

use crate::datetime_parser::{datetime_from_now, is_now};
use crate::payload_item::PayloadItem;
use crate::random_values::generate_random_datetime;
use crate::value_extractors::ValueExt;
use crate::payload_item::ItemType;

pub struct DatetimeItem {
    name: String,
    format: String,
    timezone: String,
    default_value: String,
}

impl DatetimeItem {
    pub fn new(v: &Value) -> Self {
        DatetimeItem {
            name: v.get_string("name"),
            format: v.get_string("format"),
            timezone: v.get_string("timezone"),
            default_value: v.get_string("default_value"),
        }
    }
}

impl PayloadItem for DatetimeItem {
    fn default_value(&self) -> (String, Value) {
        let value = if is_now(&self.default_value) {
            let d = datetime_from_now(&self.default_value, &self.timezone);
            let string = d.format(&self.format).to_string();
            json!(string)
        } else {
            json!(self.default_value)
        };

        (self.name.clone(), value)
    }

    fn random_value(&self) -> (String, Value) {
        (self.name.clone(), json!(generate_random_datetime(&self.format, &self.timezone)))
    }

    fn item_type(&self) -> ItemType {
        ItemType::Datetime
    }
}

#[cfg(test)]
mod datetime_item {
    use chrono::{Datelike, NaiveDate, Utc};
    use chrono_tz::Asia;

    use super::*;

    fn create_datetime_item() -> DatetimeItem {
        let v: Value = serde_json::from_str(r#"{
            "default_value": "now",
            "name": "startTime",
            "format": "%Y-%m-%d",
            "timezone": "Asia/Kolkata"
        }"#).unwrap();
        DatetimeItem::new(&v)
    }

    #[test]
    fn test_datetime_item_creation() {
        let d = create_datetime_item();
        assert_eq!(d.name, "startTime");
        assert_eq!(d.timezone, "Asia/Kolkata");
        assert_eq!(d.default_value, "now");
        assert_eq!(d.format, "%Y-%m-%d");
    }

    #[test]
    fn test_datetime_item_default_value() {
        let (name, value): (String, Value) = create_datetime_item().default_value();
        assert_eq!(name, "startTime");

        let now = Utc::now().with_timezone(&Asia::Kolkata);
        let parsed = NaiveDate::parse_from_str(
            &value.as_str().unwrap(),
            "%Y-%m-%d",
        ).unwrap();

        assert_eq!(parsed.year(), now.year());
        // These may fail on time boundaries in rare cases
        assert_eq!(parsed.month(), now.month());
        assert_eq!(parsed.day(), now.day());
    }

    #[test]
    fn test_datetime_item_random_value() {
        let (name, value): (String, Value) = create_datetime_item().random_value();
        assert_eq!(name, "startTime");
        assert!(NaiveDate::parse_from_str(value.as_str().unwrap(), "%Y-%m-%d").is_ok());
    }
}
