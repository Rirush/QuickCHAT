#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate dotenv;
extern crate hex;
extern crate rand;
extern crate sha2;
extern crate uuid;
#[macro_use]
extern crate lazy_static;
extern crate redis;
extern crate regex;

mod database;
mod logic;
mod pool;
#[macro_use]
mod router;
mod schema;
#[cfg(test)]
mod tests;

fn main() {
    dotenv::dotenv().ok();
    construct_rocket().launch();
}

use rocket::Rocket;
fn construct_rocket() -> Rocket {
    rocket::ignite()
        .manage(pool::init_pool())
        .mount("/", routes![router::index_handler])
        .mount(
            "/user",
            routes![
                router::check_username_handler,
                router::register_user_handler
            ],
        )
        .mount("/session", routes![router::create_session_handler])
        .catch(catchers![not_found, forbidden, authorization_required])
}

use router::{ErrorInformation, Result};
#[derive(Serialize)]
struct EmptyResult();

use rocket_contrib::Json;
#[catch(404)]
fn not_found() -> Json<Result<EmptyResult>> {
    method_error![code = "NOT_FOUND", details = "No such method found"]
}

#[catch(403)]
fn forbidden() -> Json<Result<EmptyResult>> {
    method_error![code = "FORBIDDEN", details = "Failed to authorize access"]
}

#[catch(401)]
fn authorization_required() -> Json<Result<EmptyResult>> {
    method_error![code = "UNAUTHORIZED", details = "Authorization required"]
}
