mod post;

extern crate router;

use crate::component::post::{api::PostApi, service::PostService};
use crate::other::database::db_connect;
use router::Router;
use std::sync::Arc;

pub fn rusty_handler() -> Router {
    let mut router = Router::new();

    // Initialize libs
    let database = db_connect("data");

    // Initialize services
    let post_service = Arc::new(PostService::new());

    // Initialize apis
    let post_api = PostApi::new(post_service.clone());

    // Build routes
    post_api.routing_define(&mut router);

    router
}
