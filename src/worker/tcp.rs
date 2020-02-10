use crate::metric::Metric;
use crate::worker::error::WorkerError;
use crate::worker::error::WorkerResult;
use std::io::Read;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct TcpWorker {
    interval: Duration,
    warmup_size: u64,
    measure_size: u64,
    address: SocketAddr,
    metric: Metric,
}

impl TcpWorker {
    pub fn new<A>(
        interval: u64,
        warmup_size: u64,
        measure_size: u64,
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
            warmup_size,
            measure_size,
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
        let mut buffer = [0; 8192];
        let mut read = TcpStream::connect(&self.address).map_err(WorkerError::io_error)?;

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
