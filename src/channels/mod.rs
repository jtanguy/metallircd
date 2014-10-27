//! Chan handling module.

//! This module contains the logic for managing channels and their members.

#![experimental]

pub use self::chan::Membership;
pub use self::chan::Channel;
pub use self::manager::ChannelManager;

mod chan;
mod manager;
