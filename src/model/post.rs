use crate::other::types::PgTimestamp;
use serde::Serialize;

#[derive(Serialize)]
pub struct PostList (
    pub Vec<Post>,
);

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_timestamp: PgTimestamp,
    pub updated_timestamp: PgTimestamp,
}
