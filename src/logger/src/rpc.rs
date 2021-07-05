use crate::grpc::{logger_service_server::LoggerService, LogRequest, LogResponse};
use crate::log::Log;
use crate::logger::Logger;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct MyLoggerService<T>
where
    T: Logger + Send + 'static,
{
    logger: Arc<Mutex<T>>,
}

impl<T> MyLoggerService<T>
where
    T: Logger + Send + 'static,
{
    pub fn new(logger: T) -> Self {
        MyLoggerService {
            logger: Arc::new(Mutex::new(logger)),
        }
    }
}

#[tonic::async_trait]
impl<T> LoggerService for MyLoggerService<T>
where
    T: Logger + Send + 'static,
{
    async fn log(
        &self,
        request: tonic::Request<LogRequest>,
    ) -> Result<tonic::Response<LogResponse>, tonic::Status> {
        let mut logger = self.logger.lock().unwrap();
        logger.log(Log::from_string(&request.get_ref().message));

        Ok(tonic::Response::new(LogResponse::default()))
    }
}
