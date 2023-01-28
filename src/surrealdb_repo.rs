use std::sync::Arc;
use surrealdb::{Datastore, Error, Session};

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub ds: Arc<Datastore>,
    pub ses: Session,
}

impl SurrealDBRepo {
    pub async fn init() -> Result<Self, Error> {
        let ds = Arc::new(Datastore::new("file:///mnt/blog_data/blog.db").await?);
        let ses = Session::for_db("my_ns", "my_db");

        Ok(SurrealDBRepo { ds, ses })
    }
}
