#![allow(non_uppercase_statics)]

///! The modes system.

macro_rules! def_modes(
    ($enum_name: ident, $(($name: ident, $chr: expr, $bitval: expr)),*) => (

bitflags!{
    flags $enum_name: u64 {
        $(const $name = $bitval),*
    }
}

impl $enum_name {

    /// Returns the char associated to given mode, or `\0` if
    /// not exactly one mode is set.
    pub fn to_char(&self) -> char {
        match *self {
            $($name => $chr),*,
            _ => '\0'
        }
    }

    pub fn from_char(c: char) -> Option<$enum_name> {
        match c {
            $(c if c == $chr => Some($name)),*,
            _ => None
        }
    }
}

    )
)

//
// User Modes
//

def_modes!(UserMode,
    (UInvisible,            'i', 0x0000000000000001)
)

//
// Channel Modes
//

def_modes!(ChanMode,
    (CSecret,               's', 0x0000000000000001)
)

//
// Membership Modes
//

// Should be kept by order of importance
def_modes!(MembershipMode,
    (MVoice,                'v', 0x0000000000000001),
    (MHalfOp,               'h', 0x0000000000000002),
    (MOp,                   'o', 0x0000000000000004)
)

impl MembershipMode {

    pub fn is_at_least(&self, other: &MembershipMode) -> bool {
        self.bits as u64 >= other.bits as u64
    }

    pub fn best_mode(&self) -> MembershipMode {
        let mut mode = 1u64;
        while mode <= self.bits as u64 { mode <<= 1; }
        mode >>= 1;
        MembershipMode::from_bits(mode).unwrap()
    }

    pub fn prefix(&self) -> Option<char> {
        if self.contains(MOp) {
            Some('@')
        } else if self.contains(MHalfOp) {
            Some('%')
        } else if self.contains(MVoice) {
            Some('+')
        } else {
            None
        }
    }

}