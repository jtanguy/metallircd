//! metallircd

#![feature(macro_rules,if_let,while_let)]

extern crate getopts;
extern crate time;
extern crate toml;
extern crate uuid;

extern crate metallirc;

use std::io::Listener;
use std::io::net::tcp::TcpListener;
use std::io::net::ip::SocketAddr;
use std::os;

use metallirc::{conf, ServerData};

mod scheduling;

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
        Err(f) => { panic!(f.to_string()) }
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
    let listener = match TcpListener::bind(
        SocketAddr {
            ip: serverconfig.address,
            port: serverconfig.port
        }) {
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

    let srv_data = ServerData::new(serverconfig);

    if let Some(config) = srv_data.settings.read().modules.get(&"core".to_string()) {
        // core module must always be loaded first
        srv_data.modules_handler.write().open_module("core", config, &srv_data.logger);
    }

    for (name, config) in srv_data.settings.read().modules.iter() {
        if name.as_slice() != "core" {
            srv_data.modules_handler.write().open_module(name.as_slice(), config, &srv_data.logger);
        }
    }

    scheduling::run_server(srv_data, acceptor);

}
