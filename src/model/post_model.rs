use std::collections::BTreeMap;

use actix_web::web::Data;
use anyhow::{anyhow, Result};
use surrealdb::{
    sql::{thing, Object, Value},
    Response,
};

use crate::surrealdb_repo::SurrealDBRepo;

pub struct Post {
    id: String,
    title: String,
    content: String,
    posted: String,
    author: String,
    estimated_reading_time: u32,
    order: u32,
}

pub struct PostBMC;

impl PostBMC {
    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Post>> {
        let sql = "SELECT * FROM post;";
        let ress = db.ds.execute(sql, &db.ses, None, true).await?;

        let mut posts = Vec::new();

        for post in into_iter_objects(ress)? {
            posts.push(extract_data(post?).await?);
        }

        Ok(posts)
    }

    pub async fn create(db: Data<SurrealDBRepo>, post: Post) -> Result<String> {
        let sql = "CREATE $th CONTENT $data";
        let tid = format!("post:`{}`", &post.id);

        let data: BTreeMap<String, Value> = [
            ("title".into(), post.title.into()),
            ("content".into(), post.content.into()),
            ("posted".into(), post.posted.into()),
            ("author".into(), post.author.into()),
            (
                "estimated_reading_time".into(),
                post.estimated_reading_time.into(),
            ),
            ("order".into(), post.order.into()),
        ]
        .into();

        let vars: BTreeMap<String, Value> = [
            ("th".into(), thing(&tid)?.into()),
            ("data".into(), data.into()),
        ]
        .into();

        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        into_iter_objects(ress)?
            .next()
            .transpose()?
            .and_then(|obj| obj.get("id").map(|id| id.to_string()))
            .ok_or_else(|| anyhow!("No ID returned!"))
    }

    pub async fn get(db: Data<SurrealDBRepo>, tid: &str) -> Result<Post> {
        let sql = "SELECT * FROM $th";
        let tid = format!("post:`{}`", tid);

        let vars: BTreeMap<String, Value> = [("th".into(), thing(&tid)?.into())].into();

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;
    }
}

async fn extract_data(obj: Object) -> Result<Post> {
    Ok(Post {
        id: obj.get("id").unwrap().to_string(),
        title: obj.get("title").unwrap().to_string(),
        content: obj.get("content").unwrap().to_string(),
        posted: obj.get("posted").unwrap().to_string(),
        author: obj.get("author").unwrap().to_string(),
        estimated_reading_time: obj
            .get("estimated_reading_time")
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
        order: obj
            .get("order")
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
    })
}

fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;

    match res {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(obj) => Ok(obj),
                _ => Err(anyhow!("A record was not an object!")),
            });
            Ok(it)
        }
        _ => Err(anyhow!("No records found!")),
    }
}
