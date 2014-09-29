//! User description.

#![experimental]

use std::io::{BufferedStream, IoResult, IoError};
use std::io::net::tcp::TcpStream;
use std::sync::{Mutex, MutexGuard};
use std::sync::mpsc_queue::Queue as MPSCQueue;

use irccp::IRCMessage;

/// Data describing a user.
#[experimental]
pub struct UserData {
    /// The TcpStream of this user. Mutex protected.
    socket: Mutex<BufferedStream<TcpStream>>,
    /// The queue of this user.
    queue: MPSCQueue<IRCMessage>,
    pub nickname: String,
    pub username: String,
    pub hostname: String,
    /// is this user disconnected ?
    pub zombie: bool
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
            Ok(txt) => match from_str::<IRCMessage>(txt.as_slice()) {
                Some(msg) => Ok(msg),
                None => Err(IoError::from_errno(22u, false)), // InvalidInput
            },
            Err(e) => Err(e)
        }
    }

    /// Sends given message to the client.
    #[experimental]
    pub fn socket_write_message(&mut self, msg: IRCMessage) -> IoResult<()> {
        self.socket.write_str({
            let mut to_send = msg.to_protocol_text();
            to_send.push_str("\r\n");
            to_send
        }.as_slice())
    }

}

#[experimental]
impl UserData {

    #[experimental]
    /// Creates a new user
    pub fn new(tcpsocket: BufferedStream<TcpStream>, nick: String, username: String, hostname: String) -> UserData {
        UserData {
            socket: Mutex::new(tcpsocket),
            queue: MPSCQueue::new(),
            nickname: nick,
            username: username,
            hostname: hostname,
            zombie: false
        }
    }

    /// Pushes a message to this user's personnal queue.
    pub fn push_message(&self, msg: IRCMessage) {
        self.queue.push(msg);
    }

    /// Retrieves the private handler to this user. Only on can exist at a given time.
    /// If an other one exists, will block util it is released.
    pub fn private_handler<'a>(&'a self) -> PrivateUserDataHandler<'a> {
        PrivateUserDataHandler {
            data: self,
            socket: self.socket.lock()
        }
    }

    /// Returns the full_name of this user
    pub fn get_fullname(&self) -> String {
        let mut result = self.nickname.to_string();
        result.push_str("!");
        result.push_str(self.username.as_slice());
        result.push_str("@");
        result.push_str(self.hostname.as_slice());
        result
    }

}
