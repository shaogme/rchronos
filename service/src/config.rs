use std::{
    fs,
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use thiserror::Error;

pub use rchronos_shared::AppConfig;

pub const APP_NAME: &str = "Rchronos";
pub const CONFIG_FILE_SUFFIX: &str = ".toml";

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
}

impl ConfigError {
    fn msg(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }
}

pub trait AppConfigExt {
    fn load(path: &Path) -> Result<AppConfig, ConfigError>;
    fn save(&self, path: &Path) -> Result<(), ConfigError>;
}

impl AppConfigExt for AppConfig {
    fn load(path: &Path) -> Result<AppConfig, ConfigError> {
        if !path.exists() {
            return Ok(AppConfig::default());
        }

        let mut file = fs::File::open(path)
            .map_err(|e| ConfigError::msg(format!("open config {:?}: {e}", path)))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| ConfigError::msg(format!("read config {:?}: {e}", path)))?;

        let config = toml::from_str(&content)
            .map_err(|e| ConfigError::msg(format!("parse config {:?}: {e}", path)))?;
        Ok(config)
    }

    fn save(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ConfigError::msg(format!("create config directory {:?}: {e}", parent))
            })?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::msg(format!("serialize config {:?}: {e}", path)))?;
        let file = fs::File::create(path)
            .map_err(|e| ConfigError::msg(format!("write config {:?}: {e}", path)))?;
        let mut writer = BufWriter::new(file);
        writer
            .write_all(content.as_bytes())
            .map_err(|e| ConfigError::msg(format!("write config content {:?}: {e}", path)))?;
        Ok(())
    }
}

pub fn config_path() -> PathBuf {
    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from(APP_NAME));
    exe.with_extension(CONFIG_FILE_SUFFIX.trim_start_matches('.'))
}
