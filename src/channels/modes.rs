//! Various levels of rights a user can have in a channel.

#![experimental]
// We do not use all members of bitflag...
#![allow(dead_code)]

#[experimental]
bitflags! {
    flags MembershipMode: u8 {
        static mm_none    = 0x00,
        static mm_voice   = 0x01,
        static mm_halfop  = 0x02,
        static mm_op      = 0x04,
        static mm_founder = 0x10
    }
}

#[experimental]
impl MembershipMode {

    pub fn is_at_least(&self, other: &MembershipMode) -> bool {
        self.bits as u8 >= other.bits as u8
    }

    pub fn best_mode(&self) -> MembershipMode {
        let mut mode = 1u8;
        while mode <= self.bits as u8 { mode <<= 1; }
        mode >>= 1;
        MembershipMode::from_bits(mode).unwrap()
    }

}