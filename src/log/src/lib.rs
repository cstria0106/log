pub mod log;
pub mod proto {
    tonic::include_proto!("logger");
    tonic::include_proto!("ping");
}
