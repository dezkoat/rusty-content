mod post;
mod database;

extern crate router;

use crate::services::post::initialize as post_initialize;
use router::Router;

pub fn initialize() -> Router {
    let mut router = Router::new();

    post_initialize(&mut router);

    router
}
