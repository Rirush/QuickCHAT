#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate serde;
extern crate simplelog;
extern crate structopt;
extern crate threadpool;
extern crate websocket;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;
extern crate serde_json;
#[macro_use]
extern crate handler_macro;

mod command;

use crate::command::Dispatcher;
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "quickchat")]
pub struct Arguments {
    /// Limits logging output to essential information, warnings, and errors
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    /// Initializes database, removing all its contents
    #[structopt(long = "init")]
    database_init: bool,
    /// Sets server port
    #[structopt(short = "p", long = "port", default_value = "8080")]
    port: u16,
    /// Writes logs to file, optional
    #[structopt(short = "F", long = "log_file", parse(from_os_str))]
    log_file: Option<std::path::PathBuf>,
    /// Specifies path to database
    #[structopt(short = "D", long = "database", parse(from_os_str))]
    database: Option<std::path::PathBuf>,
}

fn get_binary_path() -> std::path::PathBuf {
    let mut p = std::env::current_exe().unwrap();
    p.pop();
    p
}

fn main() {
    // Parse command-line arguments
    let args = Arguments::from_args();

    {
        use simplelog::{Config, LevelFilter, TermLogger, WriteLogger};

        // Set logging level, depending on arguments
        let mut log_level = LevelFilter::Debug;
        if args.quiet {
            log_level = LevelFilter::Info;
        }

        // Initialize logger
        let mut arr: Vec<Box<simplelog::SharedLogger>> =
            vec![TermLogger::new(log_level, simplelog::Config::default()).unwrap()];
        if let Some(f) = args.log_file {
            arr.push(WriteLogger::new(
                LevelFilter::Debug,
                Config::default(),
                File::create(f).unwrap(),
            ));
        }
        simplelog::CombinedLogger::init(arr).unwrap();
    }

    debug!("Binary path = {}", get_binary_path().to_str().unwrap());

    // Open database
    let database = match args.database {
        Some(p) => p,
        None => {
            let mut p = get_binary_path();
            p.push("database.sqlite");
            p
        }
    };
    info!("Opening database in {}", database.to_str().unwrap());
    let manager = r2d2_sqlite::SqliteConnectionManager::file(database);
    let _pool = r2d2::Pool::new(manager).unwrap();
    // Initialize database if asked
    if args.database_init {
        info!("Intializing database");
    }

    // Bind socket to port
    info!("Starting WebSocket listener on port {}", args.port);
    use websocket::sync::Server;
    let server = Server::bind(format!("0.0.0.0:{}", args.port)).unwrap();
    info!("Ready");

    // Accept connections
    for request in server.filter_map(Result::ok) {
        use std::thread;
        thread::spawn(move || {
            debug!("Thread {:?} spawned", thread::current().id());
            // Requesting client must mark 'quickchat' as supported protocol
            if !request.protocols().contains(&"quickchat".to_string()) {
                warn!("Incoming request doesn't support 'quickchat' protocol, rejecting");
                match request.reject() {
                    Ok(_) => {
                        trace!("Rejected connection successfully");
                    }
                    Err(e) => {
                        // I don't know when this kind of error may happen
                        error!("Error occurred while rejecting connection: {}", e.1);
                    }
                }
            } else {
                // If everything is okay, we can accept connection
                match request.use_protocol("quickchat").accept() {
                    Ok(client) => {
                        let client_ip = client.peer_addr().unwrap();
                        info!("Received connection from {}", client_ip);
                        if let Ok((mut rx, mut tx)) = client.split() {
                            use crate::command::Context;
                            let mut ctx = Context::new();
                            // Load command processors
                            let dispatcher = Dispatcher::default();
                            for message in rx.incoming_messages() {
                                match message {
                                    Ok(message) => {
                                        info!("Received message from {}", client_ip);
                                        use websocket::OwnedMessage;
                                        match message {
                                            // Respong to pings with pongs
                                            OwnedMessage::Ping(p) => {
                                                debug!("Message is ping");
                                                let msg = OwnedMessage::Pong(p);
                                                if let Err(e) = tx.send_message(&msg) {
                                                    error!("Error occurred while responding to ping: {}", e)
                                                }
                                            }
                                            // Close connection if peer is asked for it
                                            OwnedMessage::Close(_) => {
                                                debug!("Message is close");
                                                let msg = OwnedMessage::Close(None);
                                                info!(
                                                    "Peer {} asked to close connection",
                                                    client_ip
                                                );
                                                if let Err(e) = tx.send_message(&msg) {
                                                    error!("Error occurred while responding to close message: {}", e);
                                                }
                                                break;
                                            }
                                            // Process binary messages
                                            OwnedMessage::Binary(data) => {
                                                debug!("Message is binary");
                                                debug!("Processing message");
                                                let result =
                                                    dispatcher.handle_binary(&mut ctx, &data);
                                                match result {
                                                    Ok(response) => {
                                                        debug!("Processing returned ok");
                                                        use crate::command::Action::{
                                                            Continue, Terminate,
                                                        };
                                                        match response {
                                                            Continue(data) => {
                                                                info!("Command processor asked to continue connection");
                                                                if let Err(e) = tx.send_message(
                                                                    &OwnedMessage::Binary(data),
                                                                ) {
                                                                    error!("Error ocurred while responding to message: {}", e);
                                                                } else {
                                                                    info!("Message has been successfully processed");
                                                                }
                                                            }
                                                            Terminate => {
                                                                info!("Command processor asked to terminate connection");
                                                                if let Err(e) = tx.send_message(
                                                                    &OwnedMessage::Close(None),
                                                                ) {
                                                                    error!("Error occurred while notifying peer about termination: {}", e);
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Err(_e) => {
                                                        // TODO: Handle this error and return it to client
                                                        debug!("Processing returned error");
                                                        //warn!("Command processor returned error after processing message: {}", e);
                                                    }
                                                }
                                            }
                                            // Terminate connection on everything else
                                            _ => {
                                                debug!("Message is unknown, this is unsupported");
                                                let msg = OwnedMessage::Close(None);
                                                info!("Peer {} sent unsupported message, terminating socket", client_ip);
                                                if let Err(e) = tx.send_message(&msg) {
                                                    error!("Error occurred while notifying peer about termination: {}", e);
                                                }
                                                break;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Error occurred while unwrapping message: {}", e);
                                    }
                                }
                            }
                        } else {
                            error!("Failed to split socket into rx and tx");
                        }
                    }
                    Err(e) => {
                        // No idea how we can fail acception of a connection
                        error!("Error occurred while accepting connection: {}", e.1)
                    }
                };
            }
            debug!("Thread {:?} destroyed", thread::current().id());
        });
    }
}
