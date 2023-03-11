mod connection;
mod package;

pub mod core;
pub mod runtime;
pub mod tcp_server;
pub mod tokio_util;
pub mod shutdown;
pub mod options;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
