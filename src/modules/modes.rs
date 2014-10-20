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
                if chan.read().has_member(user_uuid) {
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
                    } else {
                        // Trying to make modifications
                        update_chan_mode(&mut *chan.write(), args, srv);
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

fn update_chan_mode(chan: &mut Channel, args: Vec<String>, srv: &ServerData) {
    // TODO
}
