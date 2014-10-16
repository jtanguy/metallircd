//! Message handling.

#![experimental]

pub use self::ircmessage::IRCMessage;
pub use self::textmessage::{Actor, TextMessage};

mod ircmessage;
pub mod numericreply;
mod textmessage;