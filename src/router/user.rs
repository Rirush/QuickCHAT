use super::Result;

#[derive(FromForm)]
pub struct CheckAvailbaleArgs {
    username: String
}

#[derive(Serialize)]
pub struct CheckAvailbaleResult {
    available: bool
}

use super::Connection;
use std::ops::Deref;
use rocket_contrib::Json;
#[get("/checkAvailable?<args>")]
pub fn check_available_handler(conn: Connection, args: CheckAvailbaleArgs) -> Json<Result<CheckAvailbaleResult>> {
    use ::logic::user_management::find_user;

    let user = find_user(&args.username, &conn.deref());
    match user {
        Some(_) => Json(Result {
            error: false,
            error_info: None,
            result: Some(CheckAvailbaleResult {
                available: false
            })
        }),
        None => Json(Result {
            error: false,
            error_info: None,
            result: Some(CheckAvailbaleResult {
                available: true
            })
        })
    }
}
