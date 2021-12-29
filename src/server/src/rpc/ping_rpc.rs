use log::proto::{ping_service_server::PingService, PingRequest, PingResponse};

pub struct MyPingService {}

#[tonic::async_trait]
impl PingService for MyPingService {
    async fn ping(
        &self,
        request: tonic::Request<PingRequest>,
    ) -> Result<tonic::Response<PingResponse>, tonic::Status> {
        if request.get_ref().message.to_lowercase() == "ping" {
            Ok(tonic::Response::new(PingResponse {
                message: "Pong!".to_string(),
            }))
        } else {
            Err(tonic::Status::invalid_argument("Message must be \"ping\""))
        }
    }
}
