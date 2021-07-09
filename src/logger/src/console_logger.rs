use toml::Highlighter;

use crate::{log::Log, logger::Logger};

pub struct ConsoleLogger {
    highlighter: Highlighter,
}

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger {
            highlighter: Highlighter::new(),
        }
    }
}

impl Logger for ConsoleLogger {
    fn log(&mut self, log: Log) {
        println!("{}", log.to_pretty_string(&self.highlighter));
    }
}
