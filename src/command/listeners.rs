use super::ParseData::{MessagePack, JSON};
use super::{Action, Context, Error, ParseData, Result};

#[derive(Deserialize,Debug)]
pub struct AuthorizeArgs {
    pub username: String,
    pub password: String
}

#[derive(Serialize)]
pub struct AuthorizeResult {
    pub ok: bool,
}

#[handler]
/// Authorization handler
pub fn authorize(ctx: &mut Context, args: &AuthorizeArgs) -> Result<Action> {
    debug!("Authorize");
    debug!("{:?}", args);
    match ctx.user {
        // TODO: Actually check user data
        Some(_) => Err(Error::AlreadyAuthorized),
        None => {
            ctx.user = Some(());
            Ok(Action::Continue(rmp_serde::to_vec(&AuthorizeResult{ok: true}).unwrap()))
        }
    }
}
 
#[derive(Deserialize)]
pub struct DestroySessionArgs {}

#[handler]
/// Session destruction handler
pub fn destroy_session(ctx: &mut Context, _args: &DestroySessionArgs) -> Result<Action> {
    debug!("DestroySession");
    match ctx.user {
        Some(_) => Ok(Action::Terminate),
        None => Err(Error::Unauthorized),
    }
}
