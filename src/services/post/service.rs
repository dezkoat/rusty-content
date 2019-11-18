use mongodb::db::Database;

pub struct PostService {
    db: Database,
}

impl PostService {

    pub fn new(db: Database) -> PostService {
        PostService { db: db }
    }

    pub fn post_read_all(&mut self) -> String {
        String::from("Test")
    }

}