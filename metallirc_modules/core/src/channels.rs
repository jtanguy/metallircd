//! Core channel interaction commands.

#![experimental]

use std::collections::hashmap::{Vacant, Occupied};

use metallirc::messages::{IRCMessage, numericreply};
use metallirc::ServerData;
use metallirc::users::UserData;
use metallirc::util;

use uuid::Uuid;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::{CommandHandler, send_needmoreparams};

pub struct CmdJoin;

module!(CmdJoin is CommandHandler)

impl CommandHandler for CmdJoin {
    fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "JOIN" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(1,1) {
            // TODO handle chan with passwords
            for chan in args[0].as_slice().split_terminator(',') {
                if util::check_channame(chan) {
                    let has_chan = srv.channels.read().has_chan(chan);
                    if has_chan {
                        srv.channels.read().join(srv.users.read().arc_ref(user_uuid).unwrap(), chan);
                    } else {
                        srv.channels.write().join_create(srv.users.read().arc_ref(user_uuid).unwrap(), chan);
                    }
                    srv.channels.read().send_to_chan(
                        chan,
                        IRCMessage {
                            prefix: Some(user.get_fullname()),
                            command: "JOIN".to_string(),
                            args: vec!(chan.to_string()),
                            suffix: None
                        },
                        None
                    );
                    send_names(user, chan, srv);
                    super::topic::send_topic_to_user(
                        user,
                        srv.channels.read().chan_handle(chan).unwrap().read().topic.as_slice(),
                        chan,
                        srv
                    );
                } else {
                    // invalid chan name
                    user.push_message(
                        IRCMessage {
                            prefix: Some(srv.settings.read().name.clone()),
                            command: numericreply::ERR_BADCHANMASK.to_text(),
                            args: vec!(user.nickname.clone(), chan.to_string()),
                            suffix: Some("Bad Channel name.".to_string())
                        }
                    );
                }
            }
        } else {
            send_needmoreparams(user, "JOIN", srv);
        }

        (true, Nothing)
    }
}

pub struct CmdPart;

module!(CmdPart is CommandHandler)

impl CommandHandler for CmdPart {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "PART" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(1,1) {
            let partmsg = if args.len() > 1 { args[1].as_slice() } else { "Leaving." };
            for chan in args[0].as_slice().split_terminator(',') {
                match user.channels.write().entry(chan.to_string()) {
                    Occupied(e) => {
                        e.take().channel.upgrade().unwrap().read().apply_to_members(|_, m| {
                            m.user.upgrade().unwrap().read().push_message(
                                IRCMessage {
                                    prefix: Some(user.get_fullname()),
                                    command: "PART".to_string(),
                                    args: cmd.args.clone(),
                                    suffix: Some(partmsg.to_string())
                                }
                            );
                        });
                    },
                    Vacant(_) => {
                        user.push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::ERR_NOTONCHANNEL.to_text(),
                                args: vec!(user.nickname.clone(), chan.to_string()),
                                suffix: Some("You're not on that channel.".to_string())
                            }
                        );
                    }
                }
                let empty = srv.channels.read().chan_handle(chan).unwrap().write().cleanup();
                if empty { srv.channels.write().destroy_if_empty(chan); }
            }
        } else {
            send_needmoreparams(user, "PART", srv);
        }

        (true, Nothing)
    }
}

pub struct CmdNames;

module!(CmdNames is CommandHandler)

impl CommandHandler for CmdNames {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "NAMES" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(1,1) {
            for chan in args[0].as_slice().split_terminator(',') {
                if srv.channels.read().has_chan(chan) {
                    send_names(user, chan, srv);
                } else {
                    user.push_message(
                        IRCMessage {
                            prefix: Some(srv.settings.read().name.clone()),
                            command: numericreply::ERR_NOSUCHNICK.to_text(),
                            args: vec!(user.nickname.clone(), chan.to_string()),
                            suffix: Some("No such nick/channel.".to_string())
                        }
                    );
                }
            }
        } else {
            send_needmoreparams(user, "NAMES", srv);
        }
        (true, Nothing)
    }
}

/// Sends a RPL_NAMREPLY with the users of the given chan to me
/// Assumes the chan exists.
#[experimental]
pub fn send_names(me: &UserData, chan: &str, srv: &ServerData) {
    let msg = IRCMessage {
        prefix: Some(srv.settings.read().name.clone()),
        command: numericreply::RPL_NAMEREPLY.to_text(),
        args: vec!(me.nickname.clone(), "=".to_string(), chan.to_string()),
        suffix: None
    };
    let mut buffer = String::new();
    if let Some(handle) = srv.channels.read().chan_handle(chan) {
        handle.read().apply_to_members(|_, m| {
            let mut nextnick = String::new();
            match m.modes.read().prefix() {
                Some(c) => nextnick.push(c),
                None => {}
            }
            nextnick.push_str(m.user.upgrade().unwrap().read().nickname.as_slice());
            if buffer.len() + nextnick.len() + 1 > 510 - msg.protocol_len() {
                me.push_message(
                    {
                        let mut m = msg.clone();
                        m.suffix = Some(::std::mem::replace(&mut buffer, nextnick));
                        m
                    }
                );
            } else {
                if buffer.len() == 0 {
                    buffer = nextnick;
                } else {
                    buffer.push(' ');
                    buffer.push_str(nextnick.as_slice());
                }
            }
        });
    }
    if buffer.len() > 0 {
        me.push_message(
            {
                let mut m = msg.clone();
                m.suffix = Some(buffer);
                m
            }
        );
    }
    me.push_message(
        IRCMessage {
            prefix: Some(srv.settings.read().name.clone()),
            command: numericreply::RPL_ENDOFNAMES.to_text(),
            args: vec!(me.nickname.clone(), chan.to_string()),
            suffix: Some("End of NAMES list.".to_string())
        }
    );
}