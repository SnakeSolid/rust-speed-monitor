use crate::metric::Metric;
use crate::worker::error::WorkerError;
use crate::worker::error::WorkerResult;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct FileWorker {
    interval: Duration,
    warmup_size: u64,
    measure_size: u64,
    path: PathBuf,
    metric: Metric,
}

impl FileWorker {
    pub fn new<P>(
        interval: u64,
        warmup_size: u64,
        measure_size: u64,
        path: P,
        metric: Metric,
    ) -> FileWorker
    where
        P: AsRef<Path>,
    {
        FileWorker {
            interval: Duration::from_secs(interval),
            warmup_size,
            measure_size,
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
        let mut buffer = [0; 8192];
        let mut read = File::open(&self.path).map_err(WorkerError::io_error)?;

        self.read_bytes(&mut read, &mut buffer, self.warmup_size)?;

        let now = Instant::now();

        self.read_bytes(&mut read, &mut buffer, self.measure_size)?;

        let duration = 1.0e-9 * now.elapsed().as_nanos() as f64;
        let speed = self.measure_size as f64 / duration;

        self.metric.write(speed).map_err(WorkerError::metric_error)
    }

    fn read_bytes(&self, read: &mut dyn Read, buffer: &mut [u8], size: u64) -> WorkerResult<()> {
        let mut bytes_remain = size;

        loop {
            let length = buffer.len().min(bytes_remain as usize);

            match read
                .read(&mut buffer[0..length])
                .map_err(WorkerError::io_error)?
            {
                n if n > 0 => bytes_remain -= n as u64,
                0 => break,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}
