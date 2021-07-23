use async_trait::async_trait;
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

#[async_trait]
impl Device for ConsoleDevice {
    /// Print log on console.
    async fn log(&mut self, log: &Log) {
        println!("{}", log.to_pretty_string(&self.highlighter));
    }

    /// Do nothing.
    async fn store(&mut self, _: &Vec<Log>) -> device::Result<Option<String>> {
        Ok(None)
    }

    /// Do nothing.
    async fn get(&self, _: &Date<Utc>, _: Option<&[Level]>) -> device::Result<Option<Vec<Log>>> {
        Ok(None)
    }
}
