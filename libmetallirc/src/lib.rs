//! metallirc main library

#![feature(macro_rules,if_let,while_let)]

#![experimental]

extern crate time;
extern crate toml;
extern crate uuid;

use std::sync::RWLock;

pub mod channels;
pub mod conf;
pub mod logging;
pub mod messages;
pub mod modes;
pub mod modules;
pub mod users;
pub mod util;

/// Contains all data of the server in a way that is safe to be shared between the server threads.
#[experimental]
pub struct ServerData {
    pub settings: RWLock<conf::ServerConf>,
    pub users: RWLock<users::UserManager>,
    pub channels: RWLock<channels::ChannelManager>,

    pub logger: logging::Logger,
    pub signal_shutdown: RWLock<bool>,

    pub modules_handler: RWLock<modules::ModulesHandler>
}

#[experimental]
impl ServerData {

    /// Creates the server data structure from a config.
    pub fn new(settings: conf::ServerConf)-> ServerData {
        let logger = logging::Logger::new(settings.loglevel);
        let modules_hdlr = modules::ModulesHandler::init(&settings, &logger);
        ServerData {
            settings: RWLock::new(settings),
            users: RWLock::new(users::UserManager::new()),
            channels: RWLock::new(channels::ChannelManager::new()),
            logger: logger,
            signal_shutdown: RWLock::new(false),
            modules_handler: RWLock::new(modules_hdlr)
        }
    }
}