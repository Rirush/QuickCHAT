#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate dotenv;
#[macro_use]
extern crate diesel;
extern crate hex;
extern crate rand;
extern crate regex;
extern crate sha2;
extern crate uuid;
#[macro_use]
extern crate lazy_static;
extern crate redis;

// Module with macros
#[macro_use]
mod macros;
// Module with RESTful API methods
mod api;
// Module with API logic
mod logic;
// Module with database-related operations
mod data;
// Module with tests
#[cfg(test)]
mod tests;
// Module with database schemas (diesel)
mod schema;

fn main() {
    use api::messages::*;
    use api::sessions::*;
    use api::statuses::*;
    use api::users::*;
    rocket::ignite()
        .mount(
            "/",
            routes![
                create_user,
                get_current_user,
                update_current_user,
                delete_user,
                get_user,
                update_online,
                get_updates,
                send_message,
                get_all_messages,
                get_message,
                update_message,
                delete_message,
                create_session,
                get_all_sessions,
                delete_current_session,
                delete_session
            ],
        )
        .launch();
}
