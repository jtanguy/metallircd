//!Channels structure.

#![experimental]

use std::collections::HashMap;
use std::sync::{Weak, RWLock};

use time::now;

use uuid::Uuid;

use users::UserData;
use modes::{MembershipMode, ChanMode};

/// The membership of a user in a channel
#[experimental]
pub struct Membership {
    pub user: Weak<RWLock<UserData>>,
    pub channel: Weak<RWLock<Channel>>,
    pub modes: RWLock<MembershipMode>
}

/// A channel.
#[experimental]
pub struct Channel {
    pub name: String,
    pub topic: String,
    members: HashMap<Uuid, Weak<Membership>>,
    pub modes: ChanMode,
    pub creation_time: i64
}

#[experimental]
impl Channel {

    /// Creates a new channel.
    #[experimental]
    pub fn new(name: String) -> Channel {
        Channel {
            name: name,
            topic: String::new(),
            members: HashMap::new(),
            modes: ChanMode::empty(),
            creation_time: now().to_timespec().sec
        }
    }

    /// Returns the number of users in the chan.
    #[experimental]
    pub fn member_count(&self) -> uint {
        // We don't want to count ghost members
        let mut count = 0u;
        self.apply_to_members(|_, _| { count += 1; });
        count
    }

    /// Adds the user to the chan. Does nothing if the user is already in.
    #[experimental]
    pub fn join(&mut self, user: Uuid, m: Weak<Membership>) {
        if !self.members.contains_key(&user) {
            self.members.insert(user, m);
        }
    }

    /// Removes all phantom Weak<> in the chan.
    /// Returns true is the chan is now empty.
    #[experimental]
    pub fn cleanup(&mut self) -> bool {
        let ghosts = self.members.iter()
            .fold(Vec::new(), |mut v, (u, m)| { if m.upgrade().is_none() { v.push(u.clone()) } v });
        for u in ghosts.into_iter() {
            self.members.remove(&u);
        }
        self.members.is_empty()
    }

    /// Returns true if the chan is empty.
    #[experimental]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Parts the user from the chan. Does nothing if the user was not in the chan.
    #[experimental]
    pub fn part(&mut self, user: &Uuid) {
        self.members.remove(user);
    }

    /// Applies given closure to all members of the chan.
    #[experimental]
    pub fn apply_to_members(&self, func: |_: &Uuid, _: &Membership| -> ()) {
        for (u, m) in self.members.iter() {
            if let Some(handle) = m.upgrade() {
                func(u, &*handle);
            }
        };
    }

    /// Lists all members with given mode.
    #[experimental]
    pub fn members_being(&self, mode: MembershipMode) -> Vec<Uuid> {
        let mut v = Vec::new();
        self.apply_to_members(|u, m| {
            if m.modes.read().contains(mode.clone()) { v.push(u.clone()); }
        });
        v
    }

    /// Lists all members with given mode or better.
    #[experimental]
    pub fn members_at_least(&self, mode: MembershipMode) -> Vec<Uuid> {
        let mut v = Vec::new();
        self.apply_to_members(|u, m| {
            if m.modes.read().is_at_least(&mode) { v.push(u.clone()); }
        });
        v
    }

    /// Lists all members with their best mode.
    #[experimental]
    pub fn member_list(&self) -> Vec<(Uuid, MembershipMode)> {
        let mut v = Vec::with_capacity(self.members.len());
        self.apply_to_members(|u, m| {
            v.push((u.clone(), m.modes.read().best_mode()));
        });
        v
    }

}