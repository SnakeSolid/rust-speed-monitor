use crate::config::Config;
use crate::handler::util::handle_request;
use crate::handler::HandlerError;
use crate::metric;
use iron::middleware::Handler;
use iron::mime::Mime;
use iron::mime::SubLevel;
use iron::mime::TopLevel;
use iron::status::Status;
use iron::IronResult;
use iron::Request as IronRequest;
use iron::Response as IronResponse;
use std::fs::File;
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
        handle_request(request, move |request: Request| {
            let metric = request.metric;
            let path = metric::metric_path(&metric, &self.storage_root);

            if !path.exists() || !path.is_file() {
                return Err(HandlerError::new(&format!(
                    "Metric does not exists: metric = {}",
                    metric
                )));
            }

            let read = File::open(path).map_err(|_err| {
                HandlerError::new(&format!("Failed to read metric data: metric = {}", metric))
            })?;
            let content_type = Mime(TopLevel::Text, SubLevel::Plain, vec![]);

            Ok(IronResponse::with((Status::Ok, content_type, read)))
        })
    }
}

#[derive(Debug, Deserialize)]
struct Request {
    metric: String,
}
