mod services;

extern crate iron;

use crate::services::initialize;
use iron::prelude::*;

fn main() {
    let service = initialize();
    Iron::new(service.router).http("localhost:3000").unwrap();
}
