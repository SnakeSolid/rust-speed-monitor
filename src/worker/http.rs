use crate::metric::Metric;
use crate::worker::error::WorkerError;
use crate::worker::error::WorkerResult;
use crate::worker::speed::Speedometer;
use reqwest::blocking::Client;
use reqwest::IntoUrl;
use reqwest::Url;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct HttpWorker {
    interval: Duration,
    speedometer: Speedometer,
    client: Client,
    url: Url,
    metric: Metric,
}

impl HttpWorker {
    pub fn new<U>(
        interval: u64,
        speedometer: Speedometer,
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
            speedometer,
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
        let speed = self
            .speedometer
            .measure(&mut response)
            .map_err(WorkerError::io_error)?;

        self.metric.write(speed).map_err(WorkerError::metric_error)
    }
}
