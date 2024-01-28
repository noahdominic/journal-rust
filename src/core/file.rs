// Copyright 2023, 2024  Noah Dominic Miranda Silvio
// Licensed under the EUPL v1.2


use directories;

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
            FileError::FailedToCreateConfigDir => write!(f, "Failed to create Journey's config directory."),
            FileError::ProjDirsNotFound => write!(f, "Project directories cannot be found."),
            FileError::ErrorDuringWriting(ref err) => err.fmt(f),
        }
    }
}

fn get_config_file_path() -> Result<std::path::PathBuf, FileError> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
        let config_dir_path = proj_dirs.config_dir();

        std::fs::create_dir_all(&config_dir_path).map_err(|_| FileError::FailedToCreateConfigDir)?;

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