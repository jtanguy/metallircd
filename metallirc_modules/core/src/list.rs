use metallirc::channels::Channel;
use metallirc::messages::{IRCMessage, numericreply};
use metallirc::ServerData;
use metallirc::users::UserData;

use uuid::Uuid;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::CommandHandler;

pub struct CmdList;

module!(CmdList is CommandHandler)

impl CommandHandler for CmdList {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "LIST" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(0,2) { // always true !
            if args.len() > 0 {
                for chan in args[0].as_slice().split_terminator(',') {
                    if chan.contains_char('?') || chan.contains_char('*') {
                        // its a mask
                        srv.channels.read().apply_to_chans_matching(chan, |handle| {
                            // hide secret chans
                            if !handle.modes.get('s'.to_ascii()) {
                                send_chan_in_list(user, handle, srv);
                            }
                        });
                    } else if let Some(handle) = srv.channels.read().chan_handle(chan) {
                        // hide secret chans
                        if !handle.read().modes.get('s'.to_ascii()) {
                            send_chan_in_list(user, &*handle.read(), srv);
                        }
                    }
                }
            } else {
                // we just want all channels
                srv.channels.read().apply_to_chans(|handle| {
                    // hide secret chans
                    if !handle.modes.get('s'.to_ascii()) {
                        send_chan_in_list(user, handle, srv);
                    }
                });
            }
            user.push_numreply(
                numericreply::RPL_LISTEND,
                srv.settings.read().name.as_slice()
            );
        } else {
            unreachable!();
        }

        (true, Nothing)
    }
}

#[inline(always)]
fn send_chan_in_list(user: &UserData, chandle: &Channel, srv: &ServerData) {
    user.push_numreply(
        numericreply::RPL_LIST(
            chandle.name.as_slice(),
            chandle.member_count(),
            chandle.topic.as_slice()
        ),
        srv.settings.read().name.as_slice()
    );
}