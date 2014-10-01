//! Operation and data structures handling user list

pub use self::user::{UserData, PrivateUserDataHandler};
pub use self::usermanager::UserManager;
pub use self::newuser::NewUser;

use settings::ServerSettings;

use irccp::{command, ToIRCMessage};

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
pub fn disconnect_user(id: &Uuid, manager: &UserManager, reason: &str, serverconf: &ServerSettings) {
    let future_zombie = manager.get_user_by_uuid(id).unwrap();
    let zombie_nickname = future_zombie.nickname.clone();
    let mut pu = future_zombie.private_handler();
    // we don't care about the result, it will be disconnected anyway.
    let _ = pu.socket_write_message(
        command::NOTICE(zombie_nickname, format!("You will be disconnected for the reason: {}", reason))
            .to_ircmessage().with_prefix(serverconf.name.as_slice()).ok().unwrap()
    );
    pu.zombify();
}

/// Recycles a disconnected user.
#[experimental]
pub fn recycle_user(id: &Uuid, manager: &mut UserManager) {
    manager.del_user(id);
    // TODO save historyfor WHOWAS
}
