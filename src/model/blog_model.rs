#![allow(unused)]
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Object, Value};
use surrealdb::Val;

use crate::prelude::*;
use crate::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};
use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    pub id: String,
    pub title: String,
    pub content: String,
    pub posted: String,
    pub author: String,
    pub estimated_reading_time: u32,
    pub order: u32,
}

impl From<Blog> for Value {
    fn from(val: Blog) -> Self {
        map![
            "id".into() => val.id.into(),
            "title".into() => val.title.into(),
            "content".into() => val.content.into(),
            "posted".into() => val.posted.into(),
            "author".into() => val.author.into(),
            "estimated_reading_time".into() => val.estimated_reading_time.into(),
            "order".into() => val.order.into(),
        ]
        .into()
    }
}

impl Creatable for Blog {}

pub struct BlogPatch {
    pub title: Option<String>,
    pub content: Option<String>,
    pub posted: Option<String>,
    pub author: Option<String>,
    pub estimated_reading_time: Option<u32>,
    pub order: Option<u32>,
}

impl From<BlogPatch> for Value {
    fn from(val: BlogPatch) -> Self {
        let mut value: BTreeMap<String, Value> = BTreeMap::new();

        if let Some(t) = val.title {
            value.insert("title".into(), t.into());
        }

        if let Some(c) = val.content {
            value.insert("content".into(), c.into());
        }

        if let Some(p) = val.posted {
            value.insert("posted".into(), p.into());
        }

        if let Some(a) = val.author {
            value.insert("author".into(), a.into());
        }

        if let Some(e) = val.estimated_reading_time {
            value.insert("estimated_reading_time".into(), e.into());
        }

        if let Some(o) = val.order {
            value.insert("order".into(), o.into());
        }

        Value::from(value)
    }
}

impl Patchable for BlogPatch {}

pub struct BlogBMC;

impl BlogBMC {
    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
        let sql = "SELECT * FROM blog;";
        let res = db.ds.execute(sql, &db.ses, None, true).await?;

        let first_res = res.into_iter().next().expect("Expected to get a response!");
        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn create<T: Creatable>(
        db: Data<SurrealDBRepo>,
        tb: &str,
        id: String,
        data: T,
    ) -> Result<Object, Error> {
        let sql = format!("CREATE {}:{} CONTENT $data RETURN *", tb, id);

        let data: Object = W(data.into()).try_into()?;

        let vars: BTreeMap<String, Value> = map![
            "data".into() => Value::from(data),
        ];

        let ress = db.ds.execute(&sql, &db.ses, Some(vars), false).await?;

        let first_val = ress
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("Expected ID to be returned")?;

        W(first_val.first()).try_into()
    }

    pub async fn get(db: Data<SurrealDBRepo>, tid: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM $th";

        let tid = format!("blog:{}", tid);
        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
        let first_res = ress.into_iter().next().expect("Able to get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn update<T: Patchable>(
        db: Data<SurrealDBRepo>,
        tid: &str,
        data: T,
    ) -> Result<Object, Error> {
        let sql = "UPDATE $th MERGE $data RETURN *";

        let tid = format!("blog:{}", tid);
        let vars: BTreeMap<String, Value> = map![
            "th".into() => thing(&tid)?.into(),
            "data".into() => data.into(),
        ];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
        let first_res = ress.into_iter().next().expect("Able to get ID");

        let result = first_res.result?;

        W(result.first()).try_into()
    }

    pub async fn delete(db: Data<SurrealDBRepo>, tid: &str) -> Result<String, Error> {
        let sql = "DELETE $th RETURN *";

        let tid = format!("blog:{}", tid);
        let vars: BTreeMap<String, Value> = map![
            "th".into() => thing(&tid)?.into(),
        ];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;
        let first_res = ress.into_iter().next().expect("Able to get returned ID");

        first_res.result?;

        Ok(tid)
    }
}
