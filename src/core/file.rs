// Copyright 2023, 2024  Noah Dominic Miranda Silvio
// Licensed under the EUPL v1.2

use directories;
use serde::Deserialize;
use toml;
use crate::core::chrono::get_current_date_from_tz_as_str;

////////////////////////////////////////////////////////////////////////////////////////////////////
//
// Data structures required for core file functionalities
//
////////////////////////////////////////////////////////////////////////////////////////////////////

/// enum FileError
/// Represents errors that can occur during file operations in the application
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(crate) enum FileError {
    FailedToCreateConfigDir,
    FailedToCreateDataDir,
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
            },
            FileError::FailedToCreateDataDir => {
                write!(f, "Failed to create Journey's data directory.")
            }
            FileError::ProjDirsNotFound => write!(f, "Project directories cannot be found."),
            FileError::ErrorDuringWriting(ref err) => err.fmt(f),
        }
    }
}

/// enum ConfigError
/// Wrapper for all the errors that can occur during contact with the config file
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(crate) enum ConfigError {
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

/// struct ConfDefaults
/// Expected structure for config file
//   P.S.  I wish Rust had inline nested struct declarations
//   P.P.S I wish the `serde` package had automatic type deserialisers
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub(crate) struct ConfDefaults {
    pub(crate) location_full_name: String,
    #[serde(deserialize_with = "serde_string_as_f64")]
    pub(crate) location_latitude: f64,
    #[serde(deserialize_with = "serde_string_as_f64")]
    pub(crate) location_longitude: f64,
    pub(crate) timezone: String,
    pub(crate) editor: String,
}

fn serde_string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

/// struct ConfData
/// Expected structure for the config file
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub(crate) struct ConfData {
    pub(crate) defaults: ConfDefaults,
}

// Functions that get and generate dir/file paths for the journal project
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_config_file_path() -> Result<std::path::PathBuf, FileError> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
        let config_dir_path = proj_dirs.config_dir();

        std::fs::create_dir_all(&config_dir_path)
            .map_err(|_| FileError::FailedToCreateConfigDir)?;

        return Ok(config_dir_path.join("config.toml"));
    }

    Err(FileError::ProjDirsNotFound)
}

pub fn get_data_dir_path() -> Result<std::path::PathBuf, FileError> {
    let proj_dirs =
        directories::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .ok_or(FileError::ProjDirsNotFound)?;

    let data_dir_path = proj_dirs.data_dir();

    std::fs::create_dir_all(&data_dir_path)
        .map_err(|_| FileError::FailedToCreateDataDir)?;

    Ok(std::path::PathBuf::from(data_dir_path))
}

pub(crate) fn get_temp_file_path() -> Result<std::path::PathBuf, FileError> {
    Ok(get_data_dir_path()?.join(".temp_entry"))
}

pub(crate) fn get_path_for_todays_entry() -> Result<String, Box<dyn std::error::Error>>{
    let extension = "txt";

    let data_dir = get_data_dir_path()?;

    let conf_data = get_config_from_config_file()?.defaults;

    let current_date = get_current_date_from_tz_as_str(&conf_data.timezone)?;

    let todays_entry_path = format!(
        "{}/{}.{}",
        data_dir.to_string_lossy(),
        current_date.format("%Y/%m/%d.%H-%M"),
        extension
    );

    Ok(todays_entry_path)
}

// If X exists checkers
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(crate) fn is_journal_initialised() -> Result<bool, FileError> {
    Ok(get_config_file_path()?.exists())
}

// File content readers and writers
////////////////////////////////////////////////////////////////////////////////////////////////////
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
