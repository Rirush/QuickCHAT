use super::ParseData::{MessagePack, JSON};
use super::{Action, Context, Error, ParseData, Result};

#[derive(Deserialize)]
pub struct AuthorizeArgs {
    username: String,
    password: String
}

#[handler]
pub fn authorize(ctx: &mut Context, args: &AuthorizeArgs) -> Result<Action> {
    Ok(Action::Terminate)
}
 
#[handler]
pub fn destroy_session(ctx: &mut Context, data: &Vec<u8>) -> Result<Action> {
    Ok(Action::Terminate)
}
