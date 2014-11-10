//! The message dispatchers

#![experimental]

use metallirc::messages::{IRCMessage, TextMessage, Channel, User, Everybody, numericreply};
use metallirc::ServerData;
use metallirc::users::UserData;

use uuid::Uuid;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::{MessageSendingHandler, CommandHandler};

pub struct CmdPrivmsgOrNotice;

module!(CmdPrivmsgOrNotice is CommandHandler)

impl CommandHandler for CmdPrivmsgOrNotice {
    fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        let notice = match cmd.command.as_slice() {
            "NOTICE" => true,
            "PRIVMSG" => false,
            _ => return (false, Nothing)
        };

        if let Some(args) = cmd.as_nparams(2, 0) {

            if let Some(id) = srv.users.read().get_uuid_of_nickname(args[0].as_slice()) {
                srv.modules_handler.read().send_message(
                    TextMessage {
                        notice: notice,
                        source: User(user_uuid.clone(), user.nickname.clone()),
                        target: User(id, args[0].clone()),
                        text: args[1].clone()
                    }, srv);
            } else if srv.channels.read().has_chan(args[0].as_slice()) {
                srv.modules_handler.read().send_message(
                    TextMessage {
                        notice: notice,
                        source: User(user_uuid.clone(), user.nickname.clone()),
                        target: Channel(args[0].clone()),
                        text: args[1].clone()
                    }, srv);
            } else {
                user.push_numreply(
                    numericreply::ERR_NOSUCHNICK(args[0].as_slice()),
                    srv.settings.read().name.as_slice()
                );
            }
        } else if cmd.args.len() >= 1 {
            user.push_numreply(
                numericreply::ERR_NOTEXTTOSEND,
                srv.settings.read().name.as_slice()
            );
        } else {
            user.push_numreply(
                numericreply::ERR_NORECIPIENT(cmd.command.as_slice()),
                srv.settings.read().name.as_slice()
            );
        }
        (true, Nothing)
    }
}

pub struct QueryDispatcher;

module!(QueryDispatcher is MessageSendingHandler)

impl MessageSendingHandler for QueryDispatcher {
    fn handle_message_sending(&self, cmd: TextMessage, srv: &ServerData) -> Option<TextMessage> {
        match cmd.target {
            Everybody => {
                srv.users.read().apply_to_all(|u| {
                    u.push_message(
                        IRCMessage {
                            prefix: Some(cmd.source.clone().into_text()),
                            command: if cmd.notice { "NOTICE" } else { "PRIVMSG" }.to_string(),
                            args: vec!(u.nickname.clone()),
                            suffix: Some(cmd.text.clone())
                        }
                    )
                });
                None
            },
            User(id, _) => {
                srv.users.read().get_user_by_uuid(&id).unwrap().push_message(
                    IRCMessage {
                        prefix: Some(cmd.source.into_text()),
                        command: if cmd.notice { "NOTICE" } else { "PRIVMSG" }.to_string(),
                        args: vec!(cmd.target.into_text()),
                        suffix: Some(cmd.text)
                    }
                );
                None
            },
            _ => Some(cmd)
        }
    }
}

pub struct ChannelDispatcher;

module!(ChannelDispatcher is MessageSendingHandler)

impl MessageSendingHandler for ChannelDispatcher {
    fn handle_message_sending(&self, cmd: TextMessage, srv: &ServerData) -> Option<TextMessage> {
        match cmd.target {
            Channel(chan) => {
                // Check external messages
                if let User(ref id, _) = cmd.source {
                    let umanager_handle = srv.users.read();
                    let user = umanager_handle.get_user_by_uuid(id).unwrap();
                    // check external messages
                    if srv.channels.read().chan_handle(chan.as_slice())
                        .map(|c| c.read().modes.get('n'.to_ascii())).unwrap_or(false) {
                        // There is some checking to do
                        if user.membership(chan.as_slice()).is_none() {
                            user.push_numreply(
                                numericreply::ERR_CANNOTSENDTOCHAN(chan.as_slice()),
                                srv.settings.read().name.as_slice()
                            );
                            return None;
                        }
                    }
                    // check moderated chan
                    if srv.channels.read().chan_handle(chan.as_slice())
                        .map(|c| c.read().modes.get('m'.to_ascii())).unwrap_or(false) {
                        // There is some checking to do
                        if !user.membership(chan.as_slice())
                            .map(|m| !m.modes.read().none()).unwrap_or(false) {
                            user.push_numreply(
                                numericreply::ERR_CANNOTSENDTOCHAN(chan.as_slice()),
                                srv.settings.read().name.as_slice()
                            );
                            return None;
                        }
                    }
                }
                // sending is valid
                srv.channels.read().send_to_chan(
                    chan.as_slice(),
                    IRCMessage {
                        prefix: Some(cmd.source.clone().into_text()),
                        command: if cmd.notice { "NOTICE" } else { "PRIVMSG" }.to_string(),
                        args: vec!(chan.clone()),
                        suffix: Some(cmd.text)
                    },
                    if let User(id, _) = cmd.source { Some(id) } else { None }
                );
                None
            }
            _ => Some(cmd)
        }
    }
}