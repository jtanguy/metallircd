use channels::Channel;
use messages::{IRCMessage, numericreply};
use scheduling::ServerData;
use users::UserData;

use uuid::Uuid;

use super::{RecyclingAction, Nothing};
use super::CommandHandler;

pub struct CmdList;

module!(CmdList is CommandHandler)

impl CommandHandler for CmdList {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "LIST" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(0,2) { // always true !
            if args.len() > 0 {
                for chan in args[0].as_slice().split_terminator(',') {
                    if let Some(handle) = srv.channels.read().chan_handle(chan) {
                        send_chan_in_list(user, chan, &*handle.read(), srv);
                    }
                }
            } else {
                // we just want all channels
                srv.channels.read().apply_to_chans(|name, handle| {
                    send_chan_in_list(user, name, handle, srv);
                });
            }
            user.push_message(
                IRCMessage {
                    prefix: Some(srv.settings.read().name.clone()),
                    command: numericreply::RPL_LISTEND.to_text(),
                    args: vec!(user.nickname.clone()),
                    suffix: Some("End of LIST.".to_string())
                }
            );
        } else {
            unreachable!();
        }

        (true, Nothing)
    }
}

#[inline(always)]
fn send_chan_in_list(user: &UserData, chan: &str, chandle: &Channel, srv: &ServerData) {
    user.push_message(
        IRCMessage {
            prefix: Some(srv.settings.read().name.clone()),
            command: numericreply::RPL_LIST.to_text(),
            args: vec!(
                user.nickname.clone(),
                chan.to_string(),
                chandle.member_count().to_string(),
            ),
            suffix: Some(chandle.get_topic().to_string())
        }
    );
}