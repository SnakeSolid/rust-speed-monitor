use crate::config::Config;
use crate::handler::util::handle_empty;
use iron::middleware::Handler;
use iron::IronResult;
use iron::Request as IronRequest;
use iron::Response as IronResponse;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct MetricsHandler {
    metrics: BTreeSet<String>,
}

impl MetricsHandler {
    pub fn new(config: &Config) -> MetricsHandler {
        let metrics = config.metrics().keys().cloned().collect();

        MetricsHandler { metrics }
    }
}

impl Handler for MetricsHandler {
    fn handle(&self, _request: &mut IronRequest) -> IronResult<IronResponse> {
        handle_empty(move || Ok(self.metrics.clone()))
    }
}
