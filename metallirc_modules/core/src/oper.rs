//! Core Oper commands.

#![experimental]

use metallirc::logging::{Logger, Warning, Info};
use metallirc::messages::{IRCMessage, numericreply};
use metallirc::ServerData;
use metallirc::users::UserData;

use std::collections::TreeMap;

use toml;
use uuid::Uuid;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::{CommandHandler, send_needmoreparams};

pub struct CmdOper {
    opers: TreeMap<String, String>
}

impl CmdOper {
    pub fn init(conf: &toml::TomlTable, logger: &Logger) -> CmdOper {
        let mut opers = TreeMap::new();
        if let Some(&toml::Array(ref oper_list)) = conf.find(&"operators".to_string()) {
            for v in oper_list.iter() {
                if let &toml::Array(ref oper) = v {
                if oper.len() >= 2 {
                if let toml::String(ref login) = oper[0] {
                if let toml::String(ref pass) = oper[1] {
                    opers.insert(login.clone(), pass.clone());
                    continue;
                }}}}
                logger.log(Warning,
                    String::from_str("Bad syntax in operators list.\
                                      Each entry should be in the format [\"login\", \"password\"].")
                );
            }
        }
        logger.log(Info, format!("(mod_core) {} operators were loaded from config file.", opers.len()));
        CmdOper {
            opers: opers
        }
    }
}

module!(CmdOper is CommandHandler)

impl CommandHandler for CmdOper {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "OPER" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(2,0) {
            if self.opers.find(&args[0]).map(|s| s == &args[1]).unwrap_or(false) {
                // login successful
                user.modes.write().set('o'.to_ascii(), true);
                srv.logger.log(Info, format!("Operator {} logged in from user {}.", args[0], user.get_fullname()));
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::RPL_YOUREOPER.to_text(),
                        args: vec!(user.nickname.clone()),
                        suffix: Some("You are now an IRC operator.".to_string())
                    }
                );
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: "MODE".to_string(),
                        args: vec!(user.nickname.clone(), "+o".to_string()),
                        suffix: None
                    }
                );
            } else {
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_PASSWDMISMATCH.to_text(),
                        args: vec!(user.nickname.clone()),
                        suffix: Some("Password incorrect.".to_string())
                    }
                );
            }
        } else {
            send_needmoreparams(user, "OPER", srv);
        }
        (true, Nothing)
    }
}

pub struct CmdDie;

module!(CmdDie is CommandHandler)

impl CommandHandler for CmdDie {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "DIE" { return (false, Nothing); }

        if user.modes.read().get('o'.to_ascii()) {
            srv.logger.log(Info, format!("Server Shutdown was requested by {}.", user.get_fullname()));
            *srv.signal_shutdown.write() = true
        } else {
            user.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NOPRIVILIGES.to_text(),
                    args: vec!(user.nickname.clone()),
                    suffix: Some("Permission Denied: You're not an IRC operator.".to_string())
                }
            );
        }

        (true, Nothing)
    }
}