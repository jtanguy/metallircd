//!Channels structure.

#![experimental]

use std::collections::HashMap;
use std::collections::hashmap::Keys;

use time::now;

use uuid::Uuid;

use modes::{MembershipMode, ChanMode};

/// A channel.
#[experimental]
pub struct Channel {
    topic: String,
    members: HashMap<Uuid, MembershipMode>,
    pub modes: ChanMode,
    pub creation_time: i64
}

#[experimental]
impl Channel {

    /// Creates a new channel.
    #[experimental]
    pub fn new() -> Channel {
        Channel {
            topic: String::new(),
            members: HashMap::new(),
            modes: ChanMode::empty(),
            creation_time: now().to_timespec().sec
        }
    }

    /// Returns `true` if given user is in the chan.
    #[experimental]
    pub fn has_member(&self, user: &Uuid) -> bool {
        self.members.contains_key(user)
    }

    /// Returns the number of users in the chan.
    #[experimental]
    pub fn member_count(&self) -> uint {
        self.members.len()
    }

    /// Returns `true` is the chan is empty.
    #[experimental]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Adds the user to the chan. Does nothing if the user is already in.
    #[experimental]
    pub fn join(&mut self, user: Uuid) {
        if !self.members.contains_key(&user) {
            self.members.insert(user, MembershipMode::empty());
        }
    }

    /// Parts the user from the chan. Does nothing if the user was not in the chan.
    #[experimental]
    pub fn part(&mut self, user: &Uuid) {
        self.members.remove(user);
    }

    /// Applies given closure to all members of the chan.
    #[experimental]
    pub fn apply_to_members(&self, func: |_: &Uuid, _: &MembershipMode| -> ()) {
        for (u, m) in self.members.iter() { func(u, m); };
    }

    /// Returns a slice to the current topic of the chan.
    #[experimental]
    pub fn get_topic<'a>(&'a self) -> &'a str {
        self.topic.as_slice()
    }

    /// Sets the topic of the chan.
    #[experimental]
    pub fn set_topic(&mut self, topic: String) {
        self.topic = topic
    }

    /// Lists all members with given mode.
    #[experimental]
    pub fn members_being(&self, mode: MembershipMode) -> Vec<Uuid> {
        let mut v = Vec::new();
        for (u, m) in self.members.iter() {
            if m.contains(mode.clone()) { v.push(u.clone()); }
        }
        v
    }

    /// Lists all members with given mode or better.
    #[experimental]
    pub fn members_at_least(&self, mode: MembershipMode) -> Vec<Uuid> {
        let mut v = Vec::new();
        for (u, m) in self.members.iter() {
            if m.is_at_least(&mode) { v.push(u.clone()); }
        }
        v
    }

    /// Lists all members with their best mode.
    #[experimental]
    pub fn member_list(&self) -> Vec<(Uuid, MembershipMode)> {
        self.members.iter().map(|(u, m)| {
            (u.clone(), m.best_mode())
        }).collect()
    }

    /// List all members with their best mode as an iterator.
    #[experimental]
    pub fn members_iter(&self) -> Keys<Uuid, MembershipMode> {
        self.members.keys()
    }

    /// Checks if given user has given mode
    #[experimental]
    pub fn has_mode(&self, uuid: &Uuid, mode: MembershipMode) -> bool {
        self.members.find(uuid).map(|m| m.contains(mode)).unwrap_or(false)
    }

    /// Checks if given user has at least given mode
    #[experimental]
    pub fn is_at_least(&self, uuid: &Uuid, mode: MembershipMode) -> bool {
        self.members.find(uuid).map(|m| m.is_at_least(&mode)).unwrap_or(false)
    }

    /// Adds given mode to given user. Returns false if the user was
    /// not in the channel.
    #[experimental]
    pub fn add_mode_to(&mut self, uuid: &Uuid, mode: MembershipMode) -> bool {
        match self.members.find_mut(uuid) {
            Some(ref mut m) => { m.insert(mode); true },
            None => { false }
        }
    }

    /// Removes given mode from given user. Returns false if the user was
    /// not in the channel.
    #[experimental]
    pub fn remove_mode_from(&mut self, uuid: &Uuid, mode: MembershipMode) -> bool {
        match self.members.find_mut(uuid) {
            Some(ref mut m) => { m.remove(mode); true },
            None => { false }
        }
    }

}