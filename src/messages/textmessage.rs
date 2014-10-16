//! User text message.

#![experimental]

use uuid::Uuid;

use super::IRCMessage;

#[experimental]
pub enum Actor {
    Channel(String),
    Server(String),
    User(Uuid, String)
}

impl Actor {
    pub fn into_text(self) -> String{
        match self {
            Channel(s) => s,
            Server(s) => s,
            User(_, s) => s
        }
    }
}

#[experimental]
pub struct TextMessage {
    pub notice: bool,
    pub source: Actor,
    pub target: Actor,
    pub text: String
}

impl TextMessage {
    /// Generates the appropriate protocol text.
    /// Returns None if either source or target is a non-existing user.
    pub fn into_ircmessage(self) -> IRCMessage {
        IRCMessage {
            prefix: Some(self.source.into_text()),
            command: if self.notice { "NOTICE".to_string() } else { "PRIVMSG".to_string() },
            args: vec!(self.target.into_text()),
            suffix: Some(self.text)
        }
    }

}