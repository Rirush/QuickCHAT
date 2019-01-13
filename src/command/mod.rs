use std::collections::HashMap;

mod listeners;

use self::listeners::*;

type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
/// Enum containing all possble errors.
pub enum Error {
    /// Received data is too short to be used.
    DataIsTooShort,
    /// Received command is unknown to the server, or there is not handler registered to process it.
    UnkownCommand,
    /// Received data cannout be parsed because it's incomplete or malformed.
    UnparsableData,
    /// This connection already has attached user.
    AlreadyAuthorized,
    /// This connection doesn't have attached user.
    Unauthorized,
}

#[repr(u16)]
#[derive(Eq, PartialEq, Hash, Debug)]
/// Enum containing all possible commands.
pub enum Command {
    /// Authorize user and attach session to this connection.
    Authorize = 1,
    /// Destroy current session and close connection.
    DestroySession = 2,
    /// Find user by username or ID.
    FindUser = 3,
    /// Send message to user or chat.
    SendMessage = 4,
    /// Edit sent message.
    EditMessage = 5,
    /// List all messages in chat.
    ListMessages = 6,
    /// Delete sent message.
    DeleteMessage = 7,
    /// List all available chats and users.
    ListChats = 8,
}

impl Command {
    /// Constructs command from two header bytes of request.
    /// Fails if there's no such entry in enum.
    pub fn from_data(data: &Vec<u8>) -> Result<Command> {
        let mut arr: [u8; 2] = [0; 2];
        arr.copy_from_slice(&data[0..2]);
        let num = unsafe { std::mem::transmute::<[u8; 2], u16>(arr) };
        if num >= 1 && num <= 8 {
            Ok(unsafe { std::mem::transmute::<u16, Command>(num) })
        } else {
            Err(Error::UnkownCommand)
        }
    }
}

#[allow(dead_code)]
pub enum ParseData {
    MessagePack(Vec<u8>),
    JSON(String),
}

/// Enum containing all possible actions after message is processed.
pub enum Action {
    /// Continue this connection and send specified data.
    Continue(Vec<u8>),
    /// Close this connection and destroy handling thread.
    Terminate,
}

/// Struct that contains contextual information, that is attached to each connection.
pub struct Context {
    pub user: Option<()>,
}

impl Context {
    /// Construct new, empty context.
    pub fn new() -> Context {
        Context { user: None }
    }
}

/// Internal handler function signature.
type Handler = fn(&mut Context, &ParseData) -> Result<Action>;

/// Dispatcher struct that contains handlers for each command.
pub struct Dispatcher {
    handlers: HashMap<Command, Handler>,
}

impl Default for Dispatcher {
    /// Create dispatcher and register all default handlers.
    fn default() -> Dispatcher {
        let mut d = Dispatcher::new();
        d.register(Command::Authorize, handler_fn!(authorize));
        d.register(Command::DestroySession, handler_fn!(destroy_session));
        d.register(Command::FindUser, handler_fn!(find_user));
        d.register(Command::SendMessage, handler_fn!(send_message));
        d.register(Command::EditMessage, handler_fn!(edit_message));
        d.register(Command::ListMessages, handler_fn!(list_messages));
        d.register(Command::DeleteMessage, handler_fn!(delete_message));
        d.register(Command::ListChats, handler_fn!(list_chats));
        d
    }
}

impl Dispatcher {
    /// Create empty dispatcher with no handlers registered.
    pub fn new() -> Dispatcher {
        Dispatcher {
            handlers: HashMap::new(),
        }
    }

    /// Register handler in this dispatcher.
    pub fn register(&mut self, command: Command, handler: Handler) {
        self.handlers.insert(command, handler);
    }

    /// Handle binary data and return the result.
    pub fn handle_binary(&self, ctx: &mut Context, data: &Vec<u8>) -> Result<Action> {
        if data.len() < 2 {
            return Err(Error::DataIsTooShort);
        }
        match Command::from_data(&data) {
            Ok(cmd) => match self.handlers.get(&cmd) {
                Some(f) => f(ctx, &ParseData::MessagePack(data.to_vec())),
                None => Err(Error::UnkownCommand),
            },
            Err(e) => Err(e),
        }
    }
}
