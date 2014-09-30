//! Setting moduleof the server.

#![experimental]

pub struct ServerSettings {
    // generic
    pub name: String,
    pub address: String,
    pub port: u16,

    // sleep times:
    pub tcp_timout: u64,
    pub thread_handler_count: uint,
    pub thread_sleep_time: i64,
    pub thread_new_users_cnx_timeout: u64,
}
