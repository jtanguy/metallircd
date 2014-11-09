//! Modes handling.

use metallirc::channels::Membership;
use metallirc::messages::{IRCMessage, numericreply};
use metallirc::ServerData;
use metallirc::users::UserData;

use std::slice::Items;

use uuid::Uuid;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::{CommandHandler, UserModeHandler, ChannelModeHandler};
use metallirc::modules::send_needmoreparams;

pub struct CmdMode;

module!(CmdMode is CommandHandler, UserModeHandler, ChannelModeHandler)

impl CommandHandler for CmdMode {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "MODE" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(1,14) {
            if let Some(other) = srv.users.read().get_user_by_nickname(args[0].as_slice()) {
                // it is me !
                if args.len() == 1 {
                    // only checking modes, oper can read all
                    if user.id == other.id || user.modes.read().get('o'.to_ascii()) {
                        user.push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::RPL_UMODEIS.to_text(),
                                args: vec!(
                                    other.nickname.clone(),
                                    other.modes.read().to_modestring()
                                ),
                                suffix: None
                            }
                        );
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
                } else {
                    // Oh, you want to change something ? Ok.
                    update_user_mode(user, &*other, args, srv);
                }
            } else if let Some(chan) = srv.channels.read().chan_handle(args[0].as_slice()) {
                // avoid deadlock
                if let Some(membership) = user.membership(args[0].as_slice()) {
                    if args.len() == 1 {
                        // only checking modes
                        user.push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::RPL_CHANNELMODEIS.to_text(),
                                args: vec!(
                                    user.nickname.clone(),
                                    args[0].clone(),
                                    chan.read().modes.to_modestring()
                                ),
                                suffix: None
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
                        let messages = update_chan_mode(user, &*membership, &args, srv);
                        for m in messages.into_iter() {
                            srv.channels.read().send_to_chan(args[0].as_slice(), m, None);
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

fn update_user_mode(user: &UserData, other: &UserData, args: Vec<String>, srv: &ServerData) {
    let mut words = args.iter().skip(1);
    let handler = srv.modules_handler.read();
    while let Some(ref txt) = words.next() {
        let (mut chars, set) = if txt.as_slice().starts_with("+") {
            (txt.as_slice().chars().skip(1), true)
        } else {
            (if txt.as_slice().starts_with("-") {
                txt.as_slice().chars().skip(1)
            } else {
                txt.as_slice().chars().skip(0)
            },
            false)
        };
        let mut response = if set { "+" } else { "-" }.to_string();
        for c in chars {
            if let Some(asc) = c.to_ascii_opt() {
            if let Some(b) = handler.handle_usermode(user, other, asc, set, srv) {
                if b {
                    response.push(c);
                } else {
                    user.push_message(
                        IRCMessage {
                            prefix: Some(srv.settings.read().name.clone()),
                            command: numericreply::ERR_USERSDONTMATCH.to_text(),
                            args: vec!(user.nickname.clone()),
                            suffix: Some("Can't change modes for other users.".to_string())
                        }
                    );
                }
            } else {
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_UMODEUNKNOWNFLAG.to_text(),
                        args: vec!(user.nickname.clone()),
                        suffix: Some(format!("Unknown MODE {}.", c))
                    }
                );
            }}
        }
        if response.len() > 1 {
            user.push_message(
                IRCMessage {
                    prefix: Some(user.get_fullname()),
                    command: "MODE".to_string(),
                    args: vec!(response),
                    suffix: None
                }
            );
        }
    }
}

fn update_chan_mode(user: &UserData, membership: &Membership,
                    args: &Vec<String>,
                    srv: &ServerData) -> Vec<IRCMessage> {
    let mut messages = Vec::new();
    let mut words = args.iter();
    let handler = srv.modules_handler.read();
    while let Some(ref txt) = words.next() {
        let mut chars = txt.as_slice().chars();
        let set = match chars.next() {
            Some('+') => true,
            Some('-') => false,
            _ => continue
        };
        let mut response = if set { "+" } else { "-" }.to_string();
        // keep track of current word status
        let words_before = words.clone();
        for c in chars {
            if let Some(asc) = c.to_ascii_opt() {
            if let Some(b) = handler.handle_channelmode(membership, asc, set, &mut words, srv) {
                if b {
                    response.push(c);
                } else {
                    user.push_message(
                        IRCMessage {
                            prefix: Some(srv.settings.read().name.clone()),
                            command: numericreply::ERR_CHANOPRIVSNEEDED.to_text(),
                            args: vec!(user.nickname.clone(), args[0].clone()),
                            suffix: Some(format!("You're not channel operator."))
                        }
                    );
                }
            } else {
                user.push_message(
                    IRCMessage {
                        prefix: Some(srv.settings.read().name.clone()),
                        command: numericreply::ERR_UMODEUNKNOWNFLAG.to_text(),
                        args: vec!(user.nickname.clone()),
                        suffix: Some(format!("Unknown MODE {}.", c))
                    }
                );
            }}
        }
        if response.len() > 1 {
            let mut msg_args = vec!(args[0].clone(), response);
            for a in words_before.take(words.len() - words_before.len()) {
                msg_args.push(a.clone());
            }
            messages.push(
                IRCMessage {
                    prefix: Some(user.get_fullname()),
                    command: "MODE".to_string(),
                    args: msg_args,
                    suffix: None
                }
            );
        }
    }
    messages
}

impl UserModeHandler for CmdMode {
    /// If can handle given mode, do it and return `Some(true)` if the
    /// transformation was allowed, `Some(false)` otherwise.
    /// If the mode is uwknown, return `None`.
    fn handle_usermode_request(&self, asker: &UserData, target: &UserData,
                               flag: Ascii, set: bool,
                               _: &ServerData) -> Option<bool> {
        // forbid change if not on self for modes I handle
        if asker.id != target.id
        && "io".to_ascii().contains(&flag) {
            return Some(false);
        }

        if set == true {
            if "i".to_ascii().contains(&flag) {
                // only certain flags are setable
                target.modes.write().set(flag, true);
                Some(true)
            } else if "o".to_ascii().contains(&flag) {
                // others are forbidden
                Some(false)
            } else {
                // or uknown
                None
            }
        } else {
            if "io".to_ascii().contains(&flag) {
                // only certain flags are removable
                target.modes.write().set(flag, false);
                Some(true)
            } else if "".to_ascii().contains(&flag) {
                // others are forbidden
                Some(false)
            } else {
                // or uknown
                None
            }
        }
    }
}

impl ChannelModeHandler for CmdMode {
    /// If can handle given mode, do it and return `Some(true)` if the
    /// transformation was allowed, `Some(false)` otherwise.
    /// If the mode is uwknown, return `None`.
    fn handle_chanmode_request(&self, asker: &Membership,
                               flag: Ascii, set: bool,
                               args: &mut Items<String>,
                               srv: &ServerData) -> Option<bool> {
        let _me = asker.user.upgrade().unwrap();
        let me = _me.read();
        // forbid change if not on oper for modes I handle, except
        // network operatorswho can do as they please
        if !asker.modes.read().get('o'.to_ascii())
        && "snmtvo".to_ascii().contains(&flag)
        &&  !me.modes.read().get('o'.to_ascii()){
            return Some(false);
        }

        let chan = asker.channel.upgrade().unwrap();

        if "vo".to_ascii().contains(&flag) {
            // it's a membership
            if let Some(nick) = args.next() {

                if let Some(target) = srv.users.read().get_user_by_nickname(nick.as_slice()) {
                    if let Some(membership) = target.channels.read().get(&chan.read().name) {
                        membership.modes.write().set(flag, set);
                    } else {
                        me.push_message(
                            IRCMessage {
                                prefix: Some(srv.settings.read().name.clone()),
                                command: numericreply::ERR_USERNOTINCHANNEL.to_text(),
                                args: vec!(me.nickname.clone(), nick.clone(), chan.read().name.clone()),
                                suffix: Some("They aren't on that channel.".to_string())
                            }
                        );
                    }
                } else {
                    me.push_message(
                        IRCMessage {
                            prefix: Some(srv.settings.read().name.clone()),
                            command: numericreply::ERR_NOSUCHNICK.to_text(),
                            args: vec!(me.nickname.clone(), nick.clone()),
                            suffix: Some("No such nick/channel.".to_string())
                        }
                    );
                }
            }
            Some(true)
        } else if "snmt".to_ascii().contains(&flag) {
            chan.write().modes.set(flag, set);
            Some(true)
        } else {
            None
        }
    }
}
