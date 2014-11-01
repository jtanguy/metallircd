//! Core module : provides very basic commands.

#![feature(if_let, while_let, phase)]

#[phase(plugin)] extern crate metallirc;
extern crate metallirc;
extern crate toml;
extern crate uuid;

use metallirc::conf::ServerConf;
use metallirc::modules::Module;
use metallirc::logging::Logger;

mod channels;
mod commands;
mod list;
mod modes;
mod oper;
mod textmessages;
mod topic;

#[no_mangle]
pub fn init(conf: &ServerConf, logger: &Logger) -> Vec<Box<Module + 'static + Send + Sync>> {
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
        textmessages::QueryDispatcher,
        textmessages::ChannelDispatcher,
        oper::CmdDie
    )
}