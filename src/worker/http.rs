use crate::metric::Metric;
use crate::worker::error::WorkerError;
use crate::worker::error::WorkerResult;
use reqwest::blocking::Client;
use reqwest::IntoUrl;
use reqwest::Url;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct HttpWorker {
    interval: Duration,
    warmup_size: u64,
    measure_size: u64,
    client: Client,
    url: Url,
    metric: Metric,
}

impl HttpWorker {
    pub fn new<U>(
        interval: u64,
        warmup_size: u64,
        measure_size: u64,
        url: U,
        metric: Metric,
    ) -> WorkerResult<HttpWorker>
    where
        U: IntoUrl,
    {
        let url = url.into_url().map_err(WorkerError::http_error)?;
        let client = Client::builder().build().map_err(WorkerError::http_error)?;

        Ok(HttpWorker {
            interval: Duration::from_secs(interval),
            warmup_size,
            measure_size,
            client,
            url,
            metric,
        })
    }

    pub fn run(self) {
        loop {
            match self.measure() {
                Ok(()) => {}
                Err(err) => error!("HTTP measurement error: url = {} - {}", self.url, err),
            }

            thread::sleep(self.interval);
        }
    }

    fn measure(&self) -> WorkerResult<()> {
        let mut buffer = [0; 8192];
        let request = self
            .client
            .get(self.url.as_ref())
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(WorkerError::http_error)?;
        let mut response = self
            .client
            .execute(request)
            .map_err(WorkerError::http_error)?;

        self.read_bytes(&mut response, &mut buffer, self.warmup_size)?;

        let now = Instant::now();

        self.read_bytes(&mut response, &mut buffer, self.measure_size)?;

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
