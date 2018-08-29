#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate uuid;

mod pool;
mod schema;
mod router;
mod logic;
mod database;

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .manage(pool::init_pool())
        .mount("/", routes![router::index_handler]).launch();
}
