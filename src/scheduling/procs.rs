//! Basic task-creation functions

use std::any::Any;
use std::collections::DList;
use std::io::{Acceptor, BufferedStream};
use std::io::{File, Append, Write};
use std::io::net::tcp::TcpAcceptor;
use std::io::timer::sleep;
use std::rt::thread::Thread;
use std::sync::{Arc, Future};
use std::sync::deque;
use std::sync::deque::{Stealer, Worker};
use std::task::TaskBuilder;
use std::time::duration::Duration;

use messages::{numericreply, IRCMessage};
use modules;

use uuid::Uuid;

use logging::Debug;
use users;

use super::ServerData;
use super::users_handling::{handle_user, destroy_user, recycle_user, disconnect_user};

pub fn spawn_newclients_handler(srv: Arc<ServerData>,
                                mut acceptor: TcpAcceptor)
                                -> Future<Result<(), Box<Any + Send>>> {
    TaskBuilder::new().named("New Clients Handler").try_future({
        proc() {
            let mut inc_list = DList::new();
            loop {
                // There is no problem with brutally closing not-yet established connections.
                if *srv.signal_shutdown.read() {
                    let _ = acceptor.close_accept();
                    return
                }
                acceptor.set_timeout(Some(50));
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
                    u.step_negociate(&*srv.settings.read());
                    if u.is_ready() {
                        let mut manager_handle = srv.users.write();
                        match manager_handle.insert(u) {
                            Ok(id) => {
                                // user was successfully inserted
                                let my_user = manager_handle.get_user_by_uuid(&id).unwrap();
                                // welcome the new user
                                my_user.push_message(
                                    IRCMessage {
                                        prefix: Some(srv.settings.read().name.clone()),
                                        command: numericreply::RPL_WELCOME.to_text(),
                                        args: vec!(my_user.nickname.clone()),
                                        suffix: Some(format!("Welcome to metallirc IRC Network {}",
                                                    my_user.get_fullname().as_slice()))
                                    }
                                );
                                srv.logger.log(Debug,
                                    format!("New user {} with UUID {}.", my_user.get_fullname(), id));
                                srv.queue_users_torecycle.push((id, modules::Nothing));
                            },
                            Err(mut nu) => {
                                // nick was already in use !
                                nu.report_unavailable_nick(&*srv.settings.read());
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

pub fn spawn_clients_handler(srv: Arc<ServerData>, recycled_stealer: deque::Stealer<Uuid>, number: uint)
                              -> Future<Result<(), Box<Any + Send>>> {
    TaskBuilder::new().named(format!("Client handler {}", number)).try_future({
        // copy my data
        proc() {
            loop {
                match recycled_stealer.steal() {
                    deque::Data(id) => {
                        let action = handle_user(&id, &*srv);
                        srv.queue_users_torecycle.push((id, action));
                    }
                    _ => if *srv.signal_shutdown.read() {
                        return;
                    } else {
                        // there is nothing to do, sleep
                        sleep(Duration::milliseconds(50));
                    }
                }
                Thread::yield_now();
            }
        }
    })
}


pub fn spawn_clients_recycler(srv: Arc<ServerData>, recycled_worker: deque::Worker<Uuid>)
                              -> Future<Result<(), Box<Any + Send>>> {
    TaskBuilder::new().named("Client recycler").try_future({
        // the proc
        proc() {
            loop {
                match srv.queue_users_torecycle.casual_pop() {
                    Some((id, action)) => {
                        if *srv.signal_shutdown.read() {
                            // just in case
                            disconnect_user(&id, &*srv, "Server shutdown.");
                            // then delete
                            destroy_user(&id, &*srv);
                        } else {
                            // this user is disconnected, free it
                            let is_zombie = srv.users.read().get_user_by_uuid(&id).map_or(true, |u| u.is_zombie());
                            if is_zombie {
                                destroy_user(&id, &*srv);
                            } else if action == modules::Nothing {
                                recycled_worker.push(id);
                            } else {
                                recycle_user(&id, action, &*srv);
                                recycled_worker.push(id);
                            }
                        }

                    }
                    None => if *srv.signal_shutdown.read() && (*srv.users.read()).is_empty() {
                        // cleanup is finished
                        return;
                    } else {
                        // there is nothing to do, sleep
                        sleep(Duration::milliseconds(50));
                    }
                }
                Thread::yield_now();
            }
        }
    })
}

pub fn spawn_logger(srv: Arc<ServerData>) -> Future<Result<(), Box<Any + Send>>> {
TaskBuilder::new().named("Logger").try_future({
        // the proc
        proc() {
            let mut file = match File::open_mode(&srv.settings.read().logfile, Append, Write) {
                Ok(f) => f,
                Err(e) => {
                    println!("Unable to open log file {} : {}",
                                srv.settings.read().logfile.as_str().unwrap_or(""),
                                e);
                    return
                }
            };
            loop {
                while match srv.logger.pop() {
                    Some(line) => { let _ = file.write_line(line.as_slice()); true },
                    None => if *srv.signal_shutdown.read() { return; } else { false }
                } { /* empty loop body */}
                let _ = file.datasync();
                sleep(Duration::milliseconds(200));
                Thread::yield_now();
            }
        }
    })
}