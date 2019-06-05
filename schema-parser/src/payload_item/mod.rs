use std::fmt::{Debug, Error, Formatter};

use serde_json::Value;

use crate::value_extractors::ValueExt;

pub mod float;
pub mod string;
pub mod number;
pub mod mapping;
pub mod boolean;
pub mod datetime;

pub enum ItemType {
    Datetime,
    Boolean,
    Mapping,
    Number,
    String,
    Float,
}

pub trait PayloadItem {
    fn default_value(&self) -> (String, Value);
    fn random_value(&self) -> (String, Value);
    fn item_type(&self) -> ItemType;
}

pub fn payload_item_from_json(v: &Value) -> Box<dyn PayloadItem> {
    let kind = v.get_string("kind");
    match kind.as_str() {
        "string" => Box::new(string::StringItem::new(&v)),
        "number" => Box::new(number::NumberItem::new(&v)),
        "float" => Box::new(float::FloatItem::new(&v)),
        "datetime" => Box::new(datetime::DatetimeItem::new(&v)),
        "mapping" => Box::new(mapping::MappingItem::new(&v)),
        "boolean" => Box::new(boolean::BooleanItem::new(&v)),
        _ => panic!(format!("invalid type {}", kind.as_str()))
    }
}

impl Debug for PayloadItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let (name, value) = self.default_value();
        f.write_str(&format!("{}: {}", name, value))
    }
}
