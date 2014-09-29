//! metallircd

extern crate irccp;
extern crate uuid;

use std::sync::{Arc, RWLock};
use std::sync::deque;
use std::sync::mpsc_queue::Queue as MPSCQueue;

use uuid::Uuid;

pub mod users;
pub mod scheduling;

fn main() {

    //
    // SHARED DATA
    //
    let shutdown = Arc::new(RWLock::new(false));
    let user_manager = Arc::new(RWLock::new(users::UserManager::new()));
    let user_torecycle_queue: Arc<MPSCQueue<Uuid>> = Arc::new(MPSCQueue::new());

    let user_recycled_buffer: deque::BufferPool<Uuid> = deque::BufferPool::new();
    let (user_recycled_worker, user_recycled_stealer) = user_recycled_buffer.deque();

    //
    // THREADS
    //

    // client handlers
    let client_handlers = scheduling::spawn_clients_handlers(2u, &shutdown, &user_manager,
                                                            &user_torecycle_queue,
                                                            &user_recycled_stealer);

    // clients recycler
    let client_recycler = scheduling::spawn_clients_recycler(&shutdown, &user_manager,
                                                             &user_torecycle_queue,
                                                             user_recycled_worker);

    //
    // CLEANUP
    //
    for it in client_handlers.into_iter() {
        let _ = it.unwrap();
    }

    let _ = client_recycler.unwrap();

}
