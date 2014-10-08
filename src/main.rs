//! metallircd

extern crate irccp;
extern crate time;
extern crate toml;
extern crate uuid;

use std::io::Listener;
use std::io::net::tcp::TcpListener;

pub mod channels;
pub mod conf;
pub mod logging;
pub mod scheduling;
pub mod users;
pub mod util;

fn main() {
    //
    // CONFIG
    //
    let configfile = from_str::<Path>("./metallirc.toml").unwrap();
    let serverconfig = match conf::load_config(configfile) {
        Ok(c) => c,
        Err(e) => {println!("{}", e); return}
    };

    // new clients handler
    let listener = match TcpListener::bind(serverconfig.address.as_slice(), serverconfig.port) {
        Ok(l) => l,
        Err(e) => {println!("Could not bind port: {}", e); return}
    };
    let acceptor = match listener.listen() {
        Ok(a) => a,
        Err(e) => {println!("Could not bind port: {}", e); return}
    };

    //
    // RUNNING
    //

    let srv_data = scheduling::ServerData::new(serverconfig);

    scheduling::run_server(srv_data, acceptor);

}
