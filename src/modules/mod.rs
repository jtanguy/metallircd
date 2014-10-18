//! Base of modules system.

#![experimental]

use conf::ServerConf;
use logging::Debug;
use messages::{IRCMessage, TextMessage, numericreply};
use scheduling::ServerData;
use users::UserData;

use uuid::Uuid;

mod core_textmessages;
mod core_commands;
mod core_channels;

/// Special actions to be performed by the recycler thread
#[experimental]
#[deriving(PartialEq)]
pub enum RecyclingAction {
    /// Nothing to do
    Nothing,
    /// a nick change is requested
    ChangeNick(String),
    /// the user should be zombified
    Zombify
}

/// A trait for modules handling commands.
#[experimental]
pub trait CommandHandler : Send + Sync {
    /// Tries to handle the command. Returns true if the command was matched and handled,
    /// and false if this handler don't handle this command.
    #[experimental]
    fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: &IRCMessage, srv: &ServerData)
        -> (bool, RecyclingAction);
}

/// A trait for modules altering the sending of a text message from a user.
#[experimental]
pub trait MessageSendingHandler : Send + Sync {
    /// Does whatever needed with this message (can be nothing).
    /// If returning None, the message won't go futher.
    #[experimental]
    fn handle_message_sending(&self, msg: TextMessage, srv: &ServerData) -> Option<TextMessage>;
}

pub struct ModulesHandler {
    command_handlers: Vec<Box<CommandHandler + 'static + Send + Sync>>,
    message_sending_handlers: Vec<Box<MessageSendingHandler + 'static + Send + Sync>>
}

impl ModulesHandler {
    #[experimental]
    #[allow(unused_variable)] // conf might be used ?
    pub fn init(conf: &ServerConf) -> ModulesHandler {
        // Put the modules here for them to be loaded
        ModulesHandler {
            command_handlers: vec!(
                box core_commands::CmdPing as Box<CommandHandler + Send + Sync>,
                box core_textmessages::CmdPrivmsgOrNotice as Box<CommandHandler + Send + Sync>,
                box core_channels::CmdJoin as Box<CommandHandler + Send + Sync>,
                box core_channels::CmdPart as Box<CommandHandler + Send + Sync>,
                box core_channels::CmdNames as Box<CommandHandler + Send + Sync>,
                box core_commands::CmdNick as Box<CommandHandler + Send + Sync>,
                box core_commands::CmdQuit as Box<CommandHandler + Send + Sync>
            ),
            message_sending_handlers: vec!(
                box core_textmessages::QueryDispatcher as Box<MessageSendingHandler + Send + Sync>,
                box core_textmessages::ChannelDispatcher as Box<MessageSendingHandler + Send + Sync>
            )
        }
    }

    /// Tries all the command handlers in order, stopping as soon as one successfully handles the command.
    /// Returns false if no appropriate handler was found.
    #[experimental]
    pub fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: IRCMessage, srv: &ServerData)
        -> RecyclingAction {
        for handler in self.command_handlers.iter() {
            let (done, action) = handler.handle_command(user, user_uuid, &cmd, srv);
            if done {
                return action;
            }
        }
        srv.logger.log(Debug, format!("Unknown command call {} by {}.", cmd.command, user.nickname));
        user.push_message(
            IRCMessage {
                prefix: Some(srv.settings.read().name.clone()),
                command: numericreply::ERR_UNKNOWNCOMMAND.to_text(),
                args: vec!(user.nickname.clone(), cmd.command),
                suffix: Some("Unknown Command.".to_string())
            }
        );
        Nothing
    }

    /// Sends a message by processing it through all the handlers in order until onrof them consumes it.
    #[experimental]
    pub fn send_message(&self, mut msg: TextMessage, srv: &ServerData) {
        for handler in self.message_sending_handlers.iter() {
            if let Some(m) = handler.handle_message_sending(msg, srv) {
                msg = m;
            } else {
                return;
            }
        }
        // if we reach this point, no handler consumed the message, we drop it.
    }
}

pub fn send_needmoreparams(u: &UserData, cmd: &str, srv: &ServerData) {
    u.push_message(
        IRCMessage {
            prefix: Some(srv.settings.read().name.clone()),
            command: numericreply::ERR_NEEDMOREPARAMS.to_text(),
            args: vec!(u.nickname.clone(), cmd.to_string()),
            suffix: Some("Not enough parameters.".to_string())
        }
    );
}