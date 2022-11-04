use std::path::PathBuf;

use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    JSON,
    Pretty,
    Compact,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KesConfig {
    #[serde(default = "KesConfig::default_port")]
    pub port: u16,

    #[serde(default = "KesConfig::default_workers")]
    pub workers: usize,

    #[serde(default = "KesConfig::default_log_format")]
    pub log_format: LogFormat,

    #[serde(default = "KesConfig::default_log_level")]
    pub log_level: String,

    #[serde(default = "KesConfig::default_posts_dir")]
    pub posts_dir: String,

    #[serde(default = "KesConfig::default_assets_dir")]
    pub assets_dir: String,

    pub home_template: Option<PathBuf>,

    pub post_template: Option<PathBuf>,

    pub not_found_template: Option<PathBuf>,
}

impl KesConfig {
    fn default_port() -> u16 {
        3000
    }

    fn default_workers() -> usize {
        4
    }

    fn default_log_format() -> LogFormat {
        LogFormat::JSON
    }

    fn default_log_level() -> String {
        "error".to_string()
    }

    fn default_posts_dir() -> String {
        "posts".to_string()
    }

    fn default_assets_dir() -> String {
        "assets".to_string()
    }
}

pub fn get_config() -> KesConfig {
    Config::builder()
        .add_source(File::with_name("kes.toml"))
        .add_source(Environment::with_prefix("KES"))
        .build()
        .expect("failed to load settings")
        .try_deserialize::<KesConfig>()
        .expect("failed to deserialize settings")
}
