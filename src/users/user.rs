//! User description.

#![experimental]

use std::collections::{HashMap, HashSet};
use std::io::{BufferedStream, IoResult, IoError};
use std::io::net::tcp::TcpStream;
use std::sync::{Arc, Mutex, MutexGuard, RWLock};
use std::sync::mpsc_queue::Queue as MPSCQueue;

use channels::Membership;
use messages::IRCMessage;
use modes::UserMode;
use util;

use uuid::Uuid;

/// Data describing a user.
#[experimental]
pub struct UserData {
    /// The TcpStream of this user. Mutex protected.
    socket: Mutex<BufferedStream<TcpStream>>,
    /// The queue of this user.
    queue: MPSCQueue<IRCMessage>,
    pub id: Uuid,
    pub nickname: String,
    pub username: String,
    pub hostname: String,
    pub realname: String,
    pub modes: RWLock<UserMode>,
    pub channels: RWLock<HashMap<String, Arc<Membership>>>,
    /// is this user disconnected ?
    zombie: RWLock<bool>
}

/// Private handler for this user_data
#[experimental]
pub struct PrivateUserDataHandler<'a> {
    data: &'a UserData,
    socket: MutexGuard<'a BufferedStream<TcpStream>>
}

#[experimental]
impl<'a> PrivateUserDataHandler<'a> {

    /// Retrieves next item in this user's private queue (if any).
    #[experimental]
    pub fn next_queued_message(&self) -> Option<IRCMessage> {
        self.data.queue.casual_pop()
    }

    /// Retrieves next message from this client.
    #[experimental]
    pub fn socket_read_message(&mut self) -> IoResult<IRCMessage> {
        match self.socket.read_line() {
            Ok(txt) => match from_str::<IRCMessage>(txt.as_slice().lines_any().next().unwrap()) {
                Some(msg) => Ok(msg),
                None => Err(IoError::from_errno(22u, false)), // InvalidInput
            },
            Err(e) => Err(e)
        }
    }

    /// Sends given message to the client.
    #[experimental]
    pub fn socket_write_message(&mut self, msg: IRCMessage) -> IoResult<()> {
        util::write_message(&mut *self.socket, msg)
    }

    /// Marks a client as zombie, to be recycled.
    #[experimental]
    pub fn zombify(&mut self) {
        *self.data.zombie.write() = true
    }

}

#[experimental]
impl UserData {

    #[experimental]
    /// Creates a new user
    pub fn new(tcpsocket: BufferedStream<TcpStream>, nick: String, id: Uuid,
               username: String, hostname: String, realname: String) -> UserData {
        UserData {
            socket: Mutex::new(tcpsocket),
            queue: MPSCQueue::new(),
            id: id,
            nickname: nick,
            username: username,
            hostname: hostname,
            realname: realname,
            channels: RWLock::new(HashMap::new()),
            modes: RWLock::new(UserMode::empty()),
            zombie: RWLock::new(false)
        }
    }

    /// Pushes a message to this user's personnal queue.
    #[experimental]
    pub fn push_message(&self, msg: IRCMessage) {
        self.queue.push(msg);
    }

    /// Sends given message to all known users
    pub fn send_to_known(&self, msg: IRCMessage) {
        let mut done: HashSet<Uuid> = HashSet::new();
        for c in self.channels.read().values() {
            if let Some(chan) = c.channel.upgrade() {
                chan.read().apply_to_members(|u, m| {
                    if !done.contains(u) {
                        done.insert(u.clone());
                        if let Some(other) = m.user.upgrade() {
                            other.read().push_message(msg.clone());
                        }
                    }
                });
            }
        }
    }

    /// Retrieves the private handler to this user. Only on can exist at a given time.
    /// If an other one exists, will block util it is released.
    #[experimental]
    pub fn private_handler<'a>(&'a self) -> PrivateUserDataHandler<'a> {
        PrivateUserDataHandler {
            data: self,
            socket: self.socket.lock()
        }
    }

    /// Returns the full_name of this user
    #[experimental]
    pub fn get_fullname(&self) -> String {
        let mut result = self.nickname.to_string();
        result.push_str("!");
        result.push_str(self.username.as_slice());
        result.push_str("@");
        result.push_str(self.hostname.as_slice());
        result
    }

    /// Is this user zombified ?
    #[experimental]
    pub fn is_zombie(&self) -> bool {
        *self.zombie.read()
    }

    /// Is this user in given chan ?
    #[experimental]
    pub fn membership<'a>(&'a self, chan: &str) -> Option<Arc<Membership>> {
        let lowerchan = util::label_to_lower(chan);
        self.channels.read().find(&lowerchan).map(|a| a.clone())
    }
}
