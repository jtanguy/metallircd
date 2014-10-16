//! Handles all the commands send by users.

#![experimental]

use super::replies::send_names;
use super::users_handling::{Zombify, ChangeNick, Nothing};
use super::users_handling::RecyclingAction;
use super::ServerData;

use users::UserData;
use util;

use messages::{IRCMessage,numericreply};

use uuid::Uuid;

/// Dispatches the message, returns false if the target didn't exist.
fn dispatch_msg(me: &UserData, my_id: &Uuid, to: String, msg: String, srv: &ServerData, notice: bool) -> bool {
    let message = IRCMessage {
        prefix: Some(me.get_fullname()),
        command: if notice { "NOTICE" } else { "PRIVMSG" }.to_string(),
        args: vec!(),
        suffix: Some(msg)
    };

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
    match msg.command.as_slice() {
        // == Commands ==
        // QUIT
        "QUIT" => {
            let emptied = srv.channels.read().quit(
                &*srv.users.read(),
                &my_id,
                IRCMessage {
                    prefix: Some(me.get_fullname()),
                    command: "QUIT".to_string(),
                    args: vec!(),
                    suffix: {
                        let mut it = msg.args.iter();
                        match it.next() {
                            None => None,
                            Some(txt) => Some(it.fold(txt.clone(), |f, n| f + " " + n.as_slice()))
                        }
                    }
                },
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
        "NICK" => if msg.args.len() >= 1 {
            let nick = msg.args[0].clone();
            if util::check_label(nick.as_slice()) {
                if nick != me.nickname { ChangeNick(nick) } else { Nothing }
            } else {
                me.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_ERRONEUSNICKNAME.to_text(),
                        args: vec!(me.nickname.clone(), nick),
                        suffix: Some("Erroneous nickname.".to_string())
                    }
                );
                Nothing
            }
        } else {
            me.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NEEDMOREPARAMS.to_text(),
                    args: vec!(me.nickname.clone(), msg.command.clone()),
                    suffix: Some("Not enough parameters.".to_string())
                }
            );
            Nothing
        },
        // PING
        "PING" => if msg.args.len() >= 1 {
            // TODO : more precise understanding of expected behavior !!
            me.push_message(
                IRCMessage {
                    prefix: None,
                    command: "PONG".to_string(),
                    args: vec!(srv.settings.read().name.clone(), msg.args[0].clone()),
                    suffix: None
                }
            );
            Nothing
        } else {
            me.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NEEDMOREPARAMS.to_text(),
                    args: vec!(me.nickname.clone(), msg.command.clone()),
                    suffix: Some("Not enough parameters.".to_string())
                }
            );
            Nothing
        },
        // Messages
        "PRIVMSG" | "NOTICE" => if msg.args.len() >= 2 {
            if !dispatch_msg(me, &my_id, msg.args[1].clone(), {
                // merge all remaining arguments as the message
                let txt = msg.args[1].clone();
                msg.args.iter().skip(2).fold(txt, |f, n| f + " " + n.as_slice())
            }, srv, msg.command.as_slice() == "NOTICE") {
                me.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_NOSUCHNICK.to_text(),
                        args: vec!(me.nickname.clone(), msg.args[0].clone()),
                        suffix: Some("No such nick/channel.".to_string())
                    }
                );
            }
            Nothing
        } else if msg.args.len() >= 1 {
            me.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NORECIPIENT.to_text(),
                    args: vec!(me.nickname.clone(), msg.command.clone()),
                    suffix: Some("No text to send.".to_string())
                }
            );
            Nothing
        } else {
            me.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NORECIPIENT.to_text(),
                    args: vec!(me.nickname.clone(), msg.command.clone()),
                    suffix: Some(format!("No recipient given ({}).", msg.command))
                }
            );
            Nothing
        },
        // Channel interaction
        "JOIN" => if msg.args.len() >= 1 {
            // TODO handle chan with passwords
            let chan = msg.args[0].clone();
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
                    IRCMessage {
                        prefix: Some(me.get_fullname()),
                        command: "JOIN".to_string(),
                        args: vec!(msg.args[0].clone()),
                        suffix: None
                    },
                    None
                );
                send_names(me, &chan, srv);
            } else {
                // invalid chan name
                me.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_BADCHANMASK.to_text(),
                        args: vec!(me.nickname.clone(), msg.args[0].clone()),
                        suffix: Some("Bad Channel name.".to_string())
                    }
                );
            }
            Nothing
        } else {
            me.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NEEDMOREPARAMS.to_text(),
                    args: vec!(me.nickname.clone(), msg.command.clone()),
                    suffix: Some("Not enough parameters.".to_string())
                }
            );
            Nothing
        },
        //Ok(command::PART(chan, msg)) => {
        "PART" => if msg.args.len() >= 1 {
            let chan = msg.args[0].clone();
            srv.channels.read().send_to_chan(
                &*srv.users.read(),
                &chan,
                IRCMessage {
                    prefix: Some(me.get_fullname()),
                    command: "PART".to_string(),
                    args: vec!(msg.args[0].clone()),
                    suffix: {
                        let mut it = msg.args.iter().skip(1);
                        match it.next() {
                            None => None,
                            Some(txt) => Some(it.fold(txt.clone(), |f, n| f + " " + n.as_slice()))
                        }
                    }
                },
                None
            );
            let becomes_empty = srv.channels.read().part(&my_id, &chan);
            if becomes_empty {
                srv.channels.write().destroy_if_empty(&chan);
            }
            Nothing
        } else {
            me.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NEEDMOREPARAMS.to_text(),
                    args: vec!(me.nickname.clone(), msg.command.clone()),
                    suffix: Some("Not enough parameters.".to_string())
                }
            );
            Nothing
        },
        _ => {
            me.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_UNKNOWNCOMMAND.to_text(),
                    args: vec!(me.nickname.clone(), msg.command.clone()),
                    suffix: Some("Unknown command.".to_string())
                }
            );
            Nothing
        },
    }
}
