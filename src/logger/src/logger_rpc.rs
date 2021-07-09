use crate::grpc::{
    log_request::Level, logger_service_server::LoggerService, LogRequest, LogResponse,
};
use crate::log::{Log, LogLevel};
use crate::logger::Logger;

use std::sync::{Arc, Mutex};

pub struct MyLoggerService {
    loggers: Arc<Mutex<Vec<Box<dyn Logger + Send>>>>,
}

impl MyLoggerService {
    pub fn new(loggers: Vec<Box<dyn Logger + Send>>) -> Self {
        MyLoggerService {
            loggers: Arc::new(Mutex::new(loggers)),
        }
    }
}

#[tonic::async_trait]
impl LoggerService for MyLoggerService {
    async fn log(
        &self,
        request: tonic::Request<LogRequest>,
    ) -> Result<tonic::Response<LogResponse>, tonic::Status> {
        for logger in self.loggers.lock().unwrap().iter_mut() {
            let request = request.get_ref();

            logger.as_mut().log(Log::new(
                match &request.level() {
                    Level::Info => LogLevel::Info,
                    Level::Warning => LogLevel::Warning,
                    Level::Error => LogLevel::Error,
                    Level::Debug => LogLevel::Debug,
                },
                &request.message,
                if request.other.len() > 0 {
                    Some(request.other.clone())
                } else {
                    None
                },
            ));
        }

        Ok(tonic::Response::new(LogResponse::default()))
    }
}
