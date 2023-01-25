// Source: https://github.com/rust-awesome-app/template-app-base/blob/main/src-tauri/src/store/try_froms.rs

use crate::prelude::*;
use surrealdb::sql::{Array, Object, Value};

impl TryFrom<W<Value>> for Object {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Object, Error> {
        match val.0 {
            Value::Object(obj) => Ok(obj),
            _ => Err(Error::ValueNotOfType("Object")),
        }
    }
}

impl TryFrom<W<Value>> for Array {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Array, Error> {
        match val.0 {
            Value::Array(obj) => Ok(obj),
            _ => Err(Error::ValueNotOfType("Array")),
        }
    }
}

impl TryFrom<W<Value>> for i64 {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<i64, Error> {
        match val.0 {
            Value::Number(obj) => Ok(obj.as_int()),
            _ => Err(Error::ValueNotOfType("i64")),
        }
    }
}

impl TryFrom<W<Value>> for bool {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<bool, Error> {
        match val.0 {
            Value::False => Ok(false),
            Value::True => Ok(true),
            _ => Err(Error::ValueNotOfType("bool")),
        }
    }
}

impl TryFrom<W<Value>> for String {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<String, Error> {
        match val.0 {
            Value::Strand(strand) => Ok(strand.as_string()),
            Value::Thing(thing) => Ok(thing.to_string()),
            _ => Err(Error::ValueNotOfType("String")),
        }
    }
}
