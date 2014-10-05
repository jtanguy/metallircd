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

/// Checks if a label (nick or chan name (excluding prefix)) is valid
#[experimental]
pub fn check_label(label: &str) -> bool {
    // only ascii nicks are allowed
    if label.len() == 0 || !label.is_ascii() { return false; }
    // digit is forbidden in first place
    if label.chars().next().unwrap().is_digit() { return false; }
    // only, letter, digit, or []{}\|^
    for c in label.chars() {
        if !c.is_alphanumeric() && !"{}|^[]\\-_`".contains_char(c) { return false; }
    }
    true
}

/// Checks if a chan name (including prefix) is valid
pub fn check_channame(mask: &str) -> bool {
    mask.starts_with("#") && check_label(mask.slice_from(1))
}

/// Returns the lower-case version of a label, nick or chan name.
/// Assumes it is valid.
#[experimental]
pub fn label_to_lower(nick: &str) -> String {
    nick.chars().map(|c| {
        match c {
            '[' => '{',
            ']' => '}',
            '\\' => '|',
            _ => c.to_lowercase()
        }
    }).collect()
}
