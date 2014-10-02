//! Handles all the commands send by users.

#![experimental]

use super::RecyclingAction;
use super::usermanager::UserManager;
use super::user::UserData;

use settings::ServerSettings;
use util;

use irccp;
use irccp::{IRCMessage, from_ircmessage, command, numericreply, ToIRCMessage};

fn dispatch_msg(from: String, to: String, msg: String, manager: &UserManager, notice: bool) {
    match manager.get_user_by_nickname(&to) {
        Some(u) => {
            u.push_message(
                if notice { command::NOTICE(to, msg) } else { command::PRIVMSG(to, msg) }
                    .to_ircmessage().with_prefix(from.as_slice()).ok().unwrap()
            );
        },
        None => {}
    }
}

pub fn handle_command(me: &UserData, msg: IRCMessage, manager: &UserManager, serverconf: &ServerSettings) -> RecyclingAction {
    match from_ircmessage::<command::Command>(&msg) {
        // == Commands ==
        // QUIT
        Ok(command::QUIT(_)) => super::Zombify,
        // NICK
        Ok(command::NICK(nick)) => {
            if util::check_nick(nick.as_slice()) {
                if nick != me.nickname { super::ChangeNick(nick) } else { super::Nothing }
            } else {
                me.push_message(
                    numericreply::ERR_ERRONEUSNICKNAME.to_ircmessage()
                        .with_prefix(serverconf.name.as_slice()).ok().unwrap()
                        .add_arg(nick.as_slice()).ok().unwrap()
                        .with_suffix("Erroneous nickname.").ok().unwrap()
                );
                super::Nothing
            }
        },
        // PING
        Ok(command::PING(from, target)) => {
            match target {
                None => {
                    // just bounce back the content to client
                    me.push_message(command::PONG(from, None).to_ircmessage());
                },
                Some(to) => if to == serverconf.name {
                    me.push_message(command::PONG(to, Some(from)).to_ircmessage());
                } else {
                    // TODO handle ping forward to other servers
                }
            }
            super::Nothing
        },
        // Messages
        Ok(command::PRIVMSG(target, msg)) => {
            dispatch_msg(me.get_fullname(), target, msg, manager, false);
            super::Nothing
        },
        Ok(command::NOTICE(target, msg)) => {
            dispatch_msg(me.get_fullname(), target, msg, manager, true);
            super::Nothing
        },
        // == Errors ==
        Err(irccp::TooFewParameters) => {
            me.push_message(
                numericreply::ERR_NEEDMOREPARAMS.to_ircmessage()
                    .add_arg(msg.command.as_slice()).ok().unwrap()
                    .with_suffix("Not enough parameters.").ok().unwrap()
            );
            super::Nothing
        },
        Err(irccp::UnknownCommand) => {
            me.push_message(
                numericreply::ERR_UNKNOWNCOMMAND.to_ircmessage()
                    .add_arg(msg.command.as_slice()).ok().unwrap()
                    .with_suffix("Unknown command.").ok().unwrap()
            );
            super::Nothing
        }
        Err(irccp::OtherError(_)) => { /* nothing for now */ super::Nothing },
        // == TODO ==
        Ok(_) => {
            me.push_message(
                numericreply::ERR_UNKNOWNCOMMAND.to_ircmessage()
                    .add_arg(msg.command.as_slice()).ok().unwrap()
                    .with_suffix("Not yet implemented.").ok().unwrap()
            );
            super::Nothing
        }
    }
}
