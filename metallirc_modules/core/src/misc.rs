use metallirc::messages::{IRCMessage, numericreply};
use metallirc::ServerData;
use metallirc::users::UserData;

use uuid::Uuid;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::CommandHandler;

use time::now;

pub struct CmdTime;

module!(CmdTime is CommandHandler)

impl CommandHandler for CmdTime {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "TIME" { return (false, Nothing); }

        user.push_numreply(
            numericreply::RPL_TIME(
                srv.settings.read().name.as_slice(),
                now().rfc822z().to_string().as_slice()
            ),
            srv.settings.read().name.as_slice()
        );

        (true, Nothing)
    }
}