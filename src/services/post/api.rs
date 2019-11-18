pub mod PostApi {

    use iron::prelude::*;
    use iron::status;

    pub fn post_read_all(_req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "asdsad")))
    }

}
