//! Various utility functions.

use std::io::{BufferedStream, IoResult};
use std::io::net::tcp::TcpStream;

use irccp::IRCMessage;

/// Write an IRCMessage to a socket.
#[experimental]
pub fn write_message(socket: &mut BufferedStream<TcpStream>, msg: IRCMessage) -> IoResult<()> {
    try!(socket.write_str(msg.to_protocol_text().as_slice()));
    try!(socket.write_str("\r\n"));
    try!(socket.flush());
    Ok(())
}
