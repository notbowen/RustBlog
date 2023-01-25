use std::sync::Arc;
use surrealdb::sql::Value;
use surrealdb::{Datastore, Error, Session};

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub ds: Arc<Datastore>,
    pub ses: Session,
}

impl SurrealDBRepo {
    pub async fn init() -> Result<Self, Error> {
        let ds = Arc::new(Datastore::new("memory").await?);
        let ses = Session::for_db("my_ns", "my_db");

        Ok(SurrealDBRepo { ds, ses })
    }
}
