mod error;
mod file;
mod http;
mod tcp;

pub use error::WorkerError;
pub use error::WorkerResult;

use crate::metric::Metric;
use file::FileWorker;
use http::HttpWorker;
use reqwest::IntoUrl;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::thread::Builder;
use tcp::TcpWorker;

pub fn start_file_worker<P>(
    interval: u64,
    warmup_size: u64,
    measure_size: u64,
    path: P,
    metric: Metric,
) -> WorkerResult<()>
where
    P: AsRef<Path>,
{
    let worker = FileWorker::new(interval, warmup_size, measure_size, path, metric);

    Builder::new()
        .spawn(move || worker.run())
        .map(|_| ())
        .map_err(WorkerError::io_error)
}

pub fn start_http_worker<U>(
    interval: u64,
    warmup_size: u64,
    measure_size: u64,
    url: U,
    metric: Metric,
) -> WorkerResult<()>
where
    U: IntoUrl,
{
    let worker = HttpWorker::new(interval, warmup_size, measure_size, url, metric)?;

    Builder::new()
        .spawn(move || worker.run())
        .map(|_| ())
        .map_err(WorkerError::io_error)
}

pub fn start_tcp_worker<A>(
    interval: u64,
    warmup_size: u64,
    measure_size: u64,
    address: A,
    metric: Metric,
) -> WorkerResult<()>
where
    A: ToSocketAddrs,
{
    let worker = TcpWorker::new(interval, warmup_size, measure_size, address, metric)?;

    Builder::new()
        .spawn(move || worker.run())
        .map(|_| ())
        .map_err(WorkerError::io_error)
}
