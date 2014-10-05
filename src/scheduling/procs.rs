//! Basic task-creation functions

use std::any::Any;
use std::collections::DList;
use std::io::{Acceptor, BufferedStream};
use std::io::net::tcp::TcpAcceptor;
use std::io::timer::sleep;
use std::rt::thread::Thread;
use std::sync::{Arc, RWLock, Future};
use std::sync::deque;
use std::sync::deque::{Stealer, Worker};
use std::sync::mpsc_queue::Queue as MPSCQueue;
use std::task::TaskBuilder;
use std::time::duration::Duration;

use irccp::{numericreply, ToIRCMessage};

use uuid::Uuid;

use settings::ServerSettings;
use users;

use super::users_handling;
use super::users_handling::{RecyclingAction, handle_user, destroy_user, recycle_user, disconnect_user};

pub fn spawn_newclients_handler(serverconf: &Arc<ServerSettings>,
                                mut acceptor: TcpAcceptor,
                                shutdown: &Arc<RWLock<bool>>,
                                usermanager: &Arc<RWLock<users::UserManager>>,
                                torecycle: &Arc<MPSCQueue<(Uuid, RecyclingAction)>>)
                                -> Future<Result<(), Box<Any + Send>>> {
    TaskBuilder::new().named("New Clients Handler").try_future({
        // first, get handles to what we will need
        let my_serverconf = serverconf.clone();
        let my_shutdown = shutdown.clone();
        let my_manager = usermanager.clone();
        let my_torecycle = torecycle.clone();
        // then, the proc
        proc() {
            let mut inc_list = DList::new();
            loop {
                // There is no problem with brutally closing not-yet established connections.
                if *my_shutdown.read() { let _ = acceptor.close_accept(); return }
                acceptor.set_timeout(Some(my_serverconf.thread_new_users_cnx_timeout));
                match acceptor.accept() {
                    Ok(mut socket) => {
                        // prepare the new connection
                        socket.set_timeout(Some(0));
                        inc_list.push(users::NewUser::new(BufferedStream::new(socket)));
                    },
                    // TODO : handle errors other than timeout
                    Err(_) => {}
                }
                // loop once over the new connections
                // TODO : timeout for initial negociation
                let mut not_finished = DList::new();
                for mut u in inc_list.into_iter() {
                    u.step_negociate(&*my_serverconf);
                    if u.is_ready() {
                        let mut manager_handle = my_manager.write();
                        match manager_handle.insert(u) {
                            Ok(id) => {
                                // user was successfully inserted
                                let my_user = manager_handle.get_user_by_uuid(&id).unwrap();
                                // welcome the new user
                                my_user.push_message(
                                    numericreply::RPL_WELCOME.to_ircmessage()
                                        .with_prefix(my_serverconf.name.as_slice()).unwrap()
                                        .add_arg(my_user.nickname.as_slice()).ok().unwrap()
                                        .with_suffix(
                                            format!("Welcome to metallirc IRC Network {}",
                                                    my_user.get_fullname().as_slice()).as_slice()
                                        ).ok().unwrap()
                                );
                                println!("New user {} with UUID {}.", my_user.get_fullname(), id);
                                my_torecycle.push((id, users_handling::Nothing));
                            },
                            Err(mut nu) => {
                                // nick was already in use !
                                nu.report_unavailable_nick(&*my_serverconf);
                                not_finished.push(nu);
                            }
                        }
                    } else {
                        not_finished.push(u);
                    }
                }
                inc_list = not_finished;
            }
        }
    })
}

pub fn spawn_clients_handlers(serverconf: &Arc<ServerSettings>,
                              shutdown: &Arc<RWLock<bool>>,
                              usermanager: &Arc<RWLock<users::UserManager>>,
                              torecycle: &Arc<MPSCQueue<(Uuid, RecyclingAction)>>,
                              next_user: &Stealer<Uuid>)
                              -> Vec<Future<Result<(), Box<Any + Send>>>> {

    let mut client_handlers = Vec::new();
    for i in range(0u, serverconf.thread_handler_count) {
        client_handlers.push(
            TaskBuilder::new().named(format!("Client handler {}", i)).try_future({
                // first, get handles to what we will need
                let my_serverconf = serverconf.clone();
                let my_shutdown = shutdown.clone();
                let my_manager = usermanager.clone();
                let my_next_user = next_user.clone();
                let my_torecycle = torecycle.clone();
                // then, the proc
                proc() {
                    loop {
                        match my_next_user.steal() {
                            deque::Data(id) => {
                                let action = if *my_shutdown.read() {
                                    disconnect_user(&id, &*my_manager.read(), "Server shutdown.", &*my_serverconf);
                                    users_handling::Nothing
                                } else {
                                    handle_user(&id, &*my_manager.read(), &*my_serverconf)
                                };
                                my_torecycle.push((id, action));
                            }
                            _ => if *my_shutdown.read() {
                                return;
                            } else {
                                // there is nothing to do, sleep
                                sleep(Duration::milliseconds(my_serverconf.thread_sleep_time));
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


pub fn spawn_clients_recycler(serverconf: &Arc<ServerSettings>,
                              shutdown: &Arc<RWLock<bool>>,
                              usermanager: &Arc<RWLock<users::UserManager>>,
                              torecycle: &Arc<MPSCQueue<(Uuid, RecyclingAction)>>,
                              recycled: Worker<Uuid>)
                              -> Future<Result<(), Box<Any + Send>>> {
    TaskBuilder::new().named("Client recycler").try_future({
        // first, get handles to what we will need
        let my_serverconf = serverconf.clone();
        let my_shutdown = shutdown.clone();
        let my_manager = usermanager.clone();
        let my_recycled = recycled;
        let my_torecycle = torecycle.clone();
        // then, the proc
        proc() {
            loop {
                match my_torecycle.casual_pop() {
                    Some((id, action)) => {
                        if *my_shutdown.read() {
                            // just in case
                            disconnect_user(&id, &*my_manager.read(), "Server shutdown.", &*my_serverconf);
                            // then delete
                            destroy_user(&id, &mut *my_manager.write());
                        } else {
                            // this user is disconnected, free it
                            let is_zombie = my_manager.read().get_user_by_uuid(&id).map_or(true, |u| u.is_zombie());
                            if is_zombie {
                                destroy_user(&id, &mut *my_manager.write());
                            } else if action == users_handling::Nothing {
                                my_recycled.push(id);
                            } else {
                                recycle_user(&id, action, &mut *my_manager.write(), &*my_serverconf);
                                my_recycled.push(id);
                            }
                        }

                    }
                    None => if *my_shutdown.read() && (*my_manager.read()).is_empty() {
                        // cleanup is finished
                        return;
                    } else {
                        // there is nothing to do, sleep
                        sleep(Duration::milliseconds(my_serverconf.thread_sleep_time));
                    }
                }
                Thread::yield_now();
            }
        }
    })
}
