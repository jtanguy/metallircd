//! Basic task-creation functions

use std::any::Any;
use std::io::timer::sleep;
use std::rt::thread::Thread;
use std::sync::{Arc, RWLock, Future};
use std::sync::deque;
use std::sync::deque::{Stealer, Worker};
use std::sync::mpsc_queue::Queue as MPSCQueue;
use std::task::TaskBuilder;
use std::time::duration::Duration;

use uuid::Uuid;

use users;

pub fn spawn_clients_handlers(count: uint, shutdown: &Arc<RWLock<bool>>,
                              usermanager: &Arc<RWLock<users::UserManager>>,
                              torecycle: &Arc<MPSCQueue<Uuid>>,
                              next_user: &Stealer<Uuid>)
                              -> Vec<Future<Result<(), Box<Any + Send>>>> {

    let mut client_handlers = Vec::new();
    for i in range(0u, count) {
        client_handlers.push(
            TaskBuilder::new().named(format!("Client handler {}", i)).try_future({
                // first, get handles to what we will need
                let my_shutdown = shutdown.clone();
                let my_manager = usermanager.clone();
                let my_next_user = next_user.clone();
                let my_torecycle = torecycle.clone();
                // then, the proc
                proc() {
                    loop {
                        match my_next_user.steal() {
                            deque::Data(id) => {
                                if *my_shutdown.read() {
                                    users::disconnect_user(&id, &*my_manager.read());
                                } else {
                                    users::handle_user(&id, &*my_manager.read());
                                }
                                my_torecycle.push(id);
                            }
                            _ => if *my_shutdown.read() {
                                return;
                            } else {
                                // there is nothing to do, sleep
                                sleep(Duration::milliseconds(200));
                            }
                        }
                        Thread::yield_now();
                    }
                }
            })
        );
    }
    client_handlers

}


pub fn spawn_clients_recycler(shutdown: &Arc<RWLock<bool>>,
                              usermanager: &Arc<RWLock<users::UserManager>>,
                              torecycle: &Arc<MPSCQueue<Uuid>>,
                              recycled: Worker<Uuid>)
                              -> Future<Result<(), Box<Any + Send>>> {
    TaskBuilder::new().named("Client recycler").try_future({
        // first, get handles to what we will need
        let my_shutdown = shutdown.clone();
        let my_manager = usermanager.clone();
        let my_recycled = recycled;
        let my_torecycle = torecycle.clone();
        // then, the proc
        proc() {
            loop {
                match my_torecycle.casual_pop() {
                    Some(id) => {
                        if *my_shutdown.read() {
                            // just in case
                            users::disconnect_user(&id, &*my_manager.read());
                            // then delete
                            users::recycle_user(&id, &mut *my_manager.write());
                        } else {
                            // this user is disconnected, free it
                            if my_manager.read().get_user_by_uuid(&id).map_or(true, |u| u.zombie) {
                                users::recycle_user(&id, &mut *my_manager.write());
                            }
                            my_recycled.push(id);
                        }

                    }
                    None => if *my_shutdown.read() && (*my_manager.read()).is_empty() {
                        // cleanup is finished
                        return;
                    } else {
                        // there is nothing to do, sleep
                        sleep(Duration::milliseconds(200));
                    }
                }
                Thread::yield_now();
            }
        }
    })
}
