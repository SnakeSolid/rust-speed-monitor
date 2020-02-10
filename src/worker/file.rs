use crate::metric::Metric;
use crate::worker::error::WorkerError;
use crate::worker::error::WorkerResult;
use crate::worker::speed::Speedometer;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct FileWorker {
    interval: Duration,
    speedometer: Speedometer,
    path: PathBuf,
    metric: Metric,
}

impl FileWorker {
    pub fn new<P>(interval: u64, speedometer: Speedometer, path: P, metric: Metric) -> FileWorker
    where
        P: AsRef<Path>,
    {
        FileWorker {
            interval: Duration::from_secs(interval),
            speedometer,
            path: path.as_ref().into(),
            metric,
        }
    }

    pub fn run(self) {
        loop {
            match self.measure() {
                Ok(()) => {}
                Err(err) => error!(
                    "File measurement error: path = {} - {}",
                    self.path.display(),
                    err
                ),
            }

            thread::sleep(self.interval);
        }
    }

    fn measure(&self) -> WorkerResult<()> {
        let mut file = File::open(&self.path).map_err(WorkerError::io_error)?;
        let speed = self
            .speedometer
            .measure(&mut file)
            .map_err(WorkerError::io_error)?;

        self.metric.write(speed).map_err(WorkerError::metric_error)
    }
}
