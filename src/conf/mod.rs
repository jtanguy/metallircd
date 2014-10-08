//! Configuration module of the server.

#![experimental]

use logging::{LogLevel, Warning};

pub use self::cfgfile::load_config;

mod cfgfile;

#[experimental]
pub struct ServerConf {
    // generic
    pub name: String,
    pub address: String,
    pub port: u16,

    // logs
    pub loglevel: LogLevel,
    pub logfile: Path,

    // threads
    pub thread_handler_count: uint
}

#[experimental]
impl ServerConf {

    #[experimental]
    pub fn default_conf() -> ServerConf {
        ServerConf {
            name: String::new(), // no default
            address: String::new(), // no default
            port: 0u16, // no default

            // logs
            loglevel: Warning,
            logfile: from_str("./metallirc.log").unwrap(),

            // threads
            thread_handler_count: 2u
        }
    }

}