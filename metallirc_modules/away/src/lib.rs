//! Away module

#![feature(if_let, phase)]

#[phase(plugin)] extern crate metallirc;
extern crate metallirc;
extern crate uuid;
extern crate toml;

use std::collections::HashMap;
use std::sync::RWLock;

use uuid::Uuid;

use metallirc::messages::{IRCMessage, TextMessage, User, numericreply};
use metallirc::ServerData;
use metallirc::users::UserData;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::{MessageSendingHandler, CommandHandler};

// Public init()
use metallirc::modules::Module;
use metallirc::logging::Logger;

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
                user.modes.write().set('a'.to_ascii(), true);
                user.push_numreply(
                    numericreply::RPL_NOWAWAY,
                    srv.settings.read().name.as_slice()
                );
            } else {
                // unmark away status
                self.messages.write().remove(user_uuid);
                user.modes.write().set('a'.to_ascii(), false);
                user.push_numreply(
                    numericreply::RPL_UNAWAY,
                    srv.settings.read().name.as_slice()
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
        if let User(ref sid, _) = msg.source { // to a user
        if let Some(txt) = self.messages.read().get(tid) { // and the target is away
            srv.users.read().get_user_by_uuid(sid).unwrap().push_numreply(
                numericreply::RPL_AWAY(tnick.as_slice(), txt.as_slice()),
                srv.settings.read().name.as_slice()
            );
        }}}}
        Some(msg)
    }
}

#[no_mangle]
pub fn init(_: &toml::TomlTable, _: &Logger) -> Vec<Box<Module + 'static + Send + Sync>> {
    init_modules!(
        ModAway::init()
    )
}