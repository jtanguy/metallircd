//! Scheduling operations.

#![experimental]

use channels::ChannelManager;
use logging::{Logger, Info};
use conf::ServerConf;
use users::UserManager;
use modules::{ModulesHandler, RecyclingAction};

use std::io::net::tcp::TcpAcceptor;
use std::sync::mpsc_queue::Queue as MPSCQueue;
use std::sync::{Arc, deque, RWLock};

use uuid::Uuid;

mod users_handling;
mod procs;

#[experimental]
pub struct ServerData {
    pub settings: RWLock<ServerConf>,
    pub users: RWLock<UserManager>,
    pub channels: RWLock<ChannelManager>,

    pub logger: Logger,
    pub queue_users_torecycle: MPSCQueue<(Uuid, RecyclingAction)>,
    pub signal_shutdown: RWLock<bool>,

    pub modules_handler: RWLock<ModulesHandler>
}

#[experimental]
impl ServerData {

    pub fn new(settings: ServerConf)-> ServerData {
        let logger = Logger::new(settings.loglevel);
        let modules_hdlr = ModulesHandler::init(&settings, &logger);
        ServerData {
            settings: RWLock::new(settings),
            users: RWLock::new(UserManager::new()),
            channels: RWLock::new(ChannelManager::new()),
            logger: logger,
            queue_users_torecycle: MPSCQueue::new(),
            signal_shutdown: RWLock::new(false),
            modules_handler: RWLock::new(modules_hdlr)
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
    for i in range(1u, arc_srv.settings.read().thread_handler_count) {
        thread_handles.push(
            procs::spawn_clients_handler(arc_srv.clone(), user_recycled_stealer.clone(), i)
        );
    }

    arc_srv.logger.log(Info, format!("Initialised {} clients hanlders.",arc_srv.settings.read().thread_handler_count));

    // clients recycler
    thread_handles.push(
        procs::spawn_clients_recycler(arc_srv.clone(), user_recycled_worker)
    );

    // logger
    thread_handles.push(
        procs::spawn_logger(arc_srv.clone())
    );

    //
    // CLEANUP & JOINING
    //
    for it in thread_handles.into_iter() {
        let _ = it.unwrap();
    }
}