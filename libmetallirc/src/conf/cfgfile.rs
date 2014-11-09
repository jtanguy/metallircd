//! Config file parser.

#![experimental]

use super::ServerConf;
use logging;


use std::io::fs::File;

use toml;

/// Loads a config file. Expected format with default values
/// ```
/// [metallircd]
/// server_name = <needed>
/// address = <needed>
/// port = <needed>
/// loglevel = "Warning"
/// logfile = "./metallircd.log"
/// workers = 2
///
/// [[module]]
/// name = "mod_name"
/// path = "path/to/mod.so"
/// <any other option>
/// ```

pub fn load_config(file: Path) -> Result<ServerConf,String> {
    let mut fhandle = match File::open(&file) {
        Ok(f) => f,
        Err(e) => return Err(format!("Unable to open config file {} : {}", file.display(), e))
    };

    let str_content = match fhandle.read_to_string() {
        Ok(s) => s,
        Err(e) => return Err(format!("Unable to read config file {} : {}", file.display(), e))
    };

    let toml_table = match toml::Parser::new(str_content.as_slice()).parse() {
        Some(t) => t,
        None => return Err(format!("Unable to parse config file {} : invalid Toml.", file.display()))
    };

    let mut config = ServerConf::default_conf();

    // [metallircd]
    match toml_table.get(&"metallircd".to_string()) {
        Some(&toml::Table(ref ircd_table)) => {
            config.name = match ircd_table.get(&"server_name".to_string()) {
                Some(&toml::String(ref s)) => s.clone(),
                _ => return Err(
                    format!("Error parsing config file {} : missing or invalid metallircd.server_name", file.display())
                )
            };
            config.address = match ircd_table.get(&"address".to_string()) {
                Some(&toml::String(ref s)) => match from_str(s.as_slice()) {
                    Some(addr) => addr,
                    None => return Err(
                    format!("Error parsing config file {} : invalid metallircd.address {}", file.display(), s)
                )
                },
                _ => return Err(
                    format!("Error parsing config file {} : missing or invalid metallircd.address", file.display())
                )
            };
            config.port = match ircd_table.get(&"port".to_string()) {
                Some(&toml::Integer(i)) => i as u16,
                _ => return Err(
                    format!("Error parsing config file {} : missing or invalid metallircd.address", file.display())
                )
            };
            match ircd_table.get(&"loglevel".to_string()) {
                Some(&toml::String(ref s)) => match s.as_slice() {
                    "Debug" => config.loglevel = logging::Debug,
                    "Info" => config.loglevel = logging::Info,
                    "Warning" => config.loglevel = logging::Warning,
                    "Error" => config.loglevel = logging::Error,
                    _ => {}
                },
                _ => {}
            };
            match ircd_table.get(&"logfile".to_string()) {
                Some(&toml::String(ref s)) => match from_str::<Path>(s.as_slice()) {
                    Some(p) => config.logfile = p,
                    None => {}
                },
                _ => {}
            };
            match ircd_table.get(&"workers".to_string()) {
                Some(&toml::Integer(i)) => config.thread_handler_count = i as uint,
                _ => {}
            };
        }
        _ => {
            return Err("Could not find [metallircd] section in config file.".to_string())
        }
    }

    // [modules]
    match toml_table.get(&"module".to_string()) {
        Some(&toml::Table(ref modules_table)) => {
            for (name, module) in modules_table.iter() {
                if let &toml::Table(ref mod_table) = module {
                    if mod_table.contains_key(&"path".to_string()) {
                        // only take valid modules
                        config.modules.insert(name.clone(), mod_table.clone());
                    }
                }
            }
        },
        Some(_) => {
            return Err("Module sections should be in the form [module.name] .".to_string())
        },
        None => {
            // No modules ? ok...
        }
    }

    Ok(config)
}