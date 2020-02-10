mod error;

pub use self::error::HttpClientError;
pub use self::error::HttpClientResult;

use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::IntoUrl;
use std::fmt::Display;

#[derive(Debug)]
struct HttpClient {
    url: String,
    client: Client,
}

impl HttpClient {
    fn create(url: String) -> HttpClientResult<HttpClient> {
        let mut builder = Client::builder().no_gzip();

        Ok(HttpClient {
            url,
            client: builder.build().map_err(HttpClientError::reqwest_error)?,
        })
    }

    pub fn get<U>(self, url: U) -> HttpClientResult<Response>
    where
        U: IntoUrl + Display,
    {
        info!("Connecting: URL = {}", url);

        self.client
            .get(url)
            .send()
            .map_err(HttpClientError::reqwest_error)
    }
}
