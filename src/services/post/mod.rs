mod api;
mod service;

use iron::Request;
use crate::services::post::api::PostApi;
use crate::services::post::service::PostService;
use crate::services::Service;

pub fn initialize(service: &Service) {

    // Initialize services
    let post_service = PostService::new(service.db);

    // Initialize Apis
    let post_api = PostApi::new(post_service);
    // let post = &post_api;

    let router = &service.router;
    router.get   ("/post",          |req: &mut Request| post_api.post_read_all(req), "post_read_all");
    // router.get   ("/post/:post_id", post_api.post_read(),     "post_read");
    // router.post  ("/post",          post_api.post_create(),   "post_create");
    // router.put   ("/post/:post_id", post_api.post_update(),   "post_update");
    // router.delete("/post/:post_id", post_api.post_delete(),   "post_delete");

}
