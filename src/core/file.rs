// Copyright 2023, 2024  Noah Dominic Miranda Silvio
// Licensed under the EUPL v1.2

use directories;
use serde::Deserialize;
use toml;

/// Represents errors that can occur during file operations in the application.
///
#[derive(Debug)]
pub(crate) enum FileError {
    FailedToCreateConfigDir,
    ProjDirsNotFound,
    ErrorDuringWriting(std::io::Error),
}

impl From<std::io::Error> for FileError {
    fn from(error: std::io::Error) -> Self {
        FileError::ErrorDuringWriting(error)
    }
}

impl std::error::Error for FileError {}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::FailedToCreateConfigDir => {
                write!(f, "Failed to create Journey's config directory.")
            }
            FileError::ProjDirsNotFound => write!(f, "Project directories cannot be found."),
            FileError::ErrorDuringWriting(ref err) => err.fmt(f),
        }
    }
}

/// Wrapper for all the errors that can occur during contact with the config file
///
#[derive(Debug)]
pub enum ConfigError {
    File(FileError),
    Toml(toml::de::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::File(FileError::ErrorDuringWriting(error))
    }
}

impl From<FileError> for ConfigError {
    fn from(error: FileError) -> Self {
        ConfigError::File(error)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        ConfigError::Toml(error)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::File(ref err) => err.fmt(f),
            ConfigError::Toml(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::File(ref err) => Some(err),
            ConfigError::Toml(ref err) => Some(err),
        }
    }
}

// Expected structure for config file
// P.S.  I wish Rust had inline nested struct declarations
// P.P.S I wish the `serde` package had automatic type deserialisers
#[derive(Debug, Deserialize)]
struct ConfDefaults {
    location_full_name: String,
    #[serde(deserialize_with = "serde_string_as_f64")]
    location_latitude: f64,
    #[serde(deserialize_with = "serde_string_as_f64")]
    location_longitude: f64,
    timezone: String,
    editor: String,
}

fn serde_string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize)]
pub struct ConfData {
    defaults: ConfDefaults,
}

fn get_config_file_path() -> Result<std::path::PathBuf, FileError> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
        let config_dir_path = proj_dirs.config_dir();

        std::fs::create_dir_all(&config_dir_path)
            .map_err(|_| FileError::FailedToCreateConfigDir)?;

        return Ok(config_dir_path.join("config.toml"));
    }

    Err(FileError::ProjDirsNotFound)
}

pub(crate) fn is_config_file_exists() -> Result<bool, FileError> {
    Ok(get_config_file_path()?.exists())
}

pub(crate) fn write_contents_to_config_file(config_contents: String) -> Result<(), FileError> {
    let config_file_path = get_config_file_path()?;

    let mut file = std::fs::File::create(&config_file_path)?;
    std::io::Write::write_all(&mut file, config_contents.as_bytes())?;

    Ok(())
}

pub(crate) fn get_config_from_config_file() -> Result<ConfData, ConfigError> {
    let config_file_path: std::path::PathBuf = get_config_file_path()?;

    let contents = std::fs::read_to_string(config_file_path)?;

    let conf_data: ConfData = toml::from_str(&contents)?;

    Ok(conf_data)
}
