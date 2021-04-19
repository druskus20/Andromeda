use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use toml::*;

pub struct Config {
    config_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let default_config_dir = std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap()).join(".config"))
            .join("andromeda");

        Config::from(default_config_dir)
    }
}

impl<P: AsRef<Path>> From<P> for Config {
    fn from(path: P) -> Self {
        Config {
            config_path: PathBuf::from(path.as_ref()),
        }
    }
}

impl Config {
    pub fn new(path: impl AsRef<Path>) -> Result<Config> {
        // There will be more stuff here once I implement CL arguments
        Ok(Self::default())
    }
}
