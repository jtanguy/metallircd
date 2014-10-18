//! Core commands handling.

#![experimental]

use messages::{IRCMessage, numericreply};
use scheduling::ServerData;
use users::UserData;
use util;

use uuid::Uuid;

use super::{RecyclingAction, ChangeNick, Nothing, Zombify};
use super::{CommandHandler, send_needmoreparams};

pub struct CmdNick;

impl CommandHandler for CmdNick {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "NICK" { return (false, Nothing); }

        if cmd.args.len() >= 1 {
            let nick = cmd.args[0].clone();
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

impl CommandHandler for CmdQuit {
    fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "QUIT" { return (false, Nothing); }

        let emptied = srv.channels.read().quit(
            &*srv.users.read(),
            user_uuid,
            IRCMessage {
                prefix: Some(user.get_fullname()),
                command: "QUIT".to_string(),
                args: cmd.args.clone(),
                suffix: cmd.suffix.clone()
            }
        );
        if emptied.len() > 0 {
            let mut handle = srv.channels.write();
            for chan in emptied.iter() {
                handle.destroy_if_empty(chan.as_slice());
            }
        }
        (true, Zombify)
    }
}

pub struct CmdPing;

impl CommandHandler for CmdPing {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "PING" { return (false, Nothing); }

        if cmd.args.len() >= 1 {
            // TODO : more precise understanding of expected behavior !!
            user.push_message(
                IRCMessage {
                    prefix: None,
                    command: "PONG".to_string(),
                    args: vec!(srv.settings.read().name.clone(), cmd.args[0].clone()),
                    suffix: None
                }
            );
        } else {
            send_needmoreparams(user, "PING", srv);
        }
        (true, Nothing)
    }
}