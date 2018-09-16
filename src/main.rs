#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

#![allow(proc_macro_derive_resolution_fallback)]

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
extern crate redis;

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
        .catch(catchers![not_found, forbidden, authorization_required])
        .launch();
}


use router::{Result, ErrorInformation};
#[derive(Serialize)]
struct EmptyResult();

use rocket_contrib::Json;
#[catch(404)]
fn not_found() -> Json<Result<EmptyResult>> {
    Json(Result {
        error: true,
        error_info: Some(ErrorInformation {
            description: "No such method found".to_owned(),
            error_code: "NOT_FOUND".to_owned()
        }),
        result: None
    })
}

#[catch(403)]
fn forbidden() -> Json<Result<EmptyResult>> {
    Json(Result {
        error: true,
        error_info: Some(ErrorInformation {
            description: "Failed to authorize access".to_owned(),
            error_code: "FORBIDDEN".to_owned()
        }),
        result: None
    })
}

#[catch(401)]
fn authorization_required() -> Json<Result<EmptyResult>> {
    Json(Result {
        error: true,
        error_info: Some(ErrorInformation {
            description: "Authorization required".to_owned(),
            error_code: "UNAUTHORIZED".to_owned()
        }),
        result: None
    })
}
