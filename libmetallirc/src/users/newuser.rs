//! Structs for handling new connections

use std::io;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use messages::{IRCMessage, numericreply};

use conf::ServerConf;
use util;

/// A user with possibly missing data, not to be shared until
/// initial negociation is done and a proper user is created.
#[experimental]
pub struct NewUser {
    pub socket: BufferedStream<TcpStream>,
    pub nickname: Option<String>,
    pub username: Option<String>,
    pub realname: Option<String>,
    pub zombie: bool
}

#[experimental]
impl NewUser {

    /// Creates a NewUser from a connection
    #[experimental]
    pub fn new(socket: BufferedStream<TcpStream>) -> NewUser {
        NewUser {
            socket: socket,
            nickname: None,
            username: None,
            realname: None,
            zombie: false
        }
    }

    #[experimental]
    fn err_reply(&mut self, server: &ServerConf, code: numericreply::NumericReply, arg: &str, suffix: &str) {
        if util::write_message(&mut self.socket,
                IRCMessage {
                    prefix: Some(server.name.clone()),
                    command: code.to_text(),
                    args: vec!(arg.to_string()),
                    suffix: Some(suffix.to_string())
                }
            ).is_err()
        {
            self.zombie = true;
        }
    }

    /// Read next message in negociation of new user.
    /// Returns whether the user is ready to be promoted to a real user or not.
    #[experimental]
    pub fn step_negociate(&mut self, server: &ServerConf) {
        match self.socket.read_line() {
            // got a line
            Ok(txt) => match from_str::<IRCMessage>(txt.as_slice().lines_any().next().unwrap()) {
                Some(msg) => match msg.command.as_slice() {
                    "USER" => if let Some(args) = msg.as_nparams(4,0) {
                        // TODO : check validity
                        // TODO : allow only once
                        self.username = Some(args[0].clone());
                        self.realname = Some(args[3].clone());
                    } else {
                        self.err_reply(server, numericreply::ERR_NEEDMOREPARAMS,
                                        "USER",
                                        "Not enough parameters.")
                    },
                    "NICK" => if let Some(mut args) = msg.as_nparams(1,0) {
                        let nick = args.pop().unwrap();
                        if util::check_label(nick.as_slice()) {
                            self.nickname = Some(nick);
                        } else {
                            self.err_reply(server, numericreply::ERR_ERRONEUSNICKNAME, "",
                               format!("{} : Erroneous nickname.", nick).as_slice());
                        }
                    }else {
                        self.err_reply(server, numericreply::ERR_NEEDMOREPARAMS,
                                        "NICK",
                                        "Not enough parameters.")
                    },
                    _ => {}
                },
                None => {}
            },
            // errors
            Err(e) => match e.kind {
                // timeouts are normal
                io::TimedOut => { return; },
                // not valid UTF8 ?
                io::InvalidInput => self.err_reply(server,
                                                   numericreply::ERR_UNKNOWNCOMMAND,
                                                   "UTF8-required",
                                                   "Only UTF8 input is supported."),
                // other errors means death, I guess ?
                // TODO : be sure of it
                _ => { self.zombie = true; }
            }
        }
    }

    /// Checks whether the new user is ready to be promoted
    #[experimental]
    pub fn is_ready(&self) -> bool {
        self.nickname.is_some() && self.username.is_some() && self.realname.is_some()
    }

    /// Invalidates the nick with ad "nick already in use" message
    #[experimental]
    pub fn report_unavailable_nick(&mut self, server: &ServerConf) {
        let oldnick = self.nickname.take().unwrap();
        self.err_reply(server, numericreply::ERR_NICKNAMEINUSE, oldnick.as_slice(), "Nickname is already in use.");
        self.nickname = None;
    }

}
