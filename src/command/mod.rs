use std::collections::HashMap;

mod listeners;

use self::listeners::*;

type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    DataIsTooShort,
    UnkownCommand,
}

#[repr(u16)]
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Command {
    Authorize = 1,
    DestroySession = 2,
}

impl Command {
    pub fn from_data(data: &Vec<u8>) -> Result<Command> {
        let mut arr: [u8; 2] = [0; 2];
        arr.copy_from_slice(&data[0..2]);
        let num = unsafe { std::mem::transmute::<[u8; 2], u16>(arr) };
        if num >= 1 && num <= 2 {
            Ok(unsafe { std::mem::transmute::<u16, Command>(num) })
        } else {
            Err(Error::UnkownCommand)
        }
    }
}

pub enum ParseData {
    MessagePack(Vec<u8>),
    JSON(String),
}

pub enum Action {
    Continue(Vec<u8>),
    Terminate,
}

pub struct Context {
    pub user: Option<()>,
}

impl Context {
    pub fn new() -> Context {
        Context { user: None }
    }
}

type Handler = fn(&mut Context, &ParseData) -> Result<Action>;

pub struct Dispatcher {
    handlers: HashMap<Command, Handler>,
}

impl Default for Dispatcher {
    fn default() -> Dispatcher {
        let mut d = Dispatcher::new();
        d.register(Command::Authorize, handler_fn!(authorize));
        d.register(Command::DestroySession, handler_fn!(destroy_session));
        d
    }
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, command: Command, handler: Handler) {
        self.handlers.insert(command, handler);
    }

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

/*
type CommandProcessor = fn(&Context, &Vec<u8>) -> Result<Action>;

pub struct Processor {
    pub functions: HashMap<Command, CommandProcessor>,
}

impl Processor {
    pub fn new() -> Processor {
        let mut p = Processor {
            functions: HashMap::new(),
        };
        p.register(Command::Authorize, processor::authorize);
        p.register(Command::DestroySession, processor::destroy_session);
        p
    }

    pub fn register(&mut self, command: Command, processor: CommandProcessor) {
        debug!("Listener for command {:?} registered", command);
        self.functions.insert(command, processor);
    }

    pub fn process(&self, ctx: &mut Context, data: &Vec<u8>) -> Result<Action> {
        debug!("Command processing started");
        if data.len() < 2 {
            debug!("Data is too short");
            return Err(Error::DataIsTooShort);
        }
        match Command::from_data(&data) {
            Ok(cmd) => {
                debug!("Searching for command processor");
                match self.functions.get(&cmd) {
                    Some(f) => {
                        debug!("Processor found");
                        f(&ctx, &data[2..].to_vec())
                    }
                    None => {
                        debug!("Processor doesn't exist");
                        Err(Error::UnkownCommand)
                    }
                }
            }
            Err(e) => Err(e),
        }
    }
}
*/
