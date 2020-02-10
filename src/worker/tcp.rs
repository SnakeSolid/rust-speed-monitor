use crate::metric::Metric;
use crate::worker::error::WorkerError;
use crate::worker::error::WorkerResult;
use crate::worker::speed::Speedometer;
use std::net::Shutdown;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct TcpWorker {
    interval: Duration,
    speedometer: Speedometer,
    address: SocketAddr,
    metric: Metric,
}

impl TcpWorker {
    pub fn new<A>(
        interval: u64,
        speedometer: Speedometer,
        address: A,
        metric: Metric,
    ) -> WorkerResult<TcpWorker>
    where
        A: ToSocketAddrs,
    {
        let address = address
            .to_socket_addrs()
            .map_err(WorkerError::socket_address_error)?
            .next()
            .ok_or_else(WorkerError::empty_socket_address)?;

        Ok(TcpWorker {
            interval: Duration::from_secs(interval),
            speedometer,
            address,
            metric,
        })
    }

    pub fn run(self) {
        loop {
            match self.measure() {
                Ok(()) => {}
                Err(err) => error!(
                    "File measurement error: address = {} - {}",
                    self.address, err
                ),
            }

            thread::sleep(self.interval);
        }
    }

    fn measure(&self) -> WorkerResult<()> {
        let mut stream = TcpStream::connect(&self.address).map_err(WorkerError::io_error)?;

        stream
            .shutdown(Shutdown::Write)
            .map_err(WorkerError::io_error)?;

        let speed = self
            .speedometer
            .measure(&mut stream)
            .map_err(WorkerError::io_error)?;

        self.metric.write(speed).map_err(WorkerError::metric_error)
    }
}
