use database::User;

pub struct Authorization(User);

fn check_session(username: &str, token: &str) -> bool {
    use redis::{Client, Commands};
    use std::env;

    // TODO: Maybe create some kind of connection pool
    let client = Client::open(env::var("REDIS_URL").unwrap().as_ref()).unwrap();
    let connection = client.get_connection().unwrap();

    match connection.hget::<_, _, String>(format!("session:{}:{}", username, token), "valid") {
        Err(error) => {
            println!("{}", error);
            false
        }
        Ok(valid) => valid == "true",
    }
}

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};
impl<'a, 'b> FromRequest<'a, 'b> for Authorization {
    type Error = ();

    fn from_request(request: &'a Request<'b>) -> request::Outcome<Self, Self::Error> {
        let username: Vec<_> = request.headers().get("Username").collect();
        let token: Vec<_> = request.headers().get("Token").collect();

        if username.len() != 1 || token.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        if !check_session(username[0], token[0]) {
            return Outcome::Failure((Status::Forbidden, ()));
        }
        use logic::user_management;
        use pool::Connection;
        use std::ops::Deref;
        let connection = request.guard::<Connection>()?;

        match user_management::find_user(&username[0].to_owned(), &connection.deref()) {
            Some(user) => Outcome::Success(Authorization(user)),
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}
