//! Message handling.

#![experimental]

pub use self::ircmessage::IRCMessage;
pub use self::textmessage::{Actor, User, Server, Channel, Everybody, TextMessage};

mod ircmessage;
pub mod numericreply;
mod textmessage;