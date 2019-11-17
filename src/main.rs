mod services;

extern crate iron;

use iron::prelude::*;

use services::define_router;

fn main() {
    Iron::new(define_router()).http("localhost:3000").unwrap();
}
