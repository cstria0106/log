use crate::{log::Log, logger::Logger};

pub struct ConsoleLogger {}

impl Logger for ConsoleLogger {
    fn log(&mut self, log: Log) {
        println!("{}", log.message());
    }
}
