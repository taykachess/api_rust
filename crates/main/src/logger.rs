use std::io::Write;
use std::sync::Mutex;

use log::{Level, Metadata, Record};

struct SimpleLogger {
    sender: Mutex<std::sync::mpsc::Sender<String>>,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let sender = self.sender.lock().expect("failed to lock");
            sender
                .send(format!("{} - {} \n", record.level(), record.args()))
                .expect("Can not send message");
        }
    }

    fn flush(&self) {}
}

use log::{LevelFilter, SetLoggerError};

// static LOGGER: SimpleLogger = SimpleLogger;

pub(crate) fn init() -> Result<(), SetLoggerError> {
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    std::thread::spawn(move || {
        let path = "log.txt";
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .expect("Sorry, no logs today");

        while let Ok(msg) = rx.recv() {
            file.write_all(msg.as_bytes())
                .expect("Sorry, no logs today");
        }
    });

    let logger = Box::new(SimpleLogger {
        sender: Mutex::new(tx),
    });

    log::set_logger(Box::leak(logger)).map(|()| log::set_max_level(LevelFilter::Info))
}
