mod index;
mod user;
mod guard;

use ::pool::Connection;

pub use self::index::*;
pub use self::user::*;

#[derive(Serialize)]
pub struct ErrorInformation {
    pub description: String,
    pub error_code: String
}

use serde::Serialize;
#[derive(Serialize)]
pub struct Result<T: Serialize> {
    pub error: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_info: Option<ErrorInformation>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub result: Option<T>
}
