mod error;

pub use self::error::ConfigError;
pub use self::error::ConfigResult;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

pub type ConfigRef = Arc<Config>;

#[derive(Debug, Deserialize)]
pub struct Config {
    storage_root: PathBuf,
    metrics: HashMap<String, MerticSettings>,
}

impl Config {
    pub fn storage_root(&self) -> &Path {
        &self.storage_root
    }

    pub fn metrics(&self) -> &HashMap<String, MerticSettings> {
        &self.metrics
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum MerticSettings {
    File {
        interval: u64,
        warmup_size: u64,
        measure_size: u64,
        path: PathBuf,
    },
    Tcp {
        interval: u64,
        warmup_size: u64,
        measure_size: u64,
        address: String,
        port: u16,
    },
    Http {
        interval: u64,
        warmup_size: u64,
        measure_size: u64,
        url: String,
    },
}

pub fn load<P>(path: P) -> ConfigResult<ConfigRef>
where
    P: AsRef<Path>,
{
    let reader = File::open(path).map_err(ConfigError::io_error)?;
    let config = serde_yaml::from_reader(reader).map_err(ConfigError::yaml_error)?;

    Ok(Arc::new(config))
}
