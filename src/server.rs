use crate::config::Config;
use crate::error::ApplicationError;
use crate::error::ApplicationResult;
use crate::handler::DataHandler;
use crate::handler::MetricsHandler;
use crate::options::ServerOptions;
use iron::Iron;
use mount::Mount;
use staticfile::Static;

pub fn start(options: &ServerOptions, config: &Config) -> ApplicationResult {
    let mut mount = Mount::new();
    mount.mount("/api/v1/metrics", MetricsHandler::new(config));
    mount.mount("/api/v1/data", DataHandler::new(config));
    mount.mount("/static", Static::new("public/static"));
    mount.mount("/", Static::new("public"));

    let address = options.address();
    let port = options.port();

    println!("Listening on {}:{}...", address, port);

    Iron::new(mount)
        .http((address, port))
        .map(|_| ())
        .map_err(ApplicationError::http_error)
}
