//! metallircd

extern crate irccp;
extern crate uuid;

use std::io::Listener;
use std::io::net::tcp::TcpListener;

pub mod scheduling;
pub mod settings;
pub mod users;
pub mod util;

fn main() {
    //
    // CONFIG
    //
    let serverconfig = settings::ServerSettings {
        name: "irc@foo.bar".to_string(),
        address: "127.0.0.1".to_string(),
        port: 6667,
        //
        tcp_timout: 50,
        thread_handler_count: 2,
        thread_new_users_cnx_timeout: 100,
        thread_sleep_time: 100
    };

    // new clients handler
    let listener = match TcpListener::bind(serverconfig.address.as_slice(), serverconfig.port) {
        Ok(l) => l,
        Err(e) => fail!("Could not bind port: {}", e)
    };
    let acceptor = match listener.listen() {
        Ok(a) => a,
        Err(e) => fail!("Could not bind port: {}", e)
    };

    //
    // RUNNING
    //

    let srv_data = scheduling::ServerData::new(serverconfig);

    scheduling::run_server(srv_data, acceptor);

}
