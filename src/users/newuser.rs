//! Structs for handling new connections

use std::io;
use std::io::BufferedStream;
use std::io::net::tcp::TcpStream;

use irccp;
use irccp::{IRCMessage, command, numericreply, ToIRCMessage};
use irccp::from_ircmessage;

use settings::ServerSettings;
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
    fn err_reply(&mut self, server: &ServerSettings, code: numericreply::NumericReply, arg: &str, suffix: &str) {
        match util::write_message(&mut self.socket,
                code.to_ircmessage()
                    .with_prefix(server.name.as_slice()).ok().unwrap()
                    .add_arg(arg).ok().unwrap()
                    .with_suffix(suffix).ok().unwrap()) {
            Err(_) => { self.zombie = true; },
            _ => {}
        }
    }

    /// Read next message in negociation of new user.
    /// Returns whether the user is ready to be promoted to a real user or not.
    #[experimental]
    pub fn step_negociate(&mut self, server: &ServerSettings) {
        match self.socket.read_line() {
            // got a line
            Ok(txt) => match from_str::<IRCMessage>(txt.as_slice().lines_any().next().unwrap()) {
                Some(msg) => match from_ircmessage::<command::Command>(&msg) {
                    Ok(command::USER(username, _, realname)) => {
                        // TODO : check validity
                        // TODO : allow only once
                        self.username = Some(username);
                        self.realname = Some(realname);
                    },
                    Ok(command::NICK(nick)) => {
                        if util::check_label(nick.as_slice()) {
                            self.nickname = Some(nick);
                        } else {
                            self.err_reply(server, numericreply::ERR_ERRONEUSNICKNAME, "",
                               format!("{} : Erroneous nickname.", nick).as_slice());
                        }
                    },
                    Err(irccp::TooFewParameters) => self.err_reply(server,
                                                                   numericreply::ERR_NEEDMOREPARAMS,
                                                                   msg.command.as_slice(),
                                                                   "Not enough parameters."),
                    // at this stage, ignore everithing else
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
    pub fn report_unavailable_nick(&mut self, server: &ServerSettings) {
        let oldnick = self.nickname.take().unwrap();
        self.err_reply(server, numericreply::ERR_NICKNAMEINUSE, oldnick.as_slice(), "Nickname is already in use.");
        self.nickname = None;
    }

}
