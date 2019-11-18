mod services;

extern crate iron;

use crate::services::initialize;
use iron::prelude::*;

fn main() {
    Iron::new(initialize()).http("localhost:3000").unwrap();
}
