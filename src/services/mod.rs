mod post;

extern crate router;

use router::Router;
use post::define_router as post_router;

pub fn define_router() -> Router {
    let mut router = Router::new();

    post_router(&mut router);

    router
}
