use chrono::{Date, Utc};
use log::log::{Level, Log};
use toml_highlighter::Highlighter;

use crate::device::{self, Device};

pub struct ConsoleDevice {
    highlighter: Highlighter,
}

impl ConsoleDevice {
    pub fn new() -> Self {
        ConsoleDevice {
            highlighter: Highlighter::new(),
        }
    }
}

impl Device for ConsoleDevice {
    fn log(&mut self, log: &Log) {
        println!("{}", log.to_pretty_string(&self.highlighter));
    }

    fn store(&mut self, _: &Vec<Log>) -> device::Result<Option<String>> {
        Ok(None)
    }

    fn get(&self, _: &Date<Utc>, _: Option<&[Level]>) -> device::Result<Option<Vec<Log>>> {
        Ok(None)
    }
}
