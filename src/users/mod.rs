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
    // TODO
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
