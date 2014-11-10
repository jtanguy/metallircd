use metallirc::messages::{IRCMessage, numericreply};
use metallirc::ServerData;
use metallirc::users::UserData;

use uuid::Uuid;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::{CommandHandler, send_needmoreparams};

pub struct CmdTopic;

module!(CmdTopic is CommandHandler)

impl CommandHandler for CmdTopic {
    fn handle_command(&self, user: &UserData, _: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "TOPIC" { return (false, Nothing); }

        if let Some(args) = cmd.as_nparams(1, 1) {
            // first, find the chan
            let channels_handle = srv.channels.read();
            let chan_handle = match channels_handle.chan_handle(args[0].as_slice()) {
                Some(h) => {
                    // hide secret chans
                    if h.read().modes.get('s'.to_ascii())
                    || !user.membership(args[0].as_slice()).is_some() {
                        user.push_numreply(
                            numericreply::ERR_NOSUCHNICK(args[0].as_slice()),
                            srv.settings.read().name.as_slice()
                        );
                        return (true, Nothing);
                    }
                    h
                },
                None => {
                    user.push_numreply(
                        numericreply::ERR_NOSUCHNICK(args[0].as_slice()),
                        srv.settings.read().name.as_slice()
                    );
                    return (true, Nothing);
                }
            };

            if args.len() == 1 {
                // we are only reading the topic
                send_topic_to_user(user, chan_handle.read().topic.as_slice(),
                                   args[0].as_slice(), srv);
            } else {
                // it's an attempt to modify it.
                let can_modify = if let Some(m) = user.membership(args[0].as_slice()) {
                    if !m.channel.upgrade().unwrap().read().modes.get('t'.to_ascii())
                    || m.modes.read().get('o'.to_ascii()) {
                        true
                    } else {
                        user.push_numreply(
                            numericreply::ERR_CHANOPRIVSNEEDED(args[0].as_slice()),
                            srv.settings.read().name.as_slice()
                        );
                        false
                    }
                } else {
                    user.push_numreply(
                        numericreply::ERR_NOTONCHANNEL(args[0].as_slice()),
                        srv.settings.read().name.as_slice()
                    );
                    false
                };


                if can_modify {
                    chan_handle.write().topic = args[1].clone();
                    channels_handle.send_to_chan(
                        args[0].as_slice(),
                        IRCMessage {
                            prefix: Some(user.get_fullname()),
                            command: "TOPIC".to_string(),
                            args: vec!(args[0].clone()),
                            suffix: Some(args[1].clone())
                        },
                        None
                    );
                }
            }

        } else {
            send_needmoreparams(user, "TOPIC", srv);
        }
        (true, Nothing)
    }
}

pub fn send_topic_to_user(user: &UserData, topic: &str, channame: &str, srv: &ServerData) {
    if topic.len() == 0 {
        user.push_numreply(
            numericreply::RPL_NOTOPIC(channame),
            srv.settings.read().name.as_slice()
        );
    } else {
        user.push_numreply(
            numericreply::RPL_TOPIC(channame.as_slice(), topic.as_slice()),
            srv.settings.read().name.as_slice()
        );
    }
}