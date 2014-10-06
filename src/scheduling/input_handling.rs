//! Handles all the commands send by users.

#![experimental]

use super::replies::send_names;
use super::users_handling::{Zombify, ChangeNick, Nothing};
use super::users_handling::RecyclingAction;
use super::ServerData;

use users::UserData;
use util;

use irccp;
use irccp::{IRCMessage, from_ircmessage, command, numericreply, ToIRCMessage};

use uuid::Uuid;

/// Dispatches the message, returns false if the target didn't exist.
fn dispatch_msg(me: &UserData, my_id: &Uuid, to: String, msg: String, srv: &ServerData, notice: bool) -> bool {
    let message = if notice { command::NOTICE(to.clone(), msg) } else { command::PRIVMSG(to.clone(), msg) }
                    .to_ircmessage().with_prefix(me.get_fullname().as_slice()).ok().unwrap();

    if srv.channels.read().has_chan(&to) {
        srv.channels.read().send_to_chan(&*srv.users.read(), &to, message, Some(my_id.clone()));
        true
    } else {
        match srv.users.read().get_user_by_nickname(&to) {
            Some(u) => {
                u.push_message(message);
                true
            },
            None => false
        }
    }
}

pub fn handle_command(me: &UserData, my_id: Uuid, msg: IRCMessage, srv: &ServerData) -> RecyclingAction {
    match from_ircmessage::<command::Command>(&msg) {
        // == Commands ==
        // QUIT
        Ok(command::QUIT(msg)) => {
            let emptied = srv.channels.read().quit(
                &*srv.users.read(),
                &my_id,
                command::QUIT(msg).to_ircmessage()
                    .with_prefix(me.get_fullname().as_slice()).ok().unwrap()
            );
            if emptied.len() > 0 {
                let mut handle = srv.channels.write();
                for chan in emptied.iter() {
                    handle.destroy_if_empty(chan);
                }
            }
            Zombify
        },
        // NICK
        Ok(command::NICK(nick)) => {
            if util::check_label(nick.as_slice()) {
                if nick != me.nickname { ChangeNick(nick) } else { Nothing }
            } else {
                me.push_message(
                    numericreply::ERR_ERRONEUSNICKNAME.to_ircmessage()
                        .with_prefix(srv.settings.read().name.as_slice()).ok().unwrap()
                        .with_suffix(format!("{} : Erroneous nickname.", nick).as_slice()).ok().unwrap()
                );
                Nothing
            }
        },
        // PING
        Ok(command::PING(from, target)) => {
            match target {
                None => {
                    // just bounce back the content to client
                    me.push_message(command::PONG(from, None).to_ircmessage());
                },
                Some(to) => if to == srv.settings.read().name {
                    me.push_message(command::PONG(to, Some(from)).to_ircmessage());
                } else {
                    // TODO handle ping forward to other servers
                }
            }
            Nothing
        },
        // Messages
        Ok(command::PRIVMSG(target, msg)) => {
            if !dispatch_msg(me, &my_id, target.clone(), msg, srv, false) {
                me.push_message(
                    numericreply::ERR_NOSUCHNICK.to_ircmessage()
                        .with_prefix(srv.settings.read().name.as_slice()).ok().unwrap()
                        .with_suffix(format!("{} : No such nick/channel.", target).as_slice()).ok().unwrap()
                );
            }
            Nothing
        },
        Ok(command::NOTICE(target, msg)) => {
            if !dispatch_msg(me, &my_id, target.clone(), msg, srv, true) {
                me.push_message(
                    numericreply::ERR_NOSUCHNICK.to_ircmessage()
                        .with_prefix(srv.settings.read().name.as_slice()).ok().unwrap()
                        .with_suffix(format!("{} : No such nick/channel.", target).as_slice()).ok().unwrap()
                );
            }
            Nothing
        },
        // Channel interaction
        Ok(command::JOIN(chan, _)) => {
            // TODO handle chan with passwords
            if util::check_channame(chan.as_slice()) {
                let has_chan = srv.channels.read().has_chan(&chan);
                if has_chan {
                    srv.channels.read().join(my_id, &chan);
                } else {
                    srv.channels.write().join_create(my_id, chan.clone());
                }
                srv.channels.read().send_to_chan(
                    &*srv.users.read(),
                    &chan,
                    command::JOIN(chan.clone(), None).to_ircmessage()
                        .with_prefix(me.get_fullname().as_slice()).ok().unwrap(),
                    None
                );
                send_names(me, &chan, srv);
            } else {
                // invalid chan name
                me.push_message(
                    numericreply::ERR_BADCHANMASK.to_ircmessage()
                        .with_prefix(srv.settings.read().name.as_slice()).ok().unwrap()
                        .with_suffix(format!("{} : Bad Channel name.", chan).as_slice()).ok().unwrap()
                );
            }
            Nothing
        }
        Ok(command::PART(chan, msg)) => {
            srv.channels.read().send_to_chan(
                &*srv.users.read(),
                &chan,
                command::PART(chan.clone(), msg).to_ircmessage()
                    .with_prefix(me.get_fullname().as_slice()).ok().unwrap(),
                None
            );
            let becomes_empty = srv.channels.read().part(&my_id, &chan);
            if becomes_empty {
                srv.channels.write().destroy_if_empty(&chan);
            }
            Nothing
        }
        // == Errors ==
        Err(irccp::TooFewParameters) => {
            me.push_message(
                numericreply::ERR_NEEDMOREPARAMS.to_ircmessage()
                    .add_arg(msg.command.as_slice()).ok().unwrap()
                    .with_suffix("Not enough parameters.").ok().unwrap()
            );
            Nothing
        },
        Err(irccp::UnknownCommand) => {
            me.push_message(
                numericreply::ERR_UNKNOWNCOMMAND.to_ircmessage()
                    .add_arg(msg.command.as_slice()).ok().unwrap()
                    .with_suffix("Unknown command.").ok().unwrap()
            );
            Nothing
        }
        Err(irccp::OtherError(_)) => { /* nothing for now */ Nothing },
        // == TODO ==
        Ok(_) => {
            me.push_message(
                numericreply::ERR_UNKNOWNCOMMAND.to_ircmessage()
                    .add_arg(msg.command.as_slice()).ok().unwrap()
                    .with_suffix("Not yet implemented.").ok().unwrap()
            );
            Nothing
        }
    }
}
