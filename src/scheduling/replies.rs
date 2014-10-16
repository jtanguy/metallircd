//! Automation of big server replies.

#![experimental]

use super::ServerData;
use users::UserData;

use messages::{IRCMessage, numericreply};

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