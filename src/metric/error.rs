use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;

pub type MetricResult<T> = Result<T, MetricError>;

#[derive(Debug)]
pub enum MetricError {
    WriteError {
        name: String,
        path: PathBuf,
        error: IoError,
    },
}

impl MetricError {
    pub fn write_error<P>(name: &str, path: P, error: IoError) -> Self
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        warn!(
            "Metric write error: name = {}, path = {} - {}",
            name,
            path.display(),
            error
        );

        MetricError::WriteError {
            name: name.into(),
            path: path.into(),
            error,
        }
    }
}

impl Error for MetricError {}

impl Display for MetricError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            MetricError::WriteError { name, path, error } => write!(
                f,
                "Metric write error: name = {}, path = {} - {}",
                name,
                path.display(),
                error
            ),
        }
    }
}
