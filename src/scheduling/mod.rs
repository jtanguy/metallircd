//! Scheduling operations.

#![experimental]

use channels::ChannelManager;
use users::UserManager;
use settings::ServerSettings;

use std::io::net::tcp::TcpAcceptor;
use std::sync::mpsc_queue::Queue as MPSCQueue;
use std::sync::{Arc, deque, RWLock};

use uuid::Uuid;

mod input_handling;
mod users_handling;
mod procs;

#[experimental]
pub struct ServerData {
    pub settings: RWLock<ServerSettings>,
    pub users: RWLock<UserManager>,
    pub channels: RWLock<ChannelManager>,

    pub queue_users_torecycle: MPSCQueue<(Uuid, users_handling::RecyclingAction)>,

    pub signal_shutdown: RWLock<bool>
}

#[experimental]
impl ServerData {

    pub fn new(settings: ServerSettings)-> ServerData {
        ServerData {
            settings: RWLock::new(settings),
            users: RWLock::new(UserManager::new()),
            channels: RWLock::new(ChannelManager::new()),
            queue_users_torecycle: MPSCQueue::new(),
            signal_shutdown: RWLock::new(false)
        }
    }
}

pub fn run_server(srv: ServerData, acceptor: TcpAcceptor) {

    let arc_srv = Arc::new(srv);

    let user_recycled_buffer: deque::BufferPool<Uuid> = deque::BufferPool::new();
    let (user_recycled_worker, user_recycled_stealer) = user_recycled_buffer.deque();

    let mut thread_handles = Vec::new();

    // new clients handler
    thread_handles.push(
        procs::spawn_newclients_handler(arc_srv.clone(), acceptor)
    );

    // client handlers
    thread_handles.push(
        procs::spawn_clients_handler(arc_srv.clone(), user_recycled_stealer, 1u)
    );

    // clients recycler
    thread_handles.push(
        procs::spawn_clients_recycler(arc_srv.clone(), user_recycled_worker)
    );

    //
    // CLEANUP & JOINING
    //
    for it in thread_handles.into_iter() {
        let _ = it.unwrap();
    }
}