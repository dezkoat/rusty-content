use iron::prelude::*;
use iron::status;

pub fn post_read_all(_req: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "This should return post".to_owned())))
}

pub fn post_read(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "This should return post".to_owned())))
}

pub fn post_create(_req: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "This should return post".to_owned())))
}

pub fn post_update(_req: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "This should return post".to_owned())))
}

pub fn post_delete(_req: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "This should return post".to_owned())))
}
