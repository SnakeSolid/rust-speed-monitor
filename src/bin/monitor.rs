#[macro_use]
extern crate log;

use speed_monitor::config;
use speed_monitor::config::MerticSettings;
use speed_monitor::error::ApplicationError;
use speed_monitor::error::ApplicationResult;
use speed_monitor::metric::Metric;
use speed_monitor::options::ServerOptions;
use speed_monitor::server;
use speed_monitor::worker;
use structopt::StructOpt;

fn main() -> ApplicationResult {
    env_logger::init();

    let options = ServerOptions::from_args();
    let config =
        config::load(options.config_path()).map_err(ApplicationError::load_config_error)?;

    for (name, settings) in config.metrics() {
        info!("Starting reader for {}", name);

        let metric = Metric::new(name, config.storage_root());

        match settings {
            MerticSettings::File {
                interval,
                warmup_size,
                measure_size,
                path,
            } => {
                worker::start_file_worker(*interval, *warmup_size, *measure_size, path, metric)
                    .map_err(ApplicationError::worker_error)?;
            }
            MerticSettings::Tcp {
                interval,
                warmup_size,
                measure_size,
                address,
                port,
            } => unimplemented!(),
            MerticSettings::Http {
                interval,
                warmup_size,
                measure_size,
                url,
            } => worker::start_http_worker(*interval, *warmup_size, *measure_size, url, metric)
                .map_err(ApplicationError::worker_error)?,
        }
    }

    server::start(&options, &config)
}
