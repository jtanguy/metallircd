//! metallircd

#![feature(macro_rules,if_let,while_let)]

extern crate getopts;
extern crate time;
extern crate toml;
extern crate uuid;

use std::io::Listener;
use std::io::net::tcp::TcpListener;
use std::os;

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

    // handle options
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    let opts = [
        getopts::optopt("c", "config", "set location of config file", "FILE"),
        getopts::optflag("h", "help", "display this help message")
    ];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };

    if matches.opt_present("h") {
        let brief = getopts::short_usage(program.as_slice(), opts.as_slice());
        println!("{}", getopts::usage(brief.as_slice(), opts.as_slice()))
        return;
    }

    let cfg_path = match matches.opt_str("c") {
        Some(s) => s,
        None => "./metallirc.toml".to_string()
    };

    // load config file
    let configfile = match from_str::<Path>(cfg_path.as_slice()) {
        Some(path) => path,
        None => { println!("Invalid path for config file."); os::set_exit_status(1); return }
    };
    let serverconfig = match conf::load_config(configfile) {
        Ok(c) => c,
        Err(e) => { println!("{}", e); os::set_exit_status(1); return }
    };

    // new clients handler
    let listener = match TcpListener::bind(serverconfig.address.as_slice(), serverconfig.port) {
        Ok(l) => l,
        Err(e) => { println!("Could not bind port: {}", e); os::set_exit_status(1); return }
    };
    let acceptor = match listener.listen() {
        Ok(a) => a,
        Err(e) => { println!("Could not bind port: {}", e); os::set_exit_status(1); return }
    };

    //
    // RUNNING
    //

    println!("Server initialised and running.")

    let srv_data = scheduling::ServerData::new(serverconfig);

    scheduling::run_server(srv_data, acceptor);

}
