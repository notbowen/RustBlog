use crate::{model::post_model::Post, prelude::*};
use surrealdb::sql::{Array, Object, Value};

impl TryFrom<W<Value>> for Object {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Object, Error> {
        match val.0 {
            Value::Object(obj) => Ok(obj),
            _ => Err(Error::XValueNotOfType("Object")),
        }
    }
}

impl TryFrom<W<Value>> for Array {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Array, Error> {
        match val.0 {
            Value::Array(arr) => Ok(arr),
            _ => Err(Error::XValueNotOfType("Array")),
        }
    }
}

impl TryFrom<W<Value>> for i64 {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<i64, Error> {
        match val.0 {
            Value::Number(int) => Ok(int.as_int()),
            _ => Err(Error::XValueNotOfType("i64")),
        }
    }
}

impl TryFrom<W<Value>> for bool {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<bool, Error> {
        match val.0 {
            Value::False => Ok(false),
            Value::True => Ok(true),
            _ => Err(Error::XValueNotOfType("bool")),
        }
    }
}

impl TryFrom<W<Value>> for String {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<String, Error> {
        match val.0 {
            Value::Strand(strand) => Ok(strand.as_string()),
            Value::Thing(thing) => Ok(thing.to_string()),
            _ => Err(Error::XValueNotOfType("String")),
        }
    }
}

impl TryFrom<W<Value>> for Post {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Self, Self::Error> {
        let post_obj: Object = val.try_into()?;

        let post = Post {
            post_id: post_obj
                .get("post_id")?
                .to_string()
                .strip_prefix('\"')?
                .strip_suffix('\"')?
                .to_string(),
            author: post_obj
                .get("author")?
                .to_string()
                .strip_prefix('\"')?
                .strip_suffix('\"')?
                .to_string(),
            content: post_obj
                .get("content")?
                .to_string()
                .strip_prefix('\"')?
                .strip_suffix('\"')?
                .to_string(),
            posted: post_obj
                .get("posted")?
                .to_string()
                .strip_prefix('\"')?
                .strip_suffix('\"')?
                .to_string(),
            title: post_obj
                .get("title")?
                .to_string()
                .strip_prefix('\"')?
                .strip_suffix('\"')?
                .to_string(),
            estimated_reading_time: post_obj
                .get("estimated_reading_time")?
                .to_string()
                .parse::<u32>()?,
            order: post_obj.get("order")?.to_string().parse::<u32>()?,
        };

        Ok(post)
    }
}
