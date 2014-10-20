//! The channel manager.

#![experimental]

use std::collections::{HashMap, HashSet};
use std::collections::hashmap::{Occupied, Vacant};
use std::sync::RWLock;

use messages::IRCMessage;

use uuid::Uuid;

use super::chan::Channel;
use modes::MembershipMode;
use util;
use users::UserManager;

/// The channel manager.
///
/// Only one is to be created for in a server. It handles the channels and provides
/// a simple interface for its usage in the rest of the server.
#[experimental]
pub struct ChannelManager {
    chans: HashMap<String, RWLock<Channel>>
}

#[experimental]
impl ChannelManager {

    /// Creates a new ChannelManager.
    #[experimental]
    pub fn new() -> ChannelManager {
        ChannelManager {
            chans: HashMap::new()
        }
    }

    /// Add given user to given chan.
    /// Does nothing if the user was already in the chan.
    /// Does nothing and returns `false` if the chan didn't exist.
    #[experimental]
    pub fn join(&self, user: Uuid, chan: &str) -> bool {
        match self.chans.find(&util::label_to_lower(chan)) {
            Some(lock) => {
                lock.write().join(user);
                true
            },
            None => false
        }
    }

    /// Add given user to given chan, creating it if not existing.
    /// Does nothing if the user was already in the chan.
    #[experimental]
    pub fn join_create(&mut self, user: Uuid, chan: &str) {
        match self.chans.entry(util::label_to_lower(chan)) {
            Occupied(e) => e.get().write().join(user),
            Vacant(e) => {
                e.set(RWLock::new(Channel::new())).write().join(user)
            },
        }
    }

    /// Returns `true` if the channel exists.
    #[experimental]
    pub fn has_chan(&self, chan: &str) -> bool {
        self.chans.contains_key(&util::label_to_lower(chan.as_slice()))
    }

    /// Returns `true` if given user is in given chan.
    #[experimental]
    pub fn is_in_chan(&self, user: &Uuid, chan: &str) -> bool {
        match self.chans.find(&util::label_to_lower(chan)) {
            Some(ref ch) => ch.read().has_member(user),
            None => false
        }
    }

    /// Parts the user from given chan.
    /// Returns true if the chan is empty after this part.
    #[experimental]
    pub fn part(&self, user: &Uuid, chan: &str) -> bool {
        match self.chans.find(&util::label_to_lower(chan)) {
            Some(ch) => { ch.write().part(user); ch.read().is_empty() },
            None => false
        }
    }

    /// Destroys the chan if existing and empty and returns true, returns false otherwise.
    #[experimental]
    pub fn destroy_if_empty(&mut self, chan: &str) -> bool {
        let lower_chan = util::label_to_lower(chan);
        if self.chans.contains_key(&lower_chan)
           && self.chans.find(&lower_chan).unwrap().read().is_empty() {
            self.chans.remove(&lower_chan)
        } else {
            false
        }
    }

    /// Sends a message to a chan, ommiting an optional user.
    /// Returns false if the chan didn't exists.
    #[experimental]
    pub fn send_to_chan(&self, users: &UserManager, chan: &str, msg: IRCMessage, exclude: Option<Uuid>) -> bool {
        match self.chans.find(&util::label_to_lower(chan)) {
            None => false,
            Some(ref channel) => {
                match exclude {
                    Some(ref id) => channel.read().apply_to_members(|u, _| {
                        if u != id { users.get_user_by_uuid(u).unwrap().push_message(msg.clone()); }
                    }),
                    None => channel.read().apply_to_members(|u, _| {
                        users.get_user_by_uuid(u).unwrap().push_message(msg.clone());
                    })
                }
            true
            }
        }
    }

    /// Set of all user Uuid having at least one chan in common with given user Uuid, including itself.
    #[experimental]
    pub fn known_by_uuid(&self, user: &Uuid) -> HashSet<Uuid> {
        let mut set = HashSet::new();
        for chan in self.chans.values() {
            if chan.read().has_member(user) {
                set.extend(chan.read().members_iter().map(|u| u.clone()));
            }
        }
        set
    }

    /// Returns a handle to given chan

    pub fn chan_handle<'a>(&'a self, chan: &str) -> Option<&'a RWLock<Channel>> {
        self.chans.find(&util::label_to_lower(chan.as_slice()))
    }

    /// Parts the user from all chans and advertise everybody he knows.
    /// Returns a vec of the chans left empty by its departure.
    #[experimental]
    pub fn quit(&self, users: &UserManager, user: &Uuid, msg: IRCMessage) -> Vec<String> {
        for u in self.known_by_uuid(user).into_iter() {
            users.get_user_by_uuid(&u).unwrap().push_message(msg.clone());
        }
        let mut emptied = Vec::new();
        for (name, chan) in self.chans.iter() {
            chan.write().part(user);
            if chan.read().is_empty() {
                emptied.push(name.clone());
            }
        }
        emptied
    }

    /// Returns the member list of a chan with their best mode, as an iterator.
    #[experimental]
    pub fn member_list(&self, chan: &str) -> Vec<(Uuid, MembershipMode)> {
        match self.chans.find(&util::label_to_lower(chan)) {
            Some(ch) => ch.read().member_list(),
            None => Vec::new()
        }
    }

}