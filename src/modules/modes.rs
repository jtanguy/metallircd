//! Modes handling.

use channels::Channel;
use messages::{IRCMessage, numericreply};
use modes;
use scheduling::ServerData;
use users::UserData;

use uuid::Uuid;

use super::{RecyclingAction, Nothing};
use super::{CommandHandler, send_needmoreparams};

pub struct CmdMode;

module!(CmdMode is CommandHandler)

impl CommandHandler for CmdMode {
    fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "MODE" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(1,14) {
            if let Some(ref other) = srv.users.read().get_uuid_of_nickname(args[0].as_slice()) {
                // It is an user !
                if user_uuid == other {
                    // it is me !
                    if args.len() == 1 {
                        // only checking modes
                        user.push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::RPL_UMODEIS.to_text(),
                                args: vec!(user.nickname.clone()),
                                suffix: Some(user.modes.read().to_modestring())
                            }
                        );
                    } else {
                        // Oh, you want to change somtheing ? Ok.
                        update_user_mode(user, args, srv);
                    }
                } else {
                    // Looking someone's else modes ? No way !
                    user.push_message(
                        IRCMessage {
                            prefix: Some(srv.settings.read().name.clone()),
                            command: numericreply::ERR_USERSDONTMATCH.to_text(),
                            args: vec!(user.nickname.clone()),
                            suffix: Some("Can't change modes for other users.".to_string())
                        }
                    );
                }
            } else if let Some(chan) = srv.channels.read().chan_handle(args[0].as_slice()) {
                // avoid deadlock
                let has_member = chan.read().has_member(user_uuid);
                if has_member {
                    if args.len() == 1 {
                        // only checking modes
                        user.push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::RPL_CHANNELMODEIS.to_text(),
                                args: vec!(user.nickname.clone(), args[0].clone()),
                                suffix: Some(chan.read().modes.to_modestring())
                            }
                        );
                        // send creation date as well
                        user.push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::RPL_CREATIONTIME.to_text(),
                                args: vec!(user.nickname.clone(), args[0].clone()),
                                suffix: Some(chan.read().creation_time.to_string())
                            }
                        );
                    } else {
                        // Trying to make modifications
                        // avoid deadlock
                        let is_oper = chan.read().has_mode(user_uuid, modes::MOp);
                        if is_oper || user.modes.read().contains(modes::UOperator) {
                            let messages = update_chan_mode(user, &mut *chan.write(), &args, srv);
                            for m in messages.into_iter() {
                                srv.channels.read().send_to_chan(&*srv.users.read(), args[0].as_slice(), m, None);
                            }
                        } else {
                            user.push_message(
                                IRCMessage {
                                    prefix: Some(srv.settings.read().name.clone()),
                                    command: numericreply::ERR_CHANOPRIVSNEEDED.to_text(),
                                    args: vec!(user.nickname.clone(), args[0].clone()),
                                    suffix: Some("You're not channel operator.".to_string())
                                }
                            );
                        }
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
                }
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
        } else {
            send_needmoreparams(user, "MODE", srv);
        }
        (true, Nothing)
    }
}

fn update_user_mode(user: &UserData, args: Vec<String>, srv: &ServerData) {
    let mut words = args.iter().skip(1);
    let mut modes_handler = user.modes.write();
    while let Some(ref txt) = words.next() {
        let (mut chars, remove) = if txt.as_slice().starts_with("-") {
            (txt.as_slice().chars().skip(1), true)
        } else {
            (if txt.as_slice().starts_with("+") {
                txt.as_slice().chars().skip(1)
            } else {
                txt.as_slice().chars().skip(0)
            },
            false)
        };
        for c in chars {
            if (remove && modes::umodes_not_self_deactivable.contains_char(c))
            || (!remove && modes::umodes_not_self_activable.contains_char(c)) {
                // ignore.
            } else if let Some(m) = modes::UserMode::from_char(c) {
                if remove {
                    modes_handler.remove(m);
                } else {
                    modes_handler.insert(m);
                }
                user.push_message(
                    IRCMessage {
                        prefix: Some(user.get_fullname()),
                        command: "MODE".to_string(),
                        args: vec!(
                            user.nickname.clone(),
                            format!("{}{}",
                                if remove { "-" } else { "+" },
                                c
                            )
                        ),
                        suffix: None
                    }
                );
            } else {
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_UMODEUNKNOWNFLAG.to_text(),
                        args: vec!(user.nickname.clone()),
                        suffix: Some(format!("Unknown MODE {}.", c))
                    }
                );
            }
        }
    }
}

fn update_chan_mode(user: &UserData,
                    chan: &mut Channel, args: &Vec<String>,
                    srv: &ServerData) -> Vec<IRCMessage> {
    let mut messages = Vec::new();
    let mut words = args.iter();
    while let Some(ref txt) = words.next() {
        let mut chars = txt.as_slice().chars();
        let remove = match chars.next() {
            Some('+') => false,
            Some('-') => true,
            _ => continue
        };
        for c in chars {
            if let Some(md) = modes::MembershipMode::from_char(c) {
            if let Some(nick) = words.next() {
                // It is a membership mode and we have a nick given
                // If no nick is given we ignore it
                if let Some(id) = srv.users.read().get_uuid_of_nickname(nick.as_slice()) {
                    let result = if remove {
                        chan.remove_mode_from(&id, md)
                    } else {
                        chan.add_mode_to(&id, md)
                    };
                    if result {
                        messages.push(
                            IRCMessage {
                                prefix: Some(user.get_fullname()),
                                command: "MODE".to_string(),
                                args: vec!(
                                    args[0].clone(),
                                    format!("{}{}", if remove { "-" } else { "+" }.to_string(), c),
                                    nick.clone()
                                    ),
                                suffix: None
                            }
                        );
                    } else {
                        user.push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::ERR_USERNOTINCHANNEL.to_text(),
                                args: vec!(user.nickname.clone(), nick.clone(), args[0].clone()),
                                suffix: Some("They aren't on that channel.".to_string())
                            }
                        );
                    }
                } else {
                    user.push_message(
                        IRCMessage {
                            prefix: Some(srv.settings.read().name.clone()),
                            command: numericreply::ERR_NOSUCHNICK.to_text(),
                            args: vec!(user.nickname.clone(), nick.clone()),
                            suffix: Some("No such nick/channel.".to_string())
                        }
                    );
                }
            }} else if let Some(md) = modes::ChanMode::from_char(c) {
                if remove { chan.modes.remove(md); } else { chan.modes.insert(md); }
                messages.push(
                    IRCMessage {
                        prefix: Some(user.get_fullname()),
                        command: "MODE".to_string(),
                        args: vec!(
                            args[0].clone(),
                            format!("{}{}", if remove { "-" } else { "+" }.to_string(), c),
                            ),
                        suffix: None
                    }
                );
            } else {
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_UNKNOWNMODE.to_text(),
                        args: vec!(user.nickname.clone(), c.to_string()),
                        suffix: Some(format!("is unknown mode char to me for {}", args[0]))
                    }
                );
            }
        }

    }
    messages
}
