//! Core channel interaction commands.

#![experimental]

use std::collections::hash_map::{Vacant, Occupied};

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
                    user.push_numreply(
                        numericreply::ERR_BADCHANMASK(chan),
                        srv.settings.read().name.as_slice()
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
                        user.push_numreply(
                            numericreply::ERR_NOTONCHANNEL(chan),
                            srv.settings.read().name.as_slice()
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
                if let Some(handle) = srv.channels.read().chan_handle(chan) {
                    // hide secret channels
                    if (!handle.read().modes.get('s'.to_ascii()))
                    || user.membership(chan).is_some() {
                        send_names(user, chan, srv);
                        // don't send error message
                        continue;
                    }
                }
                user.push_numreply(
                    numericreply::ERR_NOSUCHNICK(chan),
                    srv.settings.read().name.as_slice()
                );
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
    let mut buffer = Vec::new();
    if let Some(handle) = srv.channels.read().chan_handle(chan) {
        handle.read().apply_to_members(|_, m| {
            buffer.push((
                if m.modes.read().get('o'.to_ascii()) {
                    Some('@')
                } else if m.modes.read().get('v'.to_ascii()) {
                    Some('+')
                } else {
                    None
                },
                m.user.upgrade().unwrap().read().nickname.as_slice().into_string()
            ));
            if buffer.len() == 10 {
                let oldbuff = ::std::mem::replace(&mut buffer, Vec::new());
                me.push_numreply(
                    numericreply::RPL_NAMEREPLY('=', chan, oldbuff),
                    srv.settings.read().name.as_slice()
                );
            }
        });
    }
    if buffer.len() > 0 {
        me.push_numreply(
            numericreply::RPL_NAMEREPLY('=', chan, buffer),
            srv.settings.read().name.as_slice()
        );
    }
    me.push_numreply(
        numericreply::RPL_ENDOFNAMES(chan),
        srv.settings.read().name.as_slice()
    );
}