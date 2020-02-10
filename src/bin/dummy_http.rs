#[macro_use]
extern crate log;

use iron::mime::Mime;
use iron::mime::SubLevel;
use iron::mime::TopLevel;
use iron::response::WriteBody;
use iron::status;
use iron::Iron;
use iron::IronResult;
use iron::Request;
use iron::Response;
use rand::Rng;
use speed_monitor::options::DummyOptions;
use std::io::Result as IoResult;
use std::io::Write;
use structopt::StructOpt;

const BUFFER_SIZE: usize = 8 * 1024 * 1024;

#[derive(Debug)]
struct RandomBody;

impl WriteBody for RandomBody {
    fn write_body(&mut self, res: &mut dyn Write) -> IoResult<()> {
        let mut rng = rand::thread_rng();
        let mut buffer: Vec<u8> = (0..BUFFER_SIZE).map(|_| 0).collect();

        loop {
            rng.fill(buffer.as_mut_slice());

            match res.write(&buffer) {
                Ok(0) => {
                    break;
                }
                Ok(_) => {}
                Err(err) => {
                    warn!("Fail to send data: {}", err);

                    break;
                }
            }
        }

        debug!("Sending complete");

        Ok(())
    }
}

fn hello_world(request: &mut Request) -> IronResult<Response> {
    debug!(
        "Accepted request: local_addr = {:?}, remote_addr = {:?}",
        request.local_addr.to_string(),
        request.remote_addr.to_string()
    );

    let mime = Mime(TopLevel::Application, SubLevel::Star, Vec::default());
    let body: Box<dyn WriteBody> = Box::new(RandomBody);

    Ok(Response::with((status::Ok, mime, body)))
}

fn main() {
    env_logger::init();

    let options = DummyOptions::from_args();
    let address = options.address();
    let port = options.port();

    println!("Dummy HTTP, listening on {}:{}", address, port);

    if let Err(err) = Iron::new(hello_world).http((address, port)) {
        error!("Failed to start server: {}", err);
    }
}
