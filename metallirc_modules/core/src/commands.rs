//! Core commands handling.

#![experimental]

use metallirc::messages::{IRCMessage, numericreply};
use metallirc::ServerData;
use metallirc::users::UserData;
use metallirc::util;

use uuid::Uuid;

use metallirc::modules::{RecyclingAction, ChangeNick, Nothing, Zombify};
use metallirc::modules::{CommandHandler, send_needmoreparams};

pub struct CmdNick;

module!(CmdNick is CommandHandler)

impl CommandHandler for CmdNick {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "NICK" { return (false, Nothing); }

        if let Some(mut args) = cmd.as_nparams(1,0) {
            let nick = args.pop().unwrap();
            if util::check_label(nick.as_slice()) {
                if nick != user.nickname { return (true, ChangeNick(nick)) }
            } else {
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_ERRONEUSNICKNAME.to_text(),
                        args: vec!(user.nickname.clone(), nick),
                        suffix: Some("Erroneous nickname.".to_string())
                    }
                );
            }
        } else {
            send_needmoreparams(user, "NICK", srv);
        }
        (true, Nothing)
    }
}

pub struct CmdQuit;

module!(CmdQuit is CommandHandler)

impl CommandHandler for CmdQuit {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, _: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "QUIT" { return (false, Nothing); }

        user.send_to_known(
            IRCMessage {
                prefix: Some(user.get_fullname()),
                command: "QUIT".to_string(),
                args: cmd.args.clone(),
                suffix: cmd.suffix.clone()
            }
        );
        (true, Zombify)
    }
}

pub struct CmdPing;

module!(CmdPing is CommandHandler)

impl CommandHandler for CmdPing {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "PING" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(1,1) {
            // TODO : more precise understanding of expected behavior !!
            user.push_message(
                IRCMessage {
                    prefix: None,
                    command: "PONG".to_string(),
                    args: vec!(srv.settings.read().name.clone(), args[0].clone()),
                    suffix: None
                }
            );
        } else {
            send_needmoreparams(user, "PING", srv);
        }
        (true, Nothing)
    }
}