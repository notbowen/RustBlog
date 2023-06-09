use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub post_id: String,
    pub title: String,
    pub content: String,
    pub posted: String,
    pub tags: String,
    pub estimated_reading_time: u32,
    pub order: u32,
}
