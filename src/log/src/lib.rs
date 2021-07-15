pub mod log;
pub mod grpc {
    tonic::include_proto!("logger");
    tonic::include_proto!("ping");
}
