use crate::logger::Logger;
use chrono::{NaiveDate, TimeZone, Utc};
use log::{
    grpc::{
        logger_service_server::LoggerService, FollowResponse, GetRequest, GetResponse, LogRequest,
        LogResponse,
    },
    log::Log,
};
use std::{pin::Pin, sync::Arc};
use tokio::sync::Mutex;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

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

        let mut logger = self.logger.lock().await.log(log).await;

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
                .await
                .get(&date, None)
                .map(|logs| logs.iter().map(|log| log.to_grpc_log()).collect())
                .unwrap_or(Vec::new()),
        }))
    }

    type FollowStream =
        Pin<Box<dyn Stream<Item = Result<log::grpc::FollowResponse, tonic::Status>> + Send + Sync>>;

    async fn follow(
        &self,
        _: tonic::Request<log::grpc::FollowRequest>,
    ) -> Result<tonic::Response<Self::FollowStream>, tonic::Status> {
        let (sender, receiver) = tokio::sync::mpsc::channel(4);

        self.logger.lock().await.follow(sender);

        Ok(tonic::Response::new(Box::pin(
            ReceiverStream::new(receiver).map(|log| {
                Ok(FollowResponse {
                    log: Some(log.to_grpc_log()),
                })
            }),
        )))
    }
}
