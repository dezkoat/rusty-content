mod api;
mod service;

use crate::services::post::api::PostApi;
use router::Router;

pub fn initialize(router: &mut Router) {

    router.get   ("/post", PostApi::post_read_all, "post_read_all");
    // router.get   ("/post/:post_id", post_api.post_read(),     "post_read");
    // router.post  ("/post",          post_api.post_create(),   "post_create");
    // router.put   ("/post/:post_id", post_api.post_update(),   "post_update");
    // router.delete("/post/:post_id", post_api.post_delete(),   "post_delete");
}
