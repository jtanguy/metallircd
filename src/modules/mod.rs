//! Base of modules system.

//! Here is the machinery handling the module system.
//!
//! A module can implement two traits (even both):
//!
//! - `CommandHandler` if it handles one or more commands
//! - `MessageSendingHandler` if it affects the transmission of a message.
//!
//! Such traits must be declared using the `module!(..)` macro like this:
//!
//! ```
//! module!(MyModule is CommandHandler, MessageSendingHandler)
//! ```
//!
//! These are not *real* modules in the sense of they are not dynamically loaded,
//! but they provide a common generic interface for handling commands and messages
//! in an elegant way.

#![experimental]

use conf::ServerConf;
use logging::{Logger, Debug};
use messages::{IRCMessage, TextMessage, numericreply};
use scheduling::ServerData;
use users::UserData;

use uuid::Uuid;

//
//
// Dangerous Zone
//
// Here lies some pretty nasty magic needed for downcasting into their traits.
// This should probably not change util DST a completely finished in Rust language.
//

use std::intrinsics::TypeId;
use std::mem::{size_of, transmute};
use std::raw::TraitObject;

#[doc(hidden)]
trait Module: 'static {
    // HACK(eddyb) Missing upcast to Any to make this clean.
    fn get_type_id(&self) -> TypeId { TypeId::of::<&'static Self>() }
    fn get_vtable_for_trait(&self, _trait_id: TypeId) -> Option<&'static ()> { None }
}

macro_rules! module {
    ($ty:ty is $($Trait:ty),+) => (
        impl super::Module for $ty {
            fn get_vtable_for_trait(&self, trait_id: ::std::intrinsics::TypeId) -> Option<&'static ()> {
                $(if trait_id == ::std::intrinsics::TypeId::of::<&'static $Trait>() {
                    Some(unsafe {&*::std::mem::transmute::<&$Trait, ::std::raw::TraitObject>(self).vtable})
                })else+ else {
                    None
                }
            }
        }
    )
}

#[doc(hidden)]
trait ModuleRef<'a> {
    fn as_ref<Sized? T>(self) -> Option<&'a T>;
}

impl<'a> ModuleRef<'a> for &'a Module+'static {
    fn as_ref<Sized? T:'static>(self) -> Option<&'a T> {
        let type_id = TypeId::of::<&'static T>();
        unsafe {
            let obj = transmute::<_, TraitObject>(self);
            if size_of::<*const T>() == size_of::<uint>() {
                if self.get_type_id() == type_id {
                    Some(*transmute::<_, &&T>(&obj.data))
                } else {
                    None
                }
            } else {
                self.get_vtable_for_trait(type_id).map(|vtable| {
                    *transmute::<_, &&T>(&TraitObject {
                        data: obj.data,
                        vtable: vtable as *const _ as *mut _
                    })
                })
            }
        }
    }
}

macro_rules! init_modules {
    ($($m:expr),+) => {
        vec!($(box $m as Box<Module + Send + Sync>),+)
    }
}
//
// End of Dangerous Zone
//

// declare your submodules here
mod core_textmessages;
mod core_commands;
mod core_channels;
mod core_oper;

mod away;
mod modes;
mod topic;

/// Special actions to be performed by the recycler thread (requiring `&mut` access to the UserManager).
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

/// The modules handler.
///
/// It owns all modules instances and dispatches commands and messages to them.
pub struct ModulesHandler {
    modules: Vec<Box<Module + 'static + Send + Sync>>,
}

impl ModulesHandler {
    /// This method initialises all modules. It should be edited to add new modules.
    #[experimental]
    #[allow(unused_variable)] // conf might be used ?
    pub fn init(conf: &ServerConf, logger: &Logger) -> ModulesHandler {
        // Put the modules here for them to be loaded
        ModulesHandler {
            modules: init_modules!(
                core_commands::CmdPing,
                core_textmessages::CmdPrivmsgOrNotice,
                core_channels::CmdJoin,
                core_channels::CmdPart,
                core_channels::CmdNames,
                modes::CmdMode,
                away::ModAway::init(),
                topic::CmdTopic,
                core_oper::CmdOper::init(conf, logger),
                core_commands::CmdNick,
                core_commands::CmdQuit,
                core_textmessages::QueryDispatcher,
                core_textmessages::ChannelDispatcher,
                core_oper::CmdDie
            )
        }
    }

    /// Tries all the command handlers in order, stopping as soon as one successfully handles the command.
    /// Returns false if no appropriate handler was found.
    #[experimental]
    pub fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: IRCMessage, srv: &ServerData)
        -> RecyclingAction {
        for m in self.modules.iter() {
            if let Some(handler) = m.as_ref::<CommandHandler>() {
                let (done, action) = handler.handle_command(user, user_uuid, &cmd, srv);
                if done {
                    return action;
                }
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
        for m in self.modules.iter() {
            if let Some(handler) = m.as_ref::<MessageSendingHandler>() {
                if let Some(m) = handler.handle_message_sending(msg, srv) {
                    msg = m;
                } else {
                    return;
                }
            }
        }
        // if we reach this point, no handler consumed the message, we drop it.
    }
}

/// Shortcut command for modules: sends to the user a "Not enought parameters" message
/// associated with given command.
#[experimental]
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
