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

        user.push_message(
            IRCMessage {
                prefix: Some(srv.settings.read().name.clone()),
                command: numericreply::RPL_TIME.to_text(),
                args: vec!(user.nickname.clone(), srv.settings.read().name.clone()),
                suffix: Some(now().rfc822z())
            }
        );

        (true, Nothing)
    }
}