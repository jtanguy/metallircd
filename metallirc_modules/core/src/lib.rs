//! Core module : provides very basic commands.

#![feature(if_let, while_let, phase)]

#[phase(plugin)] extern crate metallirc;
extern crate metallirc;
extern crate time;
extern crate toml;
extern crate uuid;

use metallirc::modules::Module;
use metallirc::logging::Logger;

mod channels;
mod commands;
mod list;
mod misc;
mod modes;
mod oper;
mod textmessages;
mod topic;

#[no_mangle]
pub fn init(conf: &toml::TomlTable, logger: &Logger) -> Vec<Box<Module + 'static + Send + Sync>> {
    init_modules!(
        commands::CmdPing,
        textmessages::CmdPrivmsgOrNotice,
        channels::CmdJoin,
        channels::CmdPart,
        channels::CmdNames,
        modes::CmdMode,
        topic::CmdTopic,
        list::CmdList,
        oper::CmdOper::init(conf, logger),
        commands::CmdNick,
        commands::CmdQuit,
        misc::CmdTime,
        textmessages::QueryDispatcher,
        textmessages::ChannelDispatcher,
        oper::CmdDie
    )
}