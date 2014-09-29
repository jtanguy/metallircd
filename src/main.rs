//! metallircd

extern crate irccp;
extern crate uuid;

use std::sync::RWLock;

pub mod users;

fn main() {
    let user_manager = RWLock::new(users::UserManager::new());
}
