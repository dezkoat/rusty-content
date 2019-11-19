use crate::component::post::service::PostService;
use enclose::enclose;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct PostApi {
    post_service: Arc<PostService>,
}

impl PostApi {

    pub fn new(post_service: Arc<PostService>) -> Self {
        Self { post_service: post_service }
    }

    pub fn routing_define(&self, router: &mut Router) {

        router.get   ("/post",          enclose!((self => me) move |req: &mut Request| me.post_read_all(req)), "post_read_all");
        router.get   ("/post/:post_id", enclose!((self => me) move |req: &mut Request| me.post_read(req)),     "post_read");
        router.post  ("/post",          enclose!((self => me) move |req: &mut Request| me.post_create(req)),   "post_create");
        router.put   ("/post/:post_id", enclose!((self => me) move |req: &mut Request| me.post_update(req)),   "post_update");
        router.delete("/post/:post_id", enclose!((self => me) move |req: &mut Request| me.post_delete(req)),   "post_delete");

    }

    fn post_read_all(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self.post_service.post_read_all())))
    }

    fn post_read(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self.post_service.post_read())))
    }

    fn post_create(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self.post_service.post_create())))
    }

    fn post_update(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self.post_service.post_update())))
    }

    fn post_delete(&self, _req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self.post_service.post_delete())))
    }

}
