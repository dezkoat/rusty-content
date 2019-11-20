use diesel::dsl::insert_into;
use crate::schema::post::dsl::*;
use crate::model::post::{Post, PostList};
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

pub struct PostService {
    db_connection: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl PostService {

    pub fn new(db_connection: Arc<Pool<ConnectionManager<PgConnection>>>) -> PostService {
        PostService { db_connection: db_connection }
    }

    pub fn post_read_all(&self) -> String {
        let connection = self.db_connection.get().expect("Failed to connect to DB");
        let posts = PostList(post.limit(10).load::<Post>(&connection).expect("Failed to get post"));

        serde_json::to_string(&posts).unwrap()
    }

    pub fn post_read(&self) -> String {
        String::from("read")
    }

    pub fn post_create(&self) -> String {
        let connection = self.db_connection.get().expect("Failed to connect to DB");
        let result = insert_into(post).values((id.eq("1234"), title.eq("ABC"), content.eq("ASASA"))).execute(&connection).unwrap();

        if result > 0 {
            "Ok".to_owned()
        } else {
            "Fail".to_owned()
        }
    }

    pub fn post_update(&self) -> String {
        String::from("Test")
    }

    pub fn post_delete(&self) -> String {
        String::from("Test")
    }
}