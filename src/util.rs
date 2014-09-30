//! Various utility functions.

use std::io::{BufferedStream, IoResult};
use std::io::net::tcp::TcpStream;

use irccp::IRCMessage;

/// Write an IRCMessage to a socket.
#[experimental]
pub fn write_message(socket: &mut BufferedStream<TcpStream>, msg: IRCMessage) -> IoResult<()> {
    try!(socket.write_str(msg.to_protocol_text().as_slice()));
    try!(socket.write_str("\r\n"));
    try!(socket.flush());
    Ok(())
}

/// Checks if an nickname is valid
#[experimental]
pub fn check_nick(nick: &str) -> bool {
    // only ascii nicks are allowed
    if nick.len() == 0 || !nick.is_ascii() { return false; }
    // digit is forbidden in first place
    if nick.chars().next().unwrap().is_digit() { return false; }
    // only, letter, digit, or []{}\|^
    for c in nick.chars() {
        if !c.is_alphanumeric() && !"{}|^[]\\".contains_char(c) { return false; }
    }
    true
}

/// Returns the lower-case version of a nick
/// Assumes the nick is valid.
#[experimental]
pub fn nick_to_lower(nick: &str) -> String {
    nick.chars().map(|c| {
        match c {
            '[' => '{',
            ']' => '}',
            '\\' => '|',
            _ => c.to_lowercase()
        }
    }).collect()
}
