//! The message dispatchers

#![experimental]

use messages::{IRCMessage, TextMessage, Channel, User, Everybody, numericreply};
use modes;
use scheduling::ServerData;
use users::UserData;

use uuid::Uuid;

use super::{RecyclingAction, Nothing};
use super::{MessageSendingHandler, CommandHandler};

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
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_NOSUCHNICK.to_text(),
                        args: vec!(user.nickname.clone(), args[0].clone()),
                        suffix: Some("No such nick/channel.".to_string())
                    }
                );
            }
        } else if cmd.args.len() >= 1 {
            user.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NOTEXTTOSEND.to_text(),
                    args: vec!(user.nickname.clone(), cmd.command.clone()),
                    suffix: Some("No text to send.".to_string())
                }
            );
        } else {
            user.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NORECIPIENT.to_text(),
                    args: vec!(user.nickname.clone(), cmd.command.clone()),
                    suffix: Some(format!("No recipient given ({}).", cmd.command))
                }
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
                srv.users.read().iterate_map(|u| {
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
                if srv.channels.read().chan_handle(chan.as_slice())
                    .map(|c| c.read().modes.contains(modes::CNoExternalMsg)).unwrap_or(false) {
                    // There is some checking to do
                    if let User(ref id, ref nickname) = cmd.source {
                    if !srv.channels.read().is_in_chan(id, chan.as_slice()) {
                        srv.users.read().get_user_by_uuid(id).unwrap().push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::ERR_CANNOTSENDTOCHAN.to_text(),
                                args: vec!(nickname.clone(), chan.clone()),
                                suffix: Some("Cannot send to channel.".to_string())
                            }
                        );
                        return None;
                    }}
                }
                // check moderated chan
                if srv.channels.read().chan_handle(chan.as_slice())
                    .map(|c| c.read().modes.contains(modes::CModerated)).unwrap_or(false) {
                    // There is some checking to do
                    if let User(ref id, ref nickname) = cmd.source {
                    if !srv.channels.read().chan_handle(chan.as_slice())
                        .map(|c| c.read().is_at_least(id, modes::MVoice)).unwrap_or(false) {
                        srv.users.read().get_user_by_uuid(id).unwrap().push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::ERR_CANNOTSENDTOCHAN.to_text(),
                                args: vec!(nickname.clone(), chan.clone()),
                                suffix: Some("Cannot send to channel.".to_string())
                            }
                        );
                        return None;
                    }}
                }
                // sending is valid
                srv.channels.read().send_to_chan(
                    &*srv.users.read(),
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