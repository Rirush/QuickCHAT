use super::ParseData::{MessagePack, JSON};
use super::{Action, Context, Error, ParseData, Result};

#[derive(Deserialize, Debug)]
pub struct AuthorizeArgs {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthorizeResult {
    pub ok: bool,
}

#[handler]
/// Authorization handler.
pub fn authorize(ctx: &mut Context, args: &AuthorizeArgs) -> Result<Action> {
    debug!("Authorize");
    debug!("{:?}", args);
    match ctx.user {
        // TODO: Actually check user data
        Some(_) => Err(Error::AlreadyAuthorized),
        None => {
            ctx.user = Some(());
            Ok(Action::Continue(
                rmp_serde::to_vec(&AuthorizeResult { ok: true }).unwrap(),
            ))
        }
    }
}

#[derive(Deserialize)]
pub struct DestroySessionArgs {}

#[handler]
/// Session destruction handler.
pub fn destroy_session(ctx: &mut Context, _args: &DestroySessionArgs) -> Result<Action> {
    debug!("DestroySession");
    match ctx.user {
        Some(_) => Ok(Action::Terminate),
        None => Err(Error::Unauthorized),
    }
}

#[derive(Deserialize)]
pub struct FindUserArgs {}

#[handler]
/// User search handler.
pub fn find_user(_ctx: &mut Context, _data: &FindUserArgs) -> Result<Action> {
    Err(Error::UnkownCommand)
}

#[derive(Deserialize)]
pub struct SendMessageArgs {}

#[handler]
/// Send message handler.
pub fn send_message(_ctx: &mut Context, _data: &SendMessageArgs) -> Result<Action> {
    Err(Error::UnkownCommand)
}

#[derive(Deserialize)]
pub struct EditMessageArgs {}

#[handler]
/// Edit message handler.
pub fn edit_message(_ctx: &mut Context, _data: &EditMessageArgs) -> Result<Action> {
    Err(Error::UnkownCommand)
}

#[derive(Deserialize)]
pub struct ListMessagesArgs {}

#[handler]
/// Message listing handler.
pub fn list_messages(_ctx: &mut Context, _data: &ListMessagesArgs) -> Result<Action> {
    Err(Error::UnkownCommand)
}

#[derive(Deserialize)]
pub struct DeleteMessageArgs {}

#[handler]
/// Message deletion handler.
pub fn delete_message(_ctx: &mut Context, _data: &DeleteMessageArgs) -> Result<Action> {
    Err(Error::UnkownCommand)
}

#[derive(Deserialize)]
pub struct ListChatsArgs {}

#[handler]
/// Chat listing handler.
pub fn list_chats(_ctx: &mut Context, _data: &ListChatsArgs) -> Result<Action> {
    Err(Error::UnkownCommand)
}
