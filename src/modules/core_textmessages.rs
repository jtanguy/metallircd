//! The message dispatchers

#![experimental]

use messages::{IRCMessage, TextMessage, Channel, User, Everybody, numericreply};
use scheduling::ServerData;
use users::UserData;

use uuid::Uuid;

use super::{RecyclingAction, Nothing};
use super::{MessageSendingHandler, CommandHandler};

pub struct CmdPrivmsgOrNotice;

impl CommandHandler for CmdPrivmsgOrNotice {
    fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        let notice = match cmd.command.as_slice() {
            "NOTICE" => true,
            "PRIVMSG" => false,
            _ => return (false, Nothing)
        };

        if cmd.args.len() + cmd.suffix.is_some() as uint >= 2 {
            // merge all remaining arguments as the message
            let mut txt = String::new();
            if cmd.args.len() > 1 {
                txt = cmd.args.iter().skip(2).fold(cmd.args[1].clone(), |f, n| f + " " + n.as_slice());
            }
            if let Some(ref suff) = cmd.suffix {
                txt = if txt.len() > 0 { txt + " " } else { txt } + suff.as_slice();
            }

            if let Some(id) = srv.users.read().get_uuid_of_nickname(&cmd.args[0]) {
                srv.modules_handler.read().send_message(
                    TextMessage {
                        notice: notice,
                        source: User(user_uuid.clone(), user.nickname.clone()),
                        target: User(id, cmd.args[0].clone()),
                        text: txt
                    }, srv);
            } else if srv.channels.read().has_chan(&cmd.args[0]) {
                srv.modules_handler.read().send_message(
                    TextMessage {
                        notice: notice,
                        source: User(user_uuid.clone(), user.nickname.clone()),
                        target: Channel(cmd.args[0].clone()),
                        text: txt
                    }, srv);
            } else {
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_NOSUCHNICK.to_text(),
                        args: vec!(user.nickname.clone(), cmd.args[0].clone()),
                        suffix: Some("No such nick/channel.".to_string())
                    }
                );
            }
        } else if cmd.args.len() >= 1 {
            user.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::ERR_NORECIPIENT.to_text(),
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

impl MessageSendingHandler for ChannelDispatcher {
    fn handle_message_sending(&self, cmd: TextMessage, srv: &ServerData) -> Option<TextMessage> {
        match cmd.target {
            Channel(chan) => {
                srv.channels.read().send_to_chan(
                    &*srv.users.read(),
                    &chan,
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