mod error;

pub use error::MetricError;
pub use error::MetricResult;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use time::OffsetDateTime;

#[derive(Debug)]
pub struct Metric {
    name: String,
    path: PathBuf,
}

impl Metric {
    pub fn new<P>(name: &str, path: P) -> Metric
    where
        P: AsRef<Path>,
    {
        Metric {
            name: name.into(),
            path: path.as_ref().into(),
        }
    }

    pub fn write(&self, speed: f64) -> MetricResult<()> {
        debug!("Writing metric `{}` = {}", self.name, speed);

        let mut writer = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|error| MetricError::write_error(&self.name, &self.path, error))?;
        let now = OffsetDateTime::now();

        writer
            .write_fmt(format_args!("{};{}\n", now.timestamp(), speed))
            .map_err(|error| MetricError::write_error(&self.name, &self.path, error))
    }
}