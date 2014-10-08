//! Config file parser.

#![experimental]

use super::ServerConf;
use logging;


use std::io::fs::File;

use toml;

/// Loads a config file. Expected format with default values
/// ```
/// [ircd]
/// server_name = <needed>
/// address = <needed>
/// port = <needed>
///
/// [logging]
/// level = "Warning"
/// file = "./metallircd.log"
///
/// [threads]
/// workers = 2
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

	// [ircd]
	match toml_table.find(&"ircd".to_string()) {
		Some(&toml::Table(ref ircd_table)) => {
			config.name = match ircd_table.find(&"server_name".to_string()) {
				Some(&toml::String(ref s)) => s.clone(),
				_ => return Err(
					format!("Error parsing config file {} : missing or invalid ircd.server_name", file.display())
				)
			};
			config.address = match ircd_table.find(&"address".to_string()) {
				Some(&toml::String(ref s)) => s.clone(),
				_ => return Err(
					format!("Error parsing config file {} : missing or invalid ircd.address", file.display())
				)
			};
			config.port = match ircd_table.find(&"port".to_string()) {
				Some(&toml::Integer(i)) => i as u16,
				_ => return Err(
					format!("Error parsing config file {} : missing or invalid ircd.address", file.display())
				)
			};
		}
		_ => return Err(format!("Error parsing config file {} : missing section [ircd].", file.display()))
	}

	// [logging]
	match toml_table.find(&"logging".to_string()) {
		Some(&toml::Table(ref logging_table)) => {
			 match logging_table.find(&"level".to_string()) {
				Some(&toml::String(ref s)) => match s.as_slice() {
					"Debug" => config.loglevel = logging::Debug,
					"Info" => config.loglevel = logging::Info,
					"Warning" => config.loglevel = logging::Warning,
					"Error" => config.loglevel = logging::Error,
					_ => {}
				},
				_ => {}
			};
			match logging_table.find(&"file".to_string()) {
				Some(&toml::String(ref s)) => match from_str::<Path>(s.as_slice()) {
					Some(p) => config.logfile = p,
					None => {}
				},
				_ => {}
			};
		}
		_ => {}
	}

	// [threads]
	match toml_table.find(&"threads".to_string()) {
		Some(&toml::Table(ref threads_table)) => {
			match threads_table.find(&"workers".to_string()) {
				Some(&toml::Integer(i)) => config.thread_handler_count = i as uint,
				_ => {}
			};
		}
		_ => {}
	}

	Ok(config)
}