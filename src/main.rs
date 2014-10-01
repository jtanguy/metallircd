//! metallircd

extern crate irccp;
extern crate uuid;

use std::io::Listener;
use std::io::net::tcp::TcpListener;
use std::sync::{Arc, RWLock};
use std::sync::deque;
use std::sync::mpsc_queue::Queue as MPSCQueue;

use uuid::Uuid;

pub mod scheduling;
pub mod settings;
pub mod users;
pub mod util;

fn main() {
    //
    // CONFIG
    //
    let serverconfig = Arc::new(settings::ServerSettings {
        name: "irc@foo.bar".to_string(),
        address: "127.0.0.1".to_string(),
        port: 6667,
        //
        tcp_timout: 50,
        thread_handler_count: 2,
        thread_new_users_cnx_timeout: 100,
        thread_sleep_time: 100
    });

    //
    // SHARED DATA
    //
    let shutdown = Arc::new(RWLock::new(false));
    let user_manager = Arc::new(RWLock::new(users::UserManager::new()));
    let user_torecycle_queue: Arc<MPSCQueue<(Uuid, users::RecyclingAction)>> = Arc::new(MPSCQueue::new());

    let user_recycled_buffer: deque::BufferPool<Uuid> = deque::BufferPool::new();
    let (user_recycled_worker, user_recycled_stealer) = user_recycled_buffer.deque();

    //
    // THREADS
    //

    // new clients handler
    let listener = match TcpListener::bind(serverconfig.address.as_slice(), serverconfig.port) {
        Ok(l) => l,
        Err(e) => fail!("Could not bind port: {}", e)
    };
    let acceptor = match listener.listen() {
        Ok(a) => a,
        Err(e) => fail!("Could not bind port: {}", e)
    };
    let new_clients_handler = scheduling::spawn_newclients_handler(&serverconfig, acceptor,
                                                                   &shutdown,
                                                                   &user_manager,
                                                                   &user_torecycle_queue);

    // client handlers
    let client_handlers = scheduling::spawn_clients_handlers(&serverconfig, &shutdown,
                                                            &user_manager,
                                                            &user_torecycle_queue,
                                                            &user_recycled_stealer);

    // clients recycler
    let client_recycler = scheduling::spawn_clients_recycler(&serverconfig, &shutdown,
                                                             &user_manager,
                                                             &user_torecycle_queue,
                                                             user_recycled_worker);

    //
    // CLEANUP & JOINING
    //
    let _ = new_clients_handler.unwrap();

    for it in client_handlers.into_iter() {
        let _ = it.unwrap();
    }

    let _ = client_recycler.unwrap();

}
