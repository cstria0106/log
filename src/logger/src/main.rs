pub mod console_logger;
pub mod log;
pub mod logger;
pub mod s3_logger;

use std::io::stdin;

use logger::Logger;
use s3_logger::S3Logger;

use crate::log::Log;
fn main() {
    let mut logger = S3Logger::new();

    for _ in 0..40960 {
        logger.log(Log::from_string(&"Hello, world!".to_string()));
    } 

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
}
