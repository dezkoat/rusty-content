mod component;
mod model;
mod other;

extern crate iron;

use crate::component::rusty_handler;
use iron::prelude::*;

fn main() {
    Iron::new(rusty_handler()).http("localhost:3000").unwrap();
}
