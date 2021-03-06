use crate::metric::MetricError;
use reqwest::Error as ReqwestError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;

pub type WorkerResult<T> = Result<T, WorkerError>;

#[derive(Debug)]
pub enum WorkerError {
    IoError { error: IoError },
    ReqwestError { error: ReqwestError },
    MetricError { error: MetricError },
    SocketAddressError { error: IoError },
    EmptySocketAddress {},
}

impl WorkerError {
    pub fn io_error(error: IoError) -> Self {
        warn!("IO error - {}", error);

        WorkerError::IoError { error }
    }

    pub fn http_error(error: ReqwestError) -> Self {
        warn!("HTTP error - {}", error);

        WorkerError::ReqwestError { error }
    }

    pub fn metric_error(error: MetricError) -> Self {
        warn!("Metric error - {}", error);

        WorkerError::MetricError { error }
    }

    pub fn socket_address_error(error: IoError) -> Self {
        warn!("Socket address error - {}", error);

        WorkerError::SocketAddressError { error }
    }
    pub fn empty_socket_address() -> Self {
        warn!("Empty socket address");

        WorkerError::EmptySocketAddress {}
    }
}

impl Error for WorkerError {}

impl Display for WorkerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            WorkerError::IoError { error } => write!(f, "{}", error),
            WorkerError::ReqwestError { error } => write!(f, "{}", error),
            WorkerError::MetricError { error } => write!(f, "{}", error),
            WorkerError::SocketAddressError { error } => write!(f, "{}", error),
            WorkerError::EmptySocketAddress {} => write!(f, "Empty socket address"),
        }
    }
}
