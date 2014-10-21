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

    pub fn to_modestring(&self) -> String {
        let mut txt = "+".to_string();
        $( if self.contains($name) { txt.push_char($chr); } )*
        if txt.len() > 1 { txt } else { String::new() }
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
/// User modes
//
#[experimental]
def_modes!(UserMode,
    (UInvisible,            'i', 0x0000000000000001),
    (UOperator,             'o', 0x0000000000000002)
)

pub static umodes_not_self_activable: &'static str = "o";
pub static umodes_not_self_deactivable: &'static str = "";

//
/// Channel Modes
//
#[experimental]
def_modes!(ChanMode,
    (CSecret,               's', 0x0000000000000001),
    (CNoExternalMsg,        'n', 0x0000000000000002),
    (CModerated    ,        'm', 0x0000000000000004)
)

//
/// Membership Modes
//
#[experimental]
// Should be kept by order of importance
def_modes!(MembershipMode,
    (MVoice,                'v', 0x0000000000000001),
    (MOp,                   'o', 0x0000000000000004)
)

impl MembershipMode {

    /// Returns `true` if current mode of user is equal or better than `other`.
    #[experimental]
    pub fn is_at_least(&self, other: &MembershipMode) -> bool {
        self.bits as u64 >= other.bits as u64
    }

    /// Returns the best mode contained in this MembershipMode.
    #[experimental]
    pub fn best_mode(&self) -> MembershipMode {
        let mut mode = 1u64;
        while mode <= self.bits as u64 { mode <<= 1; }
        mode >>= 1;
        MembershipMode::from_bits(mode).unwrap()
    }

    /// Returns the char prefix associated with current mode,
    /// or `None` if no mode is set.
    #[experimental]
    pub fn prefix(&self) -> Option<char> {
        if self.contains(MOp) {
            Some('@')
        } else if self.contains(MVoice) {
            Some('+')
        } else {
            None
        }
    }

}