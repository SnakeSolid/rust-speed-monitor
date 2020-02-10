use crate::config::Config;
use crate::handler::util::handle_request;
use iron::middleware::Handler;
use iron::IronResult;
use iron::Request as IronRequest;
use iron::Response as IronResponse;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DataHandler {
    storage_root: PathBuf,
}

impl DataHandler {
    pub fn new(config: &Config) -> DataHandler {
        let storage_root = config.storage_root().into();

        DataHandler { storage_root }
    }
}

impl Handler for DataHandler {
    fn handle(&self, request: &mut IronRequest) -> IronResult<IronResponse> {
        handle_request(request, move |_request: Request| {
            let result: Vec<usize> = Vec::new();

            Ok(result)
        })
    }
}

#[derive(Debug, Deserialize)]
struct Request {
    metric: String,
}
