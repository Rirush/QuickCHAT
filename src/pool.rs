use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
type PostgresPool = Pool<ConnectionManager<PgConnection>>;
use std::{env, string::String};
lazy_static! {
    static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
}
pub fn init_pool() -> PostgresPool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL.as_ref());
    Pool::new(manager).expect("failed to create db pool")
}

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::State;
use rocket::{Outcome, Request};
pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);
impl<'a, 'b> FromRequest<'a, 'b> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'b>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PostgresPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
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
