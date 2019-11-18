use crate::services::post::service::PostService;
use iron::prelude::*;
use iron::status;

pub struct PostApi {
    service: PostService,
}

impl PostApi {

    pub fn new(service: PostService) -> PostApi {
      PostApi { service: service }
    }

    pub fn post_read_all(&self, req: &mut Request) -> IronResult<Response> {
        let posts = self.service.post_read_all();
        Ok(Response::with((status::Ok, posts)))
    }

    pub fn post_read(&self) -> impl Fn(&mut Request) -> IronResult<Response> {
        move |req| Ok(Response::with((status::Ok, "This should return post".to_owned())))
    }

    pub fn post_create(&self) -> impl Fn(&mut Request) -> IronResult<Response> {
        move |req| Ok(Response::with((status::Ok, "This should return post".to_owned())))
    }

    pub fn post_update(&self) -> impl Fn(&mut Request) -> IronResult<Response> {
        move |req| Ok(Response::with((status::Ok, "This should return post".to_owned())))
    }

    pub fn post_delete(&self) -> impl Fn(&mut Request) -> IronResult<Response> {
        move |req| Ok(Response::with((status::Ok, "This should return post".to_owned())))
    }

}