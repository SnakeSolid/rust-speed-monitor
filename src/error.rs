use crate::config::ConfigError;
use crate::worker::WorkerError;
use iron::error::HttpError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type ApplicationResult = Result<(), ApplicationError>;

#[derive(Debug)]
pub enum ApplicationError {
    LoadConfigError { message: String },
    ConfigError { message: String },
    WorkerError { message: String },
    HttpError { message: String },
}

impl ApplicationError {
    pub fn load_config_error(error: ConfigError) -> ApplicationError {
        error!("Failed to load configuration - {}", error);

        ApplicationError::LoadConfigError {
            message: format!("{}", error),
        }
    }

    pub fn config_error(error: ConfigError) -> ApplicationError {
        error!("Invalid configuration - {}", error);

        ApplicationError::ConfigError {
            message: format!("{}", error),
        }
    }

    pub fn worker_error(error: WorkerError) -> ApplicationError {
        error!("Worker error - {}", error);

        ApplicationError::WorkerError {
            message: format!("{}", error),
        }
    }

    pub fn http_error(error: HttpError) -> ApplicationError {
        error!("HTTP error - {}", error);

        ApplicationError::HttpError {
            message: format!("{}", error),
        }
    }
}

impl Error for ApplicationError {}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ApplicationError::LoadConfigError { message } => write!(f, "{}", message),
            ApplicationError::ConfigError { message } => write!(f, "{}", message),
            ApplicationError::WorkerError { message } => write!(f, "{}", message),
            ApplicationError::HttpError { message } => write!(f, "{}", message),
        }
    }
}
