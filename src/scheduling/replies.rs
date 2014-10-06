//! Automation of big server replies.

#![experimental]

use super::ServerData;
use channels::modes;
use users::UserData;

use irccp::{numericreply, ToIRCMessage};

/// Sends a RPL_NAMREPLY with the users of the given chan to me
#[experimental]
pub fn send_names(me: &UserData, chan: &String, srv: &ServerData) {
	let names = srv.channels.read().member_list(chan);
	let msg = numericreply::RPL_NAMEREPLY.to_ircmessage()
					.with_prefix(srv.settings.read().name.as_slice()).ok().unwrap()
					.add_arg("=").ok().unwrap()
					.add_arg(chan.as_slice()).ok().unwrap();
	let mut buffer = String::new();
	for &(id, mode) in names.iter() {
		let mut nextnick = String::new();
		match mode {
			modes::mm_founder => nextnick.push('~'),
			modes::mm_op => nextnick.push('@'),
			modes::mm_halfop => nextnick.push('%'),
			modes::mm_voice => nextnick.push('+'),
			_ => {}
		}
		nextnick.push_str(srv.users.read().get_user_by_uuid(&id).unwrap().nickname.as_slice());
		if buffer.len() + nextnick.len() + 1 > msg.max_suffix_len() {
			me.push_message(
				msg.clone().with_suffix(buffer.as_slice()).ok().unwrap()
			);
			buffer = nextnick;
		} else {
			if buffer.len() == 0 {
				buffer = nextnick;
			} else {
				buffer.push(' ');
				buffer.push_str(nextnick.as_slice());
			}
		}
	}
	if buffer.len() > 0 {
		me.push_message(
				msg.with_suffix(buffer.as_slice()).ok().unwrap()
		);
	}
}