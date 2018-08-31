#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate dotenv;
extern crate uuid;
extern crate rand;
extern crate sha2;
extern crate hex;
#[macro_use] extern crate lazy_static;
extern crate regex;

mod pool;
mod schema;
mod router;
mod logic;
mod database;

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .manage(pool::init_pool())
        .mount("/", routes![router::index_handler])
        .mount("/user", routes![router::check_available_handler, router::authorize_user_handler,
                                router::register_user_handler])
        .launch();
}
