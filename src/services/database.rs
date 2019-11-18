use mongodb::db::Database;
use mongodb::{Client, ThreadedClient};

pub fn db_connect(db_name: &str) -> Database {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    client.db(db_name)
}
