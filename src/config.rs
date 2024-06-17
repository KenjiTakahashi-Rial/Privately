use std::{env, sync::OnceLock};

use crate::error::{Result, ServerError};

const LAYOUT_DIR_ENV: &str = "SERVICE_LAYOUT_DIR";

pub fn new() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| panic!("failed to load config: {ex:?}"))
    })
}

pub struct Config {
    pub layout_dir: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            layout_dir: get_env(LAYOUT_DIR_ENV)?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| ServerError::NoEnvConfig(name))
}
