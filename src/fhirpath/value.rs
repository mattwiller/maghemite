use std::fmt::Display;

use super::*;
use rust_decimal::prelude::*;

pub type Type = &'static str;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Boolean(bool),
    String(String),
    Integer(i32),
    Decimal(Decimal),
    Date(chrono::NaiveDate),
    Time(chrono::NaiveTime),
    DateTime(chrono::DateTime<chrono::FixedOffset>),
    Quantity(Quantity),

    Complex(Box<DataNode>),

    Any(Box<Value>),
}

pub const BOOLEAN: Type = "System.Boolean";
pub const STRING: Type = "System.String";
pub const INTEGER: Type = "System.Integer";
pub const DECIMAL: Type = "System.Decimal";
pub const DATE: Type = "System.Date";
pub const TIME: Type = "System.Time";
pub const DATETIME: Type = "System.DateTime";
pub const QUANTITY: Type = "System.Quantity";
pub const ANY: Type = "System.Any";

#[derive(PartialEq, Debug, Clone)]
pub struct Quantity {
    value: Decimal,
    unit: String,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => f.write_fmt(format_args!("'{}'", str)),
            _ => todo!(),
        }
    }
}

impl Value {
    pub fn data_type(&self) -> Type {
        match self {
            Self::Boolean(_) => BOOLEAN,
            Self::String(_) => STRING,
            Self::Integer(_) => INTEGER,
            Self::Decimal(_) => DECIMAL,
            Self::Date(_) => DATE,
            Self::Time(_) => TIME,
            Self::DateTime(_) => DATETIME,
            Self::Quantity(_) => QUANTITY,

            Self::Complex(data) => data.data_type(),

            Self::Any(_) => ANY,
        }
    }

    pub fn equal(&self, other: &Value) -> Option<bool> {
        if self.data_type() != other.data_type() {
            return Some(false);
        }

        match self {
            Value::String(str) => {
                if let Value::String(str2) = other {
                    return Some(str == str2);
                }
                return Some(false);
            }

            _ => todo!(),
        }
    }
}
