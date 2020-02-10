use std::io::Read;
use std::io::Result as IoResult;
use std::time::Instant;

#[derive(Debug)]
pub struct Speedometer {
    warmup_size: u64,
    measure_size: u64,
}

const BUFFER_SIZE: usize = 8192;

impl Speedometer {
    pub fn new(warmup_size: u64, measure_size: u64) -> Speedometer {
        Speedometer {
            warmup_size,
            measure_size,
        }
    }

    pub fn measure(&self, read: &mut dyn Read) -> IoResult<f64> {
        let mut buffer = [0; BUFFER_SIZE];

        self.read_bytes(read, &mut buffer, self.warmup_size)?;

        let now = Instant::now();

        self.read_bytes(read, &mut buffer, self.measure_size)?;

        let duration = 1.0e-9 * now.elapsed().as_nanos() as f64;
        let speed = self.measure_size as f64 / duration;

        Ok(speed)
    }

    fn read_bytes(&self, read: &mut dyn Read, buffer: &mut [u8], size: u64) -> IoResult<()> {
        let mut bytes_remain = size;

        loop {
            let length = buffer.len().min(bytes_remain as usize);

            match read.read(&mut buffer[0..length])? {
                n if n > 0 => bytes_remain -= n as u64,
                0 => break,
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}
