use crate::logger::Logger;
use chrono::{NaiveDate, TimeZone, Utc};
use log::{
    grpc::{
        logger_service_server::LoggerService, GetRequest, GetResponse, LogRequest, LogResponse,
    },
    log::Log,
};
use std::sync::{Arc, Mutex};

pub struct MyLoggerService {
    logger: Arc<Mutex<Logger>>,
}

impl MyLoggerService {
    pub fn new(logger: Logger) -> Self {
        MyLoggerService {
            logger: Arc::new(Mutex::new(logger)),
        }
    }
}

#[tonic::async_trait]
impl LoggerService for MyLoggerService {
    async fn log(
        &self,
        request: tonic::Request<LogRequest>,
    ) -> Result<tonic::Response<LogResponse>, tonic::Status> {
        let request = request.get_ref();

        let log = if let Some(log) = &request.log {
            log
        } else {
            return Err(tonic::Status::invalid_argument("log required"));
        };

        let log =
            Log::from_grpc_log(log).map_err(|e| tonic::Status::invalid_argument("bad format"))?;

        self.logger
            .lock()
            .map_err(|_| tonic::Status::internal("unknown error"))?
            .log(log);

        Ok(tonic::Response::new(LogResponse::default()))
    }

    async fn get(
        &self,
        request: tonic::Request<GetRequest>,
    ) -> Result<tonic::Response<GetResponse>, tonic::Status> {
        let request = request.get_ref();

        let date = NaiveDate::parse_from_str(&request.date, "%F")
            .map_err(|_| tonic::Status::invalid_argument("bad format"))?;

        let date = if let chrono::LocalResult::Single(date) = Utc.from_local_date(&date) {
            date
        } else {
            return Err(tonic::Status::invalid_argument("bad format"));
        };

        Ok(tonic::Response::new(GetResponse {
            logs: self
                .logger
                .lock()
                .map_err(|_| tonic::Status::internal("unknown error"))?
                .get(&date, None)
                .map(|logs| logs.iter().map(|log| log.to_grpc_log()).collect())
                .unwrap_or(Vec::new()),
        }))
    }
}
