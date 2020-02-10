use crate::metric::MetricError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;

pub type WorkerResult<T> = Result<T, WorkerError>;

#[derive(Debug)]
pub enum WorkerError {
    IoError { error: IoError },
    MetricError { error: MetricError },
}

impl WorkerError {
    pub fn io_error(error: IoError) -> Self {
        warn!("IO error - {}", error);

        WorkerError::IoError { error }
    }

    pub fn metric_error(error: MetricError) -> Self {
        warn!("Metric error - {}", error);

        WorkerError::MetricError { error }
    }
}

impl Error for WorkerError {}

impl Display for WorkerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            WorkerError::IoError { error } => write!(f, "{}", error),
            WorkerError::MetricError { error } => write!(f, "{}", error),
        }
    }
}
