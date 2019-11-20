mod component;
mod model;
mod other;
mod schema;

extern crate iron;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;

use crate::component::rusty_handler;
use iron::prelude::*;

fn main() {
    Iron::new(rusty_handler()).http("localhost:3000").unwrap();
}
