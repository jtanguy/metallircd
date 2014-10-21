//! metallircd

#![feature(macro_rules,if_let,while_let)]

extern crate argparse;
extern crate time;
extern crate toml;
extern crate uuid;

use std::io::Listener;
use std::io::net::tcp::TcpListener;
use std::os::set_exit_status;

use argparse::{ArgumentParser, Store};

pub mod channels;
pub mod conf;
pub mod logging;
pub mod messages;
pub mod modes;
pub mod modules;
pub mod scheduling;
pub mod users;
pub mod util;

fn main() {
    //
    // CONFIG
    //

    let mut cfg_path = "./metallirc.toml".to_string();

    let mut ap = ArgumentParser::new();
    ap.set_description("metallircd");
    ap.refer(&mut cfg_path).add_option(["--cfg"], box Store::<String>, "config file");
    match ap.parse_args() {
        Ok(()) => {}
        Err(x) => { set_exit_status(x); return; }
    }

    let configfile = match from_str::<Path>(cfg_path.as_slice()) {
        Some(path) => path,
        None => { println!("Invalid path for config file."); set_exit_status(1); return }
    };
    let serverconfig = match conf::load_config(configfile) {
        Ok(c) => c,
        Err(e) => { println!("{}", e); set_exit_status(1); return }
    };

    // new clients handler
    let listener = match TcpListener::bind(serverconfig.address.as_slice(), serverconfig.port) {
        Ok(l) => l,
        Err(e) => { println!("Could not bind port: {}", e); set_exit_status(1); return }
    };
    let acceptor = match listener.listen() {
        Ok(a) => a,
        Err(e) => { println!("Could not bind port: {}", e); set_exit_status(1); return }
    };

    //
    // RUNNING
    //

    let srv_data = scheduling::ServerData::new(serverconfig);

    scheduling::run_server(srv_data, acceptor);

}
