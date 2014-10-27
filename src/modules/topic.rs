use messages::{IRCMessage, numericreply};
use modes;
use scheduling::ServerData;
use users::UserData;

use uuid::Uuid;

use super::{RecyclingAction, Nothing};
use super::{CommandHandler, send_needmoreparams};

pub struct CmdTopic;

module!(CmdTopic is CommandHandler)

impl CommandHandler for CmdTopic {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "TOPIC" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(1, 1) {
            // first, find the chan
            let channels_handle = srv.channels.read();
            let chan_handle = match channels_handle.chan_handle(args[0].as_slice()) {
                Some(h) => h,
                None => {
                    user.push_message(
                        IRCMessage {
                            prefix: Some(srv.settings.read().name.clone()),
                            command: numericreply::ERR_NOSUCHNICK.to_text(),
                            args: vec!(user.nickname.clone(), args[0].clone()),
                            suffix: Some("No such nick/channel.".to_string())
                        }
                    );
                    return (true, Nothing);
                }
            };

            if args.len() == 1 {
                // we are only reading the topic
                send_topic_to_user(user, chan_handle.read().topic.as_slice(),
                                   args[0].as_slice(), srv);
            } else {
                // it's an attempt to modify it.
                let can_modify = if let Some(m) = user.membership(args[0].as_slice()) {
                    if !m.channel.upgrade().unwrap().read().modes.contains(modes::CLockTopic)
                    || m.modes.read().is_at_least(&modes::MOp) {
                        true
                    } else {
                        user.push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::ERR_CHANOPRIVSNEEDED.to_text(),
                                args: vec!(user.nickname.clone(), args[0].clone()),
                                suffix: Some("You're not channel operator&.".to_string())
                            }
                        );
                        false
                    }
                } else {
                    user.push_message(
                        IRCMessage {
                            prefix: Some(srv.settings.read().name.clone()),
                            command: numericreply::ERR_NOTONCHANNEL.to_text(),
                            args: vec!(user.nickname.clone(), args[0].clone()),
                            suffix: Some("You're not on that channel.".to_string())
                        }
                    );
                    false
                };


                if can_modify {
                    chan_handle.write().topic = args[1].clone();
                    channels_handle.send_to_chan(
                        args[0].as_slice(),
                        IRCMessage {
                            prefix: Some(user.get_fullname()),
                            command: "TOPIC".to_string(),
                            args: vec!(args[0].clone()),
                            suffix: Some(args[1].clone())
                        },
                        None
                    );
                }
            }

        } else {
            send_needmoreparams(user, "TOPIC", srv);
        }
        (true, Nothing)
    }
}

pub fn send_topic_to_user(user: &UserData, topic: &str, channame: &str, srv: &ServerData) {
    if topic.len() == 0 {
        user.push_message(
            IRCMessage {
                prefix: Some(srv.settings.read().name.clone()),
                command: numericreply::RPL_NOTOPIC.to_text(),
                args: vec!(user.nickname.clone(), channame.to_string()),
                suffix: Some("No topic is set.".to_string())
            }
        );
    } else {
        user.push_message(
            IRCMessage {
                prefix: Some(srv.settings.read().name.clone()),
                command: numericreply::RPL_TOPIC.to_text(),
                args: vec!(user.nickname.clone(), channame.to_string()),
                suffix: Some(topic.to_string())
            }
        );
    }
}