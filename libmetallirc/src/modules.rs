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

use logging::{Logger, Debug, Error, Info};
use messages::{IRCMessage, TextMessage, numericreply};
use ServerData;
use users::UserData;
use channels::Membership;

use uuid::Uuid;
use toml;

use std::ascii::Ascii;
use std::dynamic_lib::DynamicLibrary;
use std::slice::Items;

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
pub trait Module: 'static {
    // HACK(eddyb) Missing upcast to Any to make this clean.
    fn get_type_id(&self) -> TypeId { TypeId::of::<&'static Self>() }
    fn get_vtable_for_trait(&self, _trait_id: TypeId) -> Option<&'static ()> { None }
}

#[macro_export]
macro_rules! module {
    ($ty:ty is $($Trait:ty),+) => (
        impl ::metallirc::modules::Module for $ty {
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
pub trait ModuleRef<'a> {
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

#[macro_export]
macro_rules! init_modules {
    ($($m:expr),+) => {
        vec!($(box $m as Box<Module + Send + Sync>),+)
    }
}
//
// End of Dangerous Zone
//

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

/// A trait for modules handling user modes.
#[experimental]
pub trait UserModeHandler : Send + Sync {
    /// If can handle given mode, do it and return `Some(true)` if the
    /// transformation was allowed, `Some(false)` otherwise.
    /// If the mode is uwknown, return `None`.
    fn handle_usermode_request(&self, asker: &UserData, target: &UserData,
                               flag: Ascii, set: bool,
                               srv: &ServerData) -> Option<bool>;
}

/// A trait for modules handling channel modes.
#[experimental]
pub trait ChannelModeHandler : Send + Sync {
    /// If can handle given mode, do it and return `Some(true)` if the
    /// transformation was allowed, `Some(false)` otherwise.
    /// If the mode is uwknown, return `None`.
    fn handle_chanmode_request(&self, asker: &Membership,
                               flag: Ascii, set: bool,
                               args: &mut Items<String>,
                               srv: &ServerData) -> Option<bool>;
}

/// The modules handler.
///
/// It owns all modules instances and dispatches commands and messages to them.
pub struct ModulesHandler {
    libs: Vec<ModuleLib>
}

#[allow(dead_code)] // We reserve these attributes for future use
struct ModuleLib {
    name: String,
    modules: Vec<Box<Module + 'static + Send + Sync>>,
    lib: DynamicLibrary
}

impl ModulesHandler {
    /// This method initialises all modules. It should be edited to add new modules.
    #[experimental]
    pub fn init() -> ModulesHandler {
        // Put the modules here for them to be loaded
        ModulesHandler { libs: Vec::new() }
    }

    pub fn open_module(&mut self, name: &str, cfg: &toml::TomlTable, logger: &Logger) {
        let path = match cfg["path".to_string()] { 
            toml::String(ref path_str) => match from_str::<Path>(path_str.as_slice()) {
                Some(p) => p,
                None => {
                    logger.log(Error, format!("Invalid path for module {}.", name));
                    return;
                }
            },
            _ => {
                logger.log(Error, format!("Invalid path for module {}.", name));
                return;
            }
        };
        logger.log(Info, format!("Opening library {} for module {}.", path.display(), name));
        match DynamicLibrary::open(Some(path)) {
            Err(e) => logger.log(Error, e),
            Ok(lib) => {
                let handle = unsafe {
                    match lib.symbol("init") {
                            Err(e) => { logger.log(Error, e); return },
                            Ok(f) => {
                                ::std::mem::transmute::<*mut u8,
                                    fn(&toml::TomlTable, &Logger) ->
                                        Vec<Box<Module + 'static + Send + Sync>>
                                    >(f)
                            },
                    }
                };
                let new_modules = handle(cfg, logger);
                logger.log(Info, format!("Loaded {} new traitment units by module {}.",new_modules.len(), name));
                self.libs.push(
                    ModuleLib {
                        name: name.to_string(),
                        modules: new_modules,
                        lib: lib
                    }
                );
            }
        }
    }

    /// Tries all the command handlers in order, stopping as soon as one successfully handles the command.
    /// Returns false if no appropriate handler was found.
    #[experimental]
    pub fn handle_command(&self, user: &UserData, user_uuid: &Uuid, cmd: IRCMessage, srv: &ServerData)
        -> RecyclingAction {
        for l in self.libs.iter().rev() {
            for m in l.modules.iter() {
                if let Some(handler) = m.as_ref::<CommandHandler>() {
                    let (done, action) = handler.handle_command(user, user_uuid, &cmd, srv);
                    if done {
                        return action;
                    }
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
        for l in self.libs.iter().rev() {
            for m in l.modules.iter() {
                if let Some(handler) = m.as_ref::<MessageSendingHandler>() {
                    if let Some(m) = handler.handle_message_sending(msg, srv) {
                        msg = m;
                    } else {
                        return;
                    }
                }
            }
        }
        // if we reach this point, no handler consumed the message, we drop it.
    }

    /// Suggests the mode to all available handlers. Returns `Some(true)` if it was
    /// handled, `Some(false)` if it was refused, and `None` if it was unknown.
    #[experimental]
    pub fn handle_usermode(&self, asker: &UserData, target: &UserData,
                           flag: Ascii, set: bool,
                           srv: &ServerData) -> Option<bool> {
        for l in self.libs.iter().rev() {
            for m in l.modules.iter() {
                if let Some(handler) = m.as_ref::<UserModeHandler>() {
                    let ret = handler.handle_usermode_request(asker, target, flag, set, srv);
                    if ret.is_some() {
                        return ret;
                    }
                }
            }
        }
        // if we reach this point, the mode was not handled
        None
    }

    /// Suggests the mode to all available handlers. Returns `Some(true)` if it was
    /// handled, `Some(false)` if it was refused, and `None` if it was unknown.
    #[experimental]
    pub fn handle_channelmode(&self, membership: &Membership,
                              flag: Ascii, set: bool,
                              args: &mut Items<String>,
                              srv: &ServerData) -> Option<bool> {
        for l in self.libs.iter().rev() {
            for m in l.modules.iter() {
                if let Some(handler) = m.as_ref::<ChannelModeHandler>() {
                    let ret = handler.handle_chanmode_request(membership, flag, set, args, srv);
                    if ret.is_some() {
                        return ret;
                    }
                }
            }
        }
        // if we reach this point, the mode was not handled
        None
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
