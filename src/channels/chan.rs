//!Channels structure.

#![experimental]

use std::collections::HashMap;
use std::collections::hashmap::Keys;

use uuid::Uuid;

use super::modes;
use super::modes::MembershipMode;

/// A channel.
#[experimental]
pub struct Channel {
	topic: String,
	members: HashMap<Uuid, MembershipMode>
}

#[experimental]
impl Channel {

	pub fn new() -> Channel {
		Channel {
			topic: String::new(),
			members: HashMap::new()
		}
	}

	#[experimental]
	pub fn has_member(&self, user: &Uuid) -> bool {
		self.members.contains_key(user)
	}

	#[experimental]
	pub fn member_count(&self) -> uint {
		self.members.len()
	}

	#[experimental]
	pub fn is_empty(&self) -> bool {
		self.members.is_empty()
	}

	/// Adds the user to the chan. Does nothing if the user is already in.
	#[experimental]
	pub fn join(&mut self, user: Uuid) {
		if !self.members.contains_key(&user) {
			self.members.insert(user, modes::mm_none);
		}
	}

	/// Parts the user from the chan. Does nothing if the user was not in the chan.
	#[experimental]
	pub fn part(&mut self, user: &Uuid) {
		self.members.remove(user);
	}

	/// Applies the closure to all members.
	#[experimental]
	pub fn apply_to_members(&self, func: |_: &Uuid, _: &MembershipMode| -> ()) {
		for (u, m) in self.members.iter() { func(u, m); };
	}

	#[experimental]
	pub fn get_topic<'a>(&'a self) -> &'a str {
		self.topic.as_slice()
	}

	#[experimental]
	pub fn set_topic(&mut self, topic: String) {
		self.topic = topic
	}

	/// Lists all members with given mode
	#[experimental]
	pub fn members_being(&self, mode: MembershipMode) -> Vec<Uuid> {
		let mut v = Vec::new();
		for (u, m) in self.members.iter() {
			if m.contains(mode.clone()) { v.push(u.clone()); }
		}
		v
	}

	/// Lists all members with given mode or better
	#[experimental]
	pub fn members_at_least(&self, mode: MembershipMode) -> Vec<Uuid> {
		let mut v = Vec::new();
		for (u, m) in self.members.iter() {
			if m.is_at_least(&mode) { v.push(u.clone()); }
		}
		v
	}

	/// Lists all members with their best mode
	#[experimental]
	pub fn member_list(&self) -> Vec<(Uuid, MembershipMode)> {
		self.members.iter().map(|(u, m)| {
			(u.clone(), m.best_mode())
		}).collect()
	}

	/// List all members as an iterator.
	pub fn members_iter(&self) -> Keys<Uuid, MembershipMode> {
		self.members.keys()
	}

}