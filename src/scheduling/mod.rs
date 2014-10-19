//! Scheduling operations.

//! This module contains the main threads handling the whole server workflow.
//!
//! This workflow is currently:
//!
//! - A thread handling new connections and putting them in the main workflow once
//!   negociation procedure is succesfully finished.
//! - Several (depending on configuration) threads handling user I/O: handling user
//!   commands and sending them all message they should receive.
//! - A recycler thread putting clients back in the loop after their have been handled.
//!   Also does "eavy" operations on users requiring `&mut` acces to the usermanager
//!   (currently nickname changing and user deletion).
//! - A logger thread handling server logging system.

#![experimental]

use channels::ChannelManager;
use logging::{Logger, Info};
use conf::ServerConf;
use users::UserManager;
use modules::ModulesHandler;

use std::io::net::tcp::TcpAcceptor;
use std::sync::{Arc, deque, RWLock};

use uuid::Uuid;

mod users_handling;
mod procs;

/// Contains all data of the server in a way that is safe to be shared between the server threads.
#[experimental]
pub struct ServerData {
    pub settings: RWLock<ServerConf>,
    pub users: RWLock<UserManager>,
    pub channels: RWLock<ChannelManager>,

    pub logger: Logger,
    pub signal_shutdown: RWLock<bool>,

    pub modules_handler: RWLock<ModulesHandler>
}

#[experimental]
impl ServerData {

    /// Creates the server data structure from a config.
    pub fn new(settings: ServerConf)-> ServerData {
        let logger = Logger::new(settings.loglevel);
        let modules_hdlr = ModulesHandler::init(&settings, &logger);
        ServerData {
            settings: RWLock::new(settings),
            users: RWLock::new(UserManager::new()),
            channels: RWLock::new(ChannelManager::new()),
            logger: logger,
            signal_shutdown: RWLock::new(false),
            modules_handler: RWLock::new(modules_hdlr)
        }
    }
}

/// Runs the server on given server data.
pub fn run_server(srv: ServerData, acceptor: TcpAcceptor) {

    let arc_srv = Arc::new(srv);

    let user_recycled_buffer: deque::BufferPool<Uuid> = deque::BufferPool::new();
    let (user_recycled_worker, user_recycled_stealer) = user_recycled_buffer.deque();
    let (to_recycle_sender, to_recycle_receiver) = channel();

    let mut thread_handles = Vec::new();

    // new clients handler
    thread_handles.push(
        procs::spawn_newclients_handler(arc_srv.clone(), acceptor, to_recycle_sender.clone())
    );

    // client handlers
    for i in range(1u, arc_srv.settings.read().thread_handler_count) {
        thread_handles.push(
            procs::spawn_clients_handler(arc_srv.clone(), user_recycled_stealer.clone(), to_recycle_sender.clone(), i)
        );
    }

    // Avoid deadlock ;-)
    drop(to_recycle_sender);

    arc_srv.logger.log(Info, format!("Initialised {} clients hanlders.",arc_srv.settings.read().thread_handler_count));

    // clients recycler
    thread_handles.push(
        procs::spawn_clients_recycler(arc_srv.clone(), user_recycled_worker, to_recycle_receiver)
    );

    // logger
    thread_handles.push(
        procs::spawn_logger(arc_srv.clone())
    );

    //
    // CLEANUP & JOINING
    //
    for it in thread_handles.into_iter() {
        let _ = it.unwrap();
    }
}