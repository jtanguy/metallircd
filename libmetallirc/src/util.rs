//! Various utility functions.

#![experimental]

use std::io::{BufferedStream, IoResult};
use std::io::net::tcp::TcpStream;

use messages::IRCMessage;

/// Write an IRCMessage to a socket.
#[experimental]
pub fn write_message(socket: &mut BufferedStream<TcpStream>, msg: IRCMessage) -> IoResult<()> {
    try!(socket.write_str(msg.to_protocol().as_slice()));
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
#[experimental]
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

/// Returns whether given mask matches given label.
#[experimental]
pub fn matches_mask(mut label: &str, mut mask: &str) -> bool {
    while !label.is_empty() {
        match mask.slice_shift_char() {
            (Some('?'), mask_tail) => {
                let (_, label_tail) = label.slice_shift_char();
                // advance and continue
                mask = mask_tail;
                label = label_tail;
            },
            (Some('*'), mask_tail) => {
                let (_, label_tail) = label.slice_shift_char();
                return matches_mask(label, mask_tail) || matches_mask(label_tail, mask);
            },
            (Some(c), mask_tail) => {
                let (d, label_tail) = label.slice_shift_char();
                // d can not be None
                if c.to_lowercase() != d.unwrap().to_lowercase() { return false; }
                // advance and continue
                mask = mask_tail;
                label = label_tail;
            },
            (None, _) => { return false; }
        }
    }
    // "" can only be matched by "*" or ""
    mask.is_empty() || mask == "*"
}

#[cfg(test)]
mod test {

    use super::matches_mask;

    #[test]
    fn test_matches_mask() {
        assert!(matches_mask("foo", "foo"));
        assert!(matches_mask("foo", "fo?"));
        assert!(matches_mask("foo", "f?o"));
        assert!(matches_mask("foo", "?oo"));
        assert!(matches_mask("foo", "f*"));
        assert!(matches_mask("foo", "*"));
        assert!(!matches_mask("foo", "foo?"));
        assert!(!matches_mask("foo", "bar"));
        assert!(!matches_mask("foo", "f?oo"));
        assert!(!matches_mask("foo", "oo"));
        assert!(!matches_mask("foo", "fo"));
    }

}