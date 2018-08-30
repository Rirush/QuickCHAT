mod index;
mod user;

use ::pool::Connection;

pub use self::index::*;
pub use self::user::*;

#[derive(Serialize)]
struct ErrorInformation {
    description: String,
    error_code: String
}

use serde::Serialize;
#[derive(Serialize)]
pub struct Result<T: Serialize> {
    error: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    error_info: Option<ErrorInformation>,
    #[serde(skip_serializing_if="Option::is_none")]
    result: Option<T>
}
