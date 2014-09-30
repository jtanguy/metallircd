//! Operation and data structures handling user list

pub use self::user::{UserData, PrivateUserDataHandler};
pub use self::usermanager::UserManager;
pub use self::newuser::NewUser;

use uuid::Uuid;

mod newuser;
mod user;
mod usermanager;

/// Handles a user, sending and receiving awaiting messages
#[experimental]
pub fn handle_user(id: &Uuid, manager: &UserManager) {
    // first, send its messages to the user
    let u = manager.get_user_by_uuid(id).unwrap();
    let mut pu = u.private_handler();
    while match pu.next_queued_message() {
        Some(msg) => match pu.socket_write_message(msg) {
            Ok(()) => true,
            Err(_) => false // interrupt sending
        },
        None => false
    } {}

    // TODO : handle user input
}

/// Forcibly disconnect the user for a server shutdown.
#[experimental]
pub fn disconnect_user(id: &Uuid, manager: &UserManager) {
    // TODO
}

/// Recycles a disconnected user.
#[experimental]
pub fn recycle_user(id: &Uuid, manager: &mut UserManager) {
    // TODO
}
