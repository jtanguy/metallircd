//! Away module

use std::collections::HashMap;
use std::sync::RWLock;

use messages::{IRCMessage, TextMessage, User, numericreply};
use scheduling::ServerData;
use users::UserData;

use uuid::Uuid;

use super::{RecyclingAction, Nothing};
use super::{MessageSendingHandler, CommandHandler};

pub struct ModAway {
    messages: RWLock<HashMap<Uuid, String>>
}

impl ModAway {
    pub fn init() -> ModAway {
        ModAway {
            messages: RWLock::new(HashMap::new())
        }
    }
}

module!(ModAway is CommandHandler, MessageSendingHandler)

impl CommandHandler for ModAway {
        fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "AWAY" { return (false, Nothing); }

        if let Some(mut args) = cmd.as_nparams(0,1) {
            if let Some(msg) = args.pop() {
                // new away message
                self.messages.write().insert(user_uuid.clone(), msg);
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::RPL_NOWAWAY.to_text(),
                        args: vec!(user.nickname.clone()),
                        suffix: Some("You have been marked as away.".to_string())
                    }
                );
            } else {
                // unmark away status
                self.messages.write().remove(user_uuid);
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::RPL_UNAWAY.to_text(),
                        args: vec!(user.nickname.clone()),
                        suffix: Some("You are no longer marked as away.".to_string())
                    }
                );
            }
        }
        (true, Nothing)
    }
}

impl MessageSendingHandler for ModAway {
    fn handle_message_sending(&self, msg: TextMessage, srv: &ServerData) -> Option<TextMessage> {
        if !msg.notice { // it's a PRIVMSG
        if let User(ref tid, ref tnick) = msg.target { // from a user
        if let User(ref sid, ref snick) = msg.source { // to a user
        if let Some(txt) = self.messages.read().find(tid) { // and the target is away
            srv.users.read().get_user_by_uuid(sid).unwrap().push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::RPL_AWAY.to_text(),
                    args: vec!(snick.clone(), tnick.clone()),
                    suffix: Some(txt.clone())
                }
            );
        }}}}
        Some(msg)
    }
}