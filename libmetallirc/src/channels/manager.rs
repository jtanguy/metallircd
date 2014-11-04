//! The channel manager.

#![experimental]

use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};
use std::sync::{Arc, RWLock};

use messages::IRCMessage;

use uuid::Uuid;

use super::chan::{Channel, Membership};
use modes::Modes;
use util;
use users::UserData;

/// The channel manager.
///
/// Only one is to be created for in a server. It handles the channels and provides
/// a simple interface for its usage in the rest of the server.
#[experimental]
pub struct ChannelManager {
    chans: HashMap<String, Arc<RWLock<Channel>>>
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

    #[experimental]
    fn do_join(user: &Arc<RWLock<UserData>>, chan: &Arc<RWLock<Channel>>, name: String) {
        if user.read().channels.read().contains_key(&name) { return; }
        let membership = Arc::new(Membership {
            user: user.downgrade(),
            channel: chan.downgrade(),
            modes: RWLock::new(Modes::new())
        });
        chan.write().join(user.read().id.clone(), membership.downgrade());
        user.read().channels.write().insert(name.to_string(), membership);
    }

    /// Add given user to given chan.
    /// Does nothing if the user was already in the chan.
    /// Does nothing and returns `false` if the chan didn't exist.
    #[experimental]
    pub fn join(&self, user: &Arc<RWLock<UserData>>, chan: &str) -> bool {
        let lowerchan = util::label_to_lower(chan);
        match self.chans.find(&lowerchan) {
            Some(chan_arc) => {
                ChannelManager::do_join(user, chan_arc, lowerchan);
                true
            },
            None => false
        }
    }

    /// Add given user to given chan, creating it if not existing.
    /// Does nothing if the user was already in the chan.
    #[experimental]
    pub fn join_create(&mut self, user: &Arc<RWLock<UserData>>, chan: &str) {
        let lowerchan = util::label_to_lower(chan);
        match self.chans.entry(lowerchan.clone()) {
            Occupied(e) => {
                ChannelManager::do_join(user, e.get(), lowerchan);
            },
            Vacant(e) => {
                let arc = e.set(Arc::new(RWLock::new(Channel::new(chan.to_string()))));
                ChannelManager::do_join(user, arc, lowerchan);
            },
        }
    }

    /// Returns `true` if the channel exists.
    #[experimental]
    pub fn has_chan(&self, chan: &str) -> bool {
        self.chans.contains_key(&util::label_to_lower(chan.as_slice()))
    }

    /// Destroys the chan if existing and empty and returns true, returns false otherwise.
    #[experimental]
    pub fn destroy_if_empty(&mut self, chan: &str) -> bool {
        let lower_chan = util::label_to_lower(chan);
        if self.chans.contains_key(&lower_chan)
        && self.chans.find(&lower_chan).unwrap().read().is_empty() {
            self.chans.remove(&lower_chan);
            true
        } else {
            false
        }
    }

    /// Sends a message to a chan, ommiting an optional user.
    /// Returns false if the chan didn't exists.
    #[experimental]
    pub fn send_to_chan(&self, chan: &str, msg: IRCMessage, exclude: Option<Uuid>) -> bool {
        match self.chans.find(&util::label_to_lower(chan)) {
            None => false,
            Some(ref channel) => {
                match exclude {
                    Some(ref id) => channel.read().apply_to_members(|u, m| {
                        if u != id {
                            m.user.upgrade().unwrap().read().push_message(msg.clone());
                        }
                    }),
                    None => channel.read().apply_to_members(|_, m| {
                        m.user.upgrade().unwrap().read().push_message(msg.clone());
                    })
                }
            true
            }
        }
    }

    /// Returns a handle to given chan
    #[experimental]
    pub fn chan_handle<'a>(&'a self, chan: &str) -> Option<&'a RWLock<Channel>> {
        self.chans.find(&util::label_to_lower(chan.as_slice())).map(|a| &**a)
    }

    /// Apply given closure to all chans.
    #[experimental]
    pub fn apply_to_chans(&self, f :|handle: &Channel|) {
        for h in self.chans.values() { f(&*h.read()); }
    }

    /// Apply given closure to all chans which name matches given mask.
    #[experimental]
    pub fn apply_to_chans_matching(&self, mask: &str, f :|handle: &Channel|) {
        for (n, h) in self.chans.iter() {
            if util::matches_mask(n.as_slice(), mask) { f(&*h.read());}
        }
    }

}