#[cfg(not(windows))]
compile_error!("rchronos only supports Windows service builds.");

mod config;
mod actor;
mod sync;
mod web;
mod windows_service;

use config::{AppConfig, AppConfigExt, config_path};
use windows_service::{dispatch_service, report_event_log};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    #[error(transparent)]
    Windows(#[from] rchronos_windows::Error),
    #[error(transparent)]
    Service(#[from] ::windows_service::Error),
}

impl AppError {
    pub fn msg(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }
}

fn load_config_or_default(path: &std::path::Path) -> AppConfig {
    match AppConfig::load(path) {
        Ok(config) => config,
        Err(err) => {
            // 这里可能还没有全局 logging，但可以打印标准错误
            eprintln!("failed to load config: {err}");
            AppConfig::default()
        }
    }
}

fn main() -> ::windows_service::Result<()> {
    let config_path = config_path();
    let config = load_config_or_default(&config_path);
    let service_name = config.service_name.clone();

    if let Err(e) = dispatch_service(service_name) {
        report_event_log(
            rchronos_windows::EventLogLevel::Error,
            &format!("Failed to start service dispatcher: {e}"),
        );
        return Err(e);
    }
    Ok(())
}
