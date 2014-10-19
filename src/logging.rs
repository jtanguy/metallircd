//! Logging module.

#![experimental]

use std::sync::mpsc_queue::Queue as MPSCQueue;

use time::now;

/// Various available loglevels.
#[experimental]
#[deriving(Show)]
pub enum LogLevel {
    Error   = 0,
    Warning = 1,
    Info    = 2,
    Debug   = 3
}

/// The logger struct. To be shared by all threads of the server.
#[experimental]
pub struct Logger {
    queue: MPSCQueue<String>,
    pub level: LogLevel
}

#[experimental]
impl Logger {

    /// Creates a new logger, to be shared betweens tasks.
    #[experimental]
    pub fn new(level: LogLevel) -> Logger {
        let logger = Logger {
            queue: MPSCQueue::new(),
            level: level
        };
        logger.log(Info, format!("Initialised logging with level {}", level));
        logger
    }

    /// Adds a new entry to the logs.
    #[experimental]
    pub fn log(&self, level: LogLevel, text: String) {
        if level as u8 <= self.level as u8 {
            let t = now();
            self.queue.push(format!("[{}] {}: {}", t.strftime("%d/%b/%Y:%H:%M:%S %z").unwrap(), level, text));
        }
    }

    /// Pops the next message, to be used only by the thread handling log writing.
    #[experimental]
    pub fn pop(&self) -> Option<String> {
        self.queue.casual_pop()
    }
}
