use metallirc::channels::Membership;
use metallirc::messages::{IRCMessage, numericreply};
use metallirc::modes::letter_for_membership;
use metallirc::ServerData;
use metallirc::users::UserData;
use metallirc::util::matches_mask;

use std::sync::Arc;

use uuid::Uuid;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::{CommandHandler, send_needmoreparams};

/*
 * WHO
 */

pub struct CmdWho;

module!(CmdWho is CommandHandler)

impl CommandHandler for CmdWho {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "WHO" { return (false, Nothing); }

        if let Some(mut args) = cmd.as_nparams(0, 2) { // always true
            let (mask, only_oper) = match (args.pop(), args.pop()) {
                (None, _) => ("*".into_string(), false),
                (Some(m), None) => (m, false),
                (Some(o), Some(m)) => (m, o.as_slice() == "o")
            };
            if let Some(chandle) = srv.channels.read().chan_handle(mask.as_slice()) {
                let me_in_chan = user.channels.read().contains_key(&mask);
                chandle.read().apply_to_members(|_, membership| {
                    if let Some(other_lock) = membership.user.upgrade() {
                        let other = other_lock.read();
                        if (me_in_chan || first_common_chan(user, &*other).is_some())
                        && (!only_oper || other.modes.read().get('o'.to_ascii())) {
                            user.push_numreply(
                                numericreply::RPL_WHOREPLY(
                                    mask.as_slice(),
                                    other.username.as_slice(),
                                    other.hostname.as_slice(),
                                    srv.settings.read().name.as_slice(),
                                    other.nickname.as_slice(),
                                    if other.modes.read().get('a'.to_ascii()) { 'G' } else { 'H' },
                                    false,
                                    letter_for_membership(&*membership.modes.read()),
                                    0,
                                    other.realname.as_slice()
                                ),
                                srv.settings.read().name.as_slice()
                            );
                        }
                    }
                });
            } else {
                srv.users.read().apply_to_all(|other| {
                    if matches_mask(other.nickname.as_slice(), mask.as_slice())
                    || matches_mask(other.username.as_slice(), mask.as_slice())
                    || matches_mask(other.hostname.as_slice(), mask.as_slice())
                    || matches_mask(other.realname.as_slice(), mask.as_slice()) {
                        let (chan, membership_tag) = {
                            if let Some((c, m)) = first_common_chan(user, other) {
                                (c, letter_for_membership(&*m.modes.read()))
                            } else {
                                ('*'.to_string(), None)
                            }
                        };
                        if (!other.modes.read().get('i'.to_ascii()) || chan.as_slice() != "*")
                        && (!only_oper || other.modes.read().get('o'.to_ascii())) {
                            user.push_numreply(
                                numericreply::RPL_WHOREPLY(
                                    chan.as_slice(),
                                    other.username.as_slice(),
                                    other.hostname.as_slice(),
                                    srv.settings.read().name.as_slice(),
                                    other.nickname.as_slice(),
                                    if other.modes.read().get('a'.to_ascii()) { 'G' } else { 'H' },
                                    false,
                                    membership_tag,
                                    0,
                                    other.realname.as_slice()
                                ),
                                srv.settings.read().name.as_slice()
                            );
                        }
                    }
                });
            }
            user.push_numreply(
                numericreply::RPL_ENDOFWHO(mask.as_slice()),
                srv.settings.read().name.as_slice()
            );
        }

        (true, Nothing)
    }
}

fn first_common_chan(a: &UserData, b: &UserData) -> Option<(String, Arc<Membership>)> {
    let a_chans = a.channels.read();
    let b_chans = b.channels.read();
    for chan in a_chans.keys() {
        if let Some(m) = b_chans.get(chan) {
            return Some((chan.clone(), m.clone()));
        }
    }
    None
}

/*
 * WHOIS
 */

pub struct CmdWhois;

module!(CmdWhois is CommandHandler)

impl CommandHandler for CmdWhois {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "WHOIS" { return (false, Nothing); }

        if let Some(mut args) = cmd.as_nparams(1,1) {
            let masks = args.pop().unwrap();
            for mask in masks.as_slice().split_terminator(',') {
                if mask.contains_char('?') || mask.contains_char('*') {
                    srv.users.read().apply_to_all(|u| {
                        if matches_mask(u.nickname.as_slice(), mask) {
                            if !u.modes.read().get('i'.to_ascii()) {
                                // Invisible users can only be WHOISed with exact match
                                send_whois(user, u, srv.settings.read().name.as_slice());
                            }
                        }
                    })
                } else if let Some(other) = srv.users.read().get_user_by_nickname(mask) {
                    send_whois(user, &*other, srv.settings.read().name.as_slice());
                } else {
                    user.push_numreply(
                        numericreply::ERR_NOSUCHNICK(mask),
                        srv.settings.read().name.as_slice()
                    );
                }
            }
            user.push_numreply(
                numericreply::RPL_ENDOFWHOIS(masks.as_slice()),
                srv.settings.read().name.as_slice()
            );
        } else {
            send_needmoreparams(user, "WHOIS", srv);
        }

        (true, Nothing)
    }
}

fn send_whois(me: &UserData, other: &UserData, srv_name: &str) {
    me.push_numreply(
        numericreply::RPL_WHOISUSER(
            other.nickname.as_slice(),
            other.username.as_slice(),
            other.hostname.as_slice(),
            other.realname.as_slice()
        ),
        srv_name
    );
    me.push_numreply(
        numericreply::RPL_WHOISSERVER(other.nickname.as_slice(), srv_name, ""),
        srv_name
    );
    me.push_numreply(
        numericreply::RPL_WHOISCHANNELS(
            other.nickname.as_slice(),
            other.channels.read().iter().map(|(name, membership)| {
                (letter_for_membership(&*membership.modes.read()),
                name.as_slice())
            }).collect::<Vec<_>>()
            ),
        srv_name
    );
    if other.modes.read().get('o'.to_ascii()) {
        me.push_numreply(
            numericreply::RPL_WHOISOPERATOR(other.nickname.as_slice()),
            srv_name
        );
    }
    // TODO: compute idletime properly
    me.push_numreply(
        numericreply::RPL_WHOISIDLE(other.nickname.as_slice(), 0),
        srv_name
    );
}