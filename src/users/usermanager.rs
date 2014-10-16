//! The UserManager. Provides the abstraction of a container for all UserData.

#![experimental]

use std::collections::HashMap;

use uuid::Uuid;

use super::newuser::NewUser;
use super::user::UserData;
use util;

/// The container for all users of the server.
#[experimental]
pub struct UserManager {
    users: HashMap<Uuid, UserData>,
    nicks: HashMap<String, Uuid>,
}

#[experimental]
impl UserManager {

    /// Creates a new UserManager.
    #[experimental]
    pub fn new() -> UserManager {
        UserManager {
            users: HashMap::new(),
            nicks: HashMap::new()
        }
    }

    /// Inserts a new user in this manager.
    /// Returns the new Uuid on success, returns the NewUser on failure
    /// (if a field was missing or the nickname already used).
    #[experimental]
    pub fn insert(&mut self, user: NewUser) -> Result<Uuid, NewUser> {
        if user.nickname.is_none() || user.username.is_none() || user.username.is_none() {
            Err(user)
        } else {
            let lower_nick = util::label_to_lower(user.nickname.as_ref().unwrap().as_slice());
            if self.nicks.contains_key(&lower_nick) {
                Err(user)
            } else { // all is ok
                // TODO auto-detect host
                let full_user = UserData::new(user.socket,
                                              user.nickname.unwrap(),
                                              user.username.unwrap(),
                                              "metallirc".to_string(),
                                              user.realname.unwrap());

                let mut id = Uuid::new_v4();
                // better safe than sorry ?
                while self.users.contains_key(&id) { id = Uuid::new_v4(); }

                self.users.insert(id.clone(), full_user);
                self.nicks.insert(lower_nick, id.clone());
                Ok(id)
            }
        }
    }

    #[experimental]
    pub fn get_user_by_uuid<'a>(&'a self, id: &Uuid) -> Option<&'a UserData> {
        self.users.find(id)
    }

    #[experimental]
    pub fn get_uuid_of_nickname(&self, nick: &String) -> Option<Uuid> {
        self.nicks.find(&util::label_to_lower(nick.as_slice())).map(|id| id.clone())
    }

    #[experimental]
    pub fn get_user_by_nickname<'a>(&'a self, nick: &String) -> Option<&'a UserData> {
        // we should *never* have a nick for a unexistent user
        // so .unwrap() should *never* fail
        self.nicks.find(&util::label_to_lower(nick.as_slice())).map(|id| self.users.find(id).unwrap())
    }

    /// Changes the nickname of given uuid.
    /// Fails if the uuid does not exists.
    /// Returns false and does nothing if the new nick was already in use.
    #[experimental]
    pub fn change_nick(&mut self, id: &Uuid, new_nick: &String) -> bool {
        let lower_new_nick = util::label_to_lower(new_nick.as_slice());
        if self.nicks.contains_key(&lower_new_nick) { return false }

        let user = self.users.get_mut(id);
        let old_nick = ::std::mem::replace(&mut user.nickname, new_nick.clone());
        let _ = self.nicks.remove(&util::label_to_lower(old_nick.as_slice()));
        self.nicks.insert(lower_new_nick, id.clone());
        true
    }

    #[experimental]
    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }

    #[experimental]
    pub fn del_user(&mut self, id: &Uuid) {
        match self.users.pop(id) {
            Some(user_data) => { self.nicks.pop(&user_data.nickname); }
            None => {}
        }
    }

    #[experimental]
    pub fn iterate_map(&self, func: |u: &UserData|) {
        for u in self.users.values() {
            func(u);
        }
    }

}
