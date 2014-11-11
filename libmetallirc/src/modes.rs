//! A bitflag-like structure handling modes

#![experimental]

use std::ascii::Ascii;
use std::collections::Bitv;

#[experimental]
pub struct Modes {
    flags: Bitv
}

#[experimental]
impl Modes {

    #[experimental]
    pub fn new() -> Modes {
        Modes { flags: Bitv::with_capacity(52, false) }
    }

    #[experimental]
    #[inline]
    fn index_of(flag: Ascii) -> Option<uint> {
        match flag.to_byte() {
            b if b >= 65 && b <= 90 => {
                // Uppercase letter
                Some(b as uint - 65 + 26)
            },
            b if b >= 97 && b <= 122 => {
                // Lowercase letter
                Some(b as uint - 97)
            },
            _ => {
                // Anything else is illegal
                None
            }
        }
    }

    /// Returns `true` if no flag is set.
    pub fn none(&self) -> bool {
        self.flags.none()
    }

    /// Retrieves the value of given flag.
    /// Returns always `false` if input flag is invalid.
    #[experimental]
    pub fn get(&self, flag: Ascii) -> bool {
        match Modes::index_of(flag) {
            Some(b) => self.flags.get(b),
            None => false
        }
    }

    /// Sets the value of given flag.
    /// Does nothing if input flag is invalid.
    #[experimental]
    pub fn set(&mut self, flag: Ascii, v: bool) {
        if let Some(b) = Modes::index_of(flag) {
            self.flags.set(b, v)
        }
    }

    /// Returns a string containing all active modes.
    #[experimental]
    pub fn to_modestring(&self) -> String {
        let mut ret = "+".to_string();
        for i in range(0u8, 26) {
            if self.flags.get(i as uint) {
                ret.push((i + 97) as char);
            }
            if self.flags.get((i + 26) as uint) {
                ret.push((i + 65) as char);
            }
        }
        if ret.len() == 1 {
            String::new()
        } else {
            ret
        }
    }

}

impl Clone for Modes {
    fn clone(&self) -> Modes {
        Modes { flags: self.flags.clone() }
    }
}

pub fn letter_for_membership(m: &Modes) -> Option<char> {
    if m.get('o'.to_ascii()) {
        Some('@')
    } else if m.get('v'.to_ascii()) {
        Some('+')
    } else {
        None
    }
}