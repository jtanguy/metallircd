//! Operation and data structures handling user list

pub use self::user::{UserData, PrivateUserDataHandler};
pub use self::usermanager::UserManager;
pub use self::newuser::NewUser;

use settings::ServerSettings;

use irccp::{command, numericreply, ToIRCMessage};

use uuid::Uuid;

mod newuser;
mod user;
mod usermanager;
mod input_handling;

/// Special actions to be performed by the recycler thread
#[experimental]
#[deriving(PartialEq)]
pub enum RecyclingAction {
    /// Nothing to do
    Nothing,
    /// a nick change is requested
    ChangeNick(String),
    /// the user should be zombified
    Zombify
}

/// Handles a user, sending and receiving awaiting messages
#[experimental]
pub fn handle_user(id: &Uuid, manager: &UserManager, serverconf: &ServerSettings) -> RecyclingAction {
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

    while match pu.socket_read_message() {
        Ok(msg) => match input_handling::handle_command(u, msg, manager, serverconf) {
            Zombify => { pu.zombify(); return Nothing; },
            Nothing => true,
            act => { return act; }
        },
        // TODO proper error handling
        Err(_) => false
    } {}

    return Nothing;
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

pub fn recycle_user(id: &Uuid, action: RecyclingAction, manager: &mut UserManager, serverconf: &ServerSettings) {
    match action {
        ChangeNick(new_nick) => {
            let old_name = manager.get_user_by_uuid(id).unwrap().get_fullname();
            let success = manager.change_nick(id, &new_nick);
            if success {
                manager.get_user_by_uuid(id).unwrap().push_message(
                    command::NICK(new_nick).to_ircmessage()
                        .with_prefix(old_name.as_slice()).ok().unwrap()
                );
            } else {
                manager.get_user_by_uuid(id).unwrap().push_message(
                    numericreply::ERR_NICKNAMEINUSE.to_ircmessage()
                        .with_prefix(serverconf.name.as_slice()).ok().unwrap()
                        .add_arg(new_nick.as_slice()).ok().unwrap()
                        .with_suffix("Nickname is already in use.").ok().unwrap()
                );
            }
        },
        _ => {}
    }
}

/// Recycles a disconnected user.
#[experimental]
pub fn destroy_user(id: &Uuid, manager: &mut UserManager) {
    println!("Recycling user {}", id);
    manager.del_user(id);
    // TODO save historyfor WHOWAS
}
