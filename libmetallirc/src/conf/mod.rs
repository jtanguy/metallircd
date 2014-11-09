//! Configuration module of the server.

#![experimental]

use logging::{LogLevel, Warning};

pub use self::cfgfile::load_config;

use std::collections::TreeMap;
use std::io::net::ip::{IpAddr, Ipv4Addr};
use toml::TomlTable;

mod cfgfile;

#[experimental]
pub struct ServerConf {
    // generic
    pub name: String,
    pub address: IpAddr,
    pub port: u16,

    // logs
    pub loglevel: LogLevel,
    pub logfile: Path,

    // threads
    pub thread_handler_count: uint,

    /// Contains the toml table of the config file, to be used by each module.
    pub modules: TreeMap<String, TomlTable>
}

#[experimental]
impl ServerConf {

    #[experimental]
    pub fn default_conf() -> ServerConf {
        ServerConf {
            name: String::new(), // no default
            address: Ipv4Addr(0,0,0,0), // no default
            port: 0u16, // no default

            // logs
            loglevel: Warning,
            logfile: from_str("./metallirc.log").unwrap(),

            // threads
            thread_handler_count: 2u,

            // rest of the config file
            modules: TreeMap::new()
        }
    }

}