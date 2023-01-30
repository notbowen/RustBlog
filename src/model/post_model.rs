use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Object, Value};

use crate::prelude::*;
use crate::surrealdb_repo::SurrealDBRepo;

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub post_id: String,
    pub title: String,
    pub content: String,
    pub posted: String,
    pub author: String,
    pub estimated_reading_time: u32,
    pub order: u32,
}

impl From<Post> for Value {
    fn from(val: Post) -> Self {
        let data: BTreeMap<String, Value> = [
            ("post_id".into(), val.post_id.into()),
            ("title".into(), val.title.into()),
            ("content".into(), val.content.into()),
            ("posted".into(), val.posted.into()),
            ("author".into(), val.author.into()),
            (
                "estimated_reading_time".into(),
                val.estimated_reading_time.into(),
            ),
            ("order".into(), val.order.into()),
        ]
        .into();

        data.into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostPatch {
    pub title: Option<String>,
    pub content: Option<String>,
    pub posted: Option<String>,
    pub author: Option<String>,
    pub estimated_reading_time: Option<u32>,
    pub order: Option<u32>,
}

impl From<PostPatch> for Value {
    fn from(val: PostPatch) -> Self {
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

pub struct PostBMC;

impl PostBMC {
    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Post>, Error> {
        let sql = "SELECT * FROM post;";
        let res = db.ds.execute(sql, &db.ses, None, true).await?;

        let first_res = res.into_iter().next().expect("Able to get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn create(db: Data<SurrealDBRepo>, data: Post) -> Result<Post, Error> {
        let sql = "CREATE $tb CONTENT $data RETURN *";
        let tid = format!("post:`{}`", &data.post_id);

        let data_obj: Object = W(data.into()).try_into()?;

        let vars: BTreeMap<String, Value> = [
            ("tb".into(), thing(&tid)?.into()),
            ("data".into(), Value::from(data_obj)),
        ]
        .into();

        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_val = ress
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("Able to get ID")?;

        W(first_val.first()).try_into()
    }

    pub async fn get(db: Data<SurrealDBRepo>, id: &str) -> Result<Post, Error> {
        let sql = "SELECT * FROM $th";
        let tid = format!("post:`{id}`");

        let vars: BTreeMap<String, Value> = [("th".into(), thing(&tid)?.into())].into();

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = ress.into_iter().next().expect("Able to step into Response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn update(db: Data<SurrealDBRepo>, id: &str, data: PostPatch) -> Result<Post, Error> {
        let sql = "UPDATE $th MERGE $data RETURN *";
        let tid = format!("post:`{id}`");

        let vars = [
            ("th".into(), thing(&tid)?.into()),
            ("data".into(), data.into()),
        ]
        .into();

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = ress.into_iter().next().expect("Able to get ID");

        W(first_res.result?.first()).try_into()
    }

    pub async fn delete(db: Data<SurrealDBRepo>, id: &str) -> Result<String, Error> {
        let sql = "DELETE $th RETURN *";
        let tid = format!("post:`{id}`");

        let vars: BTreeMap<String, Value> = [("th".into(), thing(&tid)?.into())].into();

        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        ress.into_iter().next().expect("Able to get ID").result?;

        Ok(tid)
    }
}
