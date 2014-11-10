use metallirc::ServerData;

use metallirc::modules::{RecyclingAction, Nothing, Zombify, ChangeNick};

use metallirc::logging::Debug;

use std::io;

use metallirc::messages::{IRCMessage, numericreply};

use uuid::Uuid;

/// Handles a user, sending and receiving awaiting messages
#[experimental]
pub fn handle_user(id: &Uuid, srv: &ServerData) -> RecyclingAction {
    // first, send its messages to the user
    let manager = srv.users.read();
    let u = &*manager.get_user_by_uuid(id).unwrap();
    let mut pu = u.private_handler();
    while match pu.next_queued_message() {
        Some(msg) => match pu.socket_write_message(msg) {
            Ok(()) => true,
            Err(_) => false // interrupt sending
        },
        None => false
    } {}

    while match pu.socket_read_message() {
        Ok(msg) => match srv.modules_handler.read().handle_command(u, id, msg, srv) {
            Zombify => { pu.zombify(); return Nothing; },
            Nothing => true,
            act => { return act; }
        },
        // TODO proper error handling
        Err(e) => {
            if e.kind == io::TimedOut {
                // nothing, it's normal
            } else {
                // connection error, zombify
                u.send_to_known(
                    IRCMessage {
                        prefix: Some(u.get_fullname()),
                        command: "QUIT".to_string(),
                        args: vec!(),
                        suffix: Some("Connection closed.".to_string())
                    }
                );
                pu.zombify()
            }
            // in all cases, stop looping
            false
        }
    } {}

    return Nothing;
}

/// Forcibly disconnect the user for a server shutdown.
#[experimental]
pub fn disconnect_user(id: &Uuid, srv: &ServerData, reason: &str) {
    let manager = srv.users.read();
    let future_zombie = manager.get_user_by_uuid(id).unwrap();
    let zombie_nickname = future_zombie.nickname.clone();
    let mut pu = future_zombie.private_handler();
    // we don't care about the result, it will be disconnected anyway.
    let _ = pu.socket_write_message(
        IRCMessage {
            prefix: Some(srv.settings.read().name.clone()),
            command: "NOTICE".to_string(),
            args: vec!(zombie_nickname),
            suffix: Some(format!("You will be disconnected for the reason: {}", reason))
        }
    );
    pu.zombify();
}

/// Perform a recycling action on given user.
#[experimental]
pub fn recycle_user(id: &Uuid, action: RecyclingAction, srv: &ServerData) {
    match action {
        ChangeNick(new_nick) => {
            let (success, old_name) = {
                let mut manager = srv.users.write();
                let old_name = manager.get_user_by_uuid(id).unwrap().get_fullname();
                (manager.change_nick(id, &new_nick), old_name)
            };
            if success {
                srv.users.read().get_user_by_uuid(id).unwrap().send_to_known(IRCMessage {
                    prefix: Some(old_name),
                    command: "NICK".to_string(),
                    args: vec!(new_nick),
                    suffix: None,
                });
            } else {
                srv.users.read().get_user_by_uuid(id).unwrap().push_numreply(
                    numericreply::ERR_NICKNAMEINUSE(new_nick.as_slice()),
                    srv.settings.read().name.as_slice()
                );
            }
        },
        _ => {}
    }
}

/// Recycles a disconnected user.
#[experimental]
pub fn destroy_user(id: &Uuid, srv: &ServerData) {
    srv.logger.log(Debug, format!("Recycling user {}", id));
    srv.users.write().del_user(id);
    // TODO save historyfor WHOWAS
}
