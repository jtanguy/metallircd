//! The UserManager. Provides the abstraction of a container for all UserData.

#![experimental]

use std::collections::HashMap;

use uuid::Uuid;

use super::user::UserData;

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
    #[experimental]
    pub fn insert(&mut self, user: UserData) -> Uuid {
        let nick = user.nickname.clone();

        let mut id = Uuid::new_v4();
        // better safe than sorry ?
        while self.users.contains_key(&id) { id = Uuid::new_v4(); }

        self.users.insert(id.clone(), user);
        self.nicks.insert(nick, id.clone());
        id
    }

    #[experimental]
    pub fn get_user_by_uuid<'a>(&'a self, id: &Uuid) -> Option<&'a UserData> {
        self.users.find(id)
    }

    #[experimental]
    pub fn get_uuid_of_nickname(&self, nick: &String) -> Option<Uuid> {
        self.nicks.find(nick).map(|id| id.clone())
    }

    #[experimental]
    pub fn get_user_by_nickname<'a>(&'a self, nick: &String) -> Option<&'a UserData> {
        // we should *never* have a nick for a unexistent user
        // so .unwrap() should *never* fail
        self.nicks.find(nick).map(|id| self.users.find(id).unwrap())
    }

    /// Changes the nickname of given uuid.
    /// Fails if the uuid does not exists.
    /// Returns false and does nothing if the new nick was already in use.
    #[experimental]
    pub fn change_nick(&mut self, id: Uuid, new_nick: String) -> bool {
        if self.nicks.contains_key(&new_nick) { return false }

        let user = self.users.get_mut(&id);
        let old_nick = ::std::mem::replace(&mut user.nickname, new_nick.clone());
        let _ = self.nicks.remove(&old_nick);
        self.nicks.insert(new_nick, id);
        true
    }

}
