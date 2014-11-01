//! Operation and data structures handling user list

#![experimental]

pub use self::user::{UserData, PrivateUserDataHandler};
pub use self::usermanager::UserManager;
pub use self::newuser::NewUser;

mod newuser;
mod user;
mod usermanager;
