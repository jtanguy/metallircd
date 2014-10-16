//! Core channel interaction commands.

#![experimental]

use messages::{IRCMessage, numericreply};
use scheduling::ServerData;
use users::UserData;
use util;

use uuid::Uuid;

use super::{RecyclingAction, Nothing};
use super::{CommandHandler, send_needmoreparams};

pub struct CmdJoin;

impl CommandHandler for CmdJoin {
    fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "JOIN" { return (false, Nothing); }

        if cmd.args.len() >= 1 {
            // TODO handle chan with passwords
            let chan = cmd.args[0].clone();
            if util::check_channame(chan.as_slice()) {
                let has_chan = srv.channels.read().has_chan(&chan);
                if has_chan {
                    srv.channels.read().join(user_uuid.clone(), &chan);
                } else {
                    srv.channels.write().join_create(user_uuid.clone(), chan.clone());
                }
                srv.channels.read().send_to_chan(
                    &*srv.users.read(),
                    &chan,
                    IRCMessage {
                        prefix: Some(user.get_fullname()),
                        command: "JOIN".to_string(),
                        args: vec!(cmd.args[0].clone()),
                        suffix: None
                    },
                    None
                );
                send_names(user, &chan, srv);
            } else {
                // invalid chan name
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_BADCHANMASK.to_text(),
                        args: vec!(user.nickname.clone(), cmd.args[0].clone()),
                        suffix: Some("Bad Channel name.".to_string())
                    }
                );
            }
        } else {
            send_needmoreparams(user, "JOIN", srv);
        }

        (true, Nothing)
    }
}

pub struct CmdPart;

impl CommandHandler for CmdPart {
    fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "PART" { return (false, Nothing); }

        if cmd.args.len() >= 1 {
            let chan = cmd.args[0].clone();
            srv.channels.read().send_to_chan(
                &*srv.users.read(),
                &chan,
                IRCMessage {
                    prefix: Some(user.get_fullname()),
                    command: "PART".to_string(),
                    args: cmd.args.clone(),
                    suffix: cmd.suffix.clone()
                },
                None
            );
            let becomes_empty = srv.channels.read().part(user_uuid, &chan);
            if becomes_empty {
                srv.channels.write().destroy_if_empty(&chan);
            }
        } else {
            send_needmoreparams(user, "PART", srv);
        }

        (true, Nothing)
    }
}

/// Sends a RPL_NAMREPLY with the users of the given chan to me
#[experimental]
pub fn send_names(me: &UserData, chan: &String, srv: &ServerData) {
    let names = srv.channels.read().member_list(chan);
    let msg = IRCMessage {
        prefix: Some(srv.settings.read().name.clone()),
        command: numericreply::RPL_NAMEREPLY.to_text(),
        args: vec!(me.nickname.clone(), "=".to_string(), chan.clone()),
        suffix: None
    };
    let mut buffer = String::new();
    for &(id, mode) in names.iter() {
        let mut nextnick = String::new();
        match mode.prefix() {
            Some(c) => nextnick.push(c),
            None => {}
        }
        nextnick.push_str(srv.users.read().get_user_by_uuid(&id).unwrap().nickname.as_slice());
        if buffer.len() + nextnick.len() + 1 > 510 - msg.protocol_len() {
            me.push_message(
                {
                    let mut m = msg.clone();
                    m.suffix = Some(buffer);
                    m
                }
            );
            buffer = nextnick;
        } else {
            if buffer.len() == 0 {
                buffer = nextnick;
            } else {
                buffer.push(' ');
                buffer.push_str(nextnick.as_slice());
            }
        }
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
            args: vec!(me.nickname.clone(), chan.clone()),
            suffix: Some("End of NAMES list.".to_string())
        }
    );
}