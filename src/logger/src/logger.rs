use crate::log::Log;

pub trait Logger {
    fn log(&mut self, log: Log);
}
