#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
type PostgresPool = Pool<ConnectionManager<PgConnection>>;
static DATABASE_URL: &'static str = env!("DATABASE_URL");

fn init_pool() -> PostgresPool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("failed to create db pool")
}

use rocket::State;
use rocket::http::Status;
use rocket::{Request, Outcome};
use rocket::request::{self, FromRequest};
pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);
impl<'a, 'b> FromRequest<'a, 'b> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'b>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PostgresPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

use std::ops::Deref;
impl Deref for Connection {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[get("/")]
fn index() -> &'static str {
    "OwO\n"
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index]).launch();
}
