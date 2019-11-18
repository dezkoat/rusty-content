mod post;
mod database;

extern crate router;

use crate::services::database::db_connect;
use mongodb::db::Database;
use router::Router;
use post::initialize as post_initialize;

struct Service {
    pub router: Router,
    pub db: Database
}

impl Service {

    fn new(router: Router, db: Database) -> Service {
        Service { router: router, db: db }
    }

}

pub fn initialize() -> Service {
    let service = Service::new(
        Router::new(),
        db_connect("data")
    );

    post_initialize(&service);

    service
}
