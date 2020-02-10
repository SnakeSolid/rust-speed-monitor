use crate::config::Config;
use crate::error::ApplicationError;
use crate::error::ApplicationResult;
use crate::options::ServerOptions;
use iron::Iron;
use mount::Mount;
use staticfile::Static;

#[allow(clippy::needless_pass_by_value)]
pub fn start(options: &ServerOptions, _config: &Config) -> ApplicationResult {
    let mut mount = Mount::new();
    // mount.mount("/api/v1/settings", SettingsHandler::new(config.clone()));
    mount.mount("/", Static::new("public"));

    let address = options.address();
    let port = options.port();

    println!("Listening on {}:{}...", address, port);

    Iron::new(mount)
        .http((address, port))
        .map(|_| ())
        .map_err(ApplicationError::http_error)
}
