#[macro_use]
extern crate log;

use rand::Rng;
use speed_monitor::options::DummyOptions;
use std::io::Result as IoResult;
use std::net::Shutdown;
use structopt::StructOpt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

const BUFFER_SIZE: usize = 8 * 1024 * 1024;

async fn send_data(mut stream: TcpStream) -> IoResult<()> {
    let mut rng = rand::thread_rng();
    let mut buffer: Vec<u8> = (0..BUFFER_SIZE).map(|_| 0).collect();
    let peer_addr = stream.peer_addr()?.to_string();

    stream.shutdown(Shutdown::Read)?;

    debug!("Sending data: peer_addr = {:?}", peer_addr);

    loop {
        rng.fill(buffer.as_mut_slice());

        match stream.write(&buffer).await {
            Ok(0) => {
                stream.shutdown(Shutdown::Write)?;

                break;
            }
            Ok(_) => {}
            Err(err) => {
                warn!("Fail to send data: {}", err);

                break;
            }
        }
    }

    debug!("Sending complete: peer_addr = {:?}", peer_addr);

    Ok(())
}

#[tokio::main]
async fn main() -> IoResult<()> {
    env_logger::init();

    let options = DummyOptions::from_args();
    let address = options.address();
    let port = options.port();
    let mut listener = TcpListener::bind((address, port)).await?;

    println!("Dummy TCP, listening on {}:{}", address, port);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                debug!("Connection from: addr = {:?}", addr);

                send_data(stream).await?;
            }
            Err(err) => warn!("Failed to accept connection: {}", err),
        }
    }
}
