//! Message handling.

//! This module contains the logic for handling messages through the server.
//!
//! This includes both `IRCMessage`, representing the IRC Protocol messages, and
//! `TextMessage`, representing a text message sent to a user or a channel.

#![experimental]

pub use self::ircmessage::IRCMessage;
pub use self::textmessage::{Actor, User, Server, Channel, Everybody, TextMessage};

mod ircmessage;
pub mod numericreply;
mod textmessage;