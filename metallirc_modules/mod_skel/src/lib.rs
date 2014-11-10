//! Away module

#![feature(if_let, phase)]

#[phase(plugin)] extern crate metallirc;
extern crate metallirc;
extern crate uuid;
extern crate toml;

use uuid::Uuid;

use metallirc::ServerData;
use metallirc::users::UserData;

use metallirc::modules::{RecyclingAction, Nothing};
use metallirc::modules::CommandHandler;

// Public init()
use metallirc::modules::Module;
use metallirc::logging::Logger;

pub struct ModSkel;

module!(ModSkel is CommandHandler)

impl CommandHandler for ModSkel {
    fn handle_command(&self, _: &UserData, _: &Uuid, cmd: &IRCMessage, _: &ServerData)
        -> (bool, RecyclingAction) {
        if cmd.command.as_slice() != "SKEL" { return (false, Nothing); }

        (true, Nothing)
    }
}

#[no_mangle]
pub fn init(_: &toml::TomlTable, _: &Logger) -> Vec<Box<Module + 'static + Send + Sync>> {
    init_modules!(
        ModSkel
    )
}