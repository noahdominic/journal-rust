/*
 * Copyright 2023 Noah Dominic Miranda Silvio
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the European Union Public License version 1.2,
 * as published by the European Commission.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * European Union Public Licence for more details.
 *
 * You should have received a copy of the European Union Public Licence
 * along with this program. If not, see <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
 */

use serde::de::Error;

/**
 * Represents custom errors related to file operations.
 *
 * This enum defines custom error types for file-related operations.
 *
 * # Variants
 *
 * - `HomeDirNotFound`: Indicates that the user's home directory could not be found.
 * - `InvalidPath`: Indicates that the provided path is invalid.
 *
 * # Example
 *
 * ```no_run
 * fn main() -> Result<(), FileError> {
 *     let result: Result<(), FileError> = Err(FileError::HomeDirNotFound);
 *     match result {
 *         Ok(_) => println!("Operation successful"),
 *         Err(err) => println!("Error: {}", err),
 *     }
 *     Ok(())
 * }
 * ```
 */
#[derive(Debug)]
pub(crate) enum FileError {
    HomeDirNotFound,
    InvalidPath,
}

impl std::error::Error for FileError {}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::HomeDirNotFound => write!(f, "Home directory not found"),
            FileError::InvalidPath => write!(f, "Path is invalid"),
        }
    }
}

/**
 * Creates a directory at the specified path, including all parent directories if they don't exist,
 * analogous to UNIX command `mkdir -p <name>`
 *
 * This function takes a `config_file_path` as input, which is the path to the directory that needs to be created.
 * It returns a `Result` containing a `std::path::PathBuf` representing the created directory's path on success,
 * or a boxed `dyn std::error::Error` on failure.
 *
 * # Arguments
 *
 * - `config_file_path`: A `String` representing the path to the directory that needs to be created.
 *
 * # Returns
 *
 * - `Result<std::path::PathBuf, Box<dyn std::error::Error>>`: A `Result` that wraps the path of the created directory
 *   on success, or an error on failure.
 *
 * # Example
 *
 * ```
 * use std::path::PathBuf;
 *
 * fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     let config_file_path = String::from("path/to/directory");
 *     let created_directory: PathBuf = mkdir_p(config_file_path)?;
 *     println!("Directory created: {:?}", created_directory);
 *     Ok(())
 * }
 * ```
 */
pub(crate) fn mkdir_p(
    config_file_path: String,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Expand the provided file path to a full path
    let path = expand_file_path(&config_file_path)?;

    // Print the full path for debugging purposes
    println!("{}", path.to_string_lossy());

    // Create the directory and all parent directories if they don't exist
    std::fs::create_dir_all(path.clone())?;

    // Return the path of the created directory on success
    Ok(path)
}

/**
 * Expands a file name or path, including tilde expansion to the user's home directory.
 *
 * This function takes a `file_name` as input, which can be a relative or absolute file path.
 * It returns a `Result` containing a `std::path::PathBuf` representing the expanded file path on success,
 * or a `FileError` on failure.
 *
 * # Arguments
 *
 * - `file_name`: A `&str` representing the file name or path to be expanded.
 *
 * # Returns
 *
 * - `Result<std::path::PathBuf, FileError>`: A `Result` that wraps the expanded file path on success,
 *   or a `FileError` indicating the type of failure.
 *
 * # Example
 *
 * ```
 * use std::path::PathBuf;
 *
 * fn main() -> Result<(), FileError> {
 *     let file_name = "~/documents/report.txt";
 *     let expanded_path: PathBuf = expand_file_path(file_name)?;
 *     println!("Expanded path: {:?}", expanded_path);
 *     Ok(())
 * }
 * ```
 */
pub(crate) fn expand_file_path(file_name: &str) -> Result<std::path::PathBuf, FileError> {
    // Check if the file_name is empty
    if file_name.is_empty() {
        return dirs::home_dir()
            .ok_or(FileError::HomeDirNotFound)
            .map(|home_path| home_path.join("journal"));
    }

    // Convert the file_name to a PathBuf
    let file_path = std::path::Path::new(file_name).to_path_buf();

    // Handle tilde (~) expansion
    if file_path.starts_with("~") {
        return dirs::home_dir()
            .ok_or(FileError::HomeDirNotFound)
            .and_then(|home_path| {
                file_path
                    .strip_prefix("~")
                    .map(|stripped_path| home_path.join(stripped_path))
                    .map_err(|_| FileError::InvalidPath)
            });
    }

    // Return the expanded file path on success
    Ok(file_path)
}

/**
 * Handles the case where a configuration file already exists at the provided path.
 *
 * This function takes a `config_file_pathbuf` as input, which is a reference to a `std::path::PathBuf`
 * representing the path to the configuration file. It checks if the file exists and interacts with the user
 * to decide whether to proceed with writing to the file or cancel the operation.
 *
 * If the file already exists, the user is prompted to confirm overwriting it. If the user confirms,
 * the function returns `Ok(true)`, indicating that writing should proceed. If the user cancels or if
 * the file doesn't exist, the function returns `Ok(false)` to indicate cancellation.
 *
 * # Arguments
 *
 * - `config_file_pathbuf`: A reference to a `std::path::PathBuf` representing the path to the configuration file.
 *
 * # Returns
 *
 * - `Result<bool, std::io::Error>`: A `Result` that contains `true` if writing should proceed or `false` if cancelled,
 *   or an `std::io::Error` if there was an I/O error during the process.
 *
 * # Example
 *
 * ```
 * use std::path::PathBuf;
 *
 * fn main() -> Result<(), std::io::Error> {
 *     let config_file_path: PathBuf = PathBuf::from("/path/to/config.toml");
 *     let should_proceed: bool = handle_file_exists(&config_file_path)?;
 *     if should_proceed {
 *         // Perform writing to the configuration file
 *         println!("Writing to config file...");
 *     } else {
 *         println!("Operation cancelled.");
 *     }
 *     Ok(())
 * }
 * ```
 */
pub(crate) fn is_proceed_with_writing(
    config_file_pathbuf: &std::path::PathBuf,
) -> Result<bool, std::io::Error> {
    // Check if the configuration file already exists
    if std::path::Path::new(&config_file_pathbuf).exists() {
        // Prompt the user for confirmation to overwrite
        if !crate::journal::query::for_bool(&format!(
            "A config.toml already exists in {}.  Overwrite?",
            config_file_pathbuf.to_string_lossy()
        ))? {
            // Cancel the operation if the user doesn't want to overwrite
            println!("Config init cancelled.");
            return Ok(false);
        }
    }
    // Proceed with writing if the user confirms or the file doesn't exist
    Ok(true)
}

/**
 * Checks if a dotfile exists at the designated path.
 *
 * This function determines whether a dotfile exists at the path obtained from `get_dotfile_path`.
 * It returns a `Result` containing `true` if the dotfile exists or `false` if it doesn't exist,
 * or a `FileError` on failure.
 *
 * # Returns
 *
 * - `Result<bool, FileError>`: A `Result` that contains `true` if the dotfile exists, `false` if it doesn't exist,
 *   or a `FileError` indicating the type of failure.
 *
 * # Example
 *
 * ```
 * fn main() -> Result<(), FileError> {
 *     let dotfile_exists: bool = is_dotfile_exists()?;
 *     if dotfile_exists {
 *         println!("Dotfile exists.");
 *     } else {
 *         println!("Dotfile does not exist.");
 *     }
 *     Ok(())
 * }
 * ```
 */
pub(crate) fn is_dotfile_exists() -> Result<bool, FileError> {
    // Check if the dotfile at the designated path exists
    Ok(get_dotfile_path()?.exists())
}

/**
 * Retrieves the path to the dotfile.
 *
 * This function obtains the path to the dotfile named ".journal" in the user's home directory.
 * It returns a `Result` containing a `std::path::PathBuf` representing the dotfile's path on success,
 * or a `FileError` on failure.
 *
 * # Returns
 *
 * - `Result<std::path::PathBuf, FileError>`: A `Result` that contains the path to the dotfile on success,
 *   or a `FileError` indicating the type of failure.
 *
 * # Example
 *
 * ```
 * use std::path::PathBuf;
 *
 * fn main() -> Result<(), FileError> {
 *     let dotfile_path: PathBuf = get_dotfile_path()?;
 *     println!("Dotfile path: {:?}", dotfile_path);
 *     Ok(())
 * }
 * ```
 */
pub(crate) fn get_dotfile_path() -> Result<std::path::PathBuf, FileError> {
    // Retrieve the user's home directory
    dirs::home_dir()
        .ok_or(FileError::HomeDirNotFound)
        .map(|home_path| home_path.join(".journal"))
}

/**
 * Reads the content of the journal dotfile and returns the resulting path in it.
 *
 * This function reads the content of the dotfile, which is expected to contain a path,
 * and returns the corresponding `std::path::PathBuf`. It returns a `Result` containing
 * the path on success, or a boxed `dyn std::error::Error` on failure.
 *
 * # Returns
 *
 * - `Result<std::path::PathBuf, Box<dyn std::error::Error>>`: A `Result` that contains
 *   the path read from the dotfile on success, or an error on failure.
 *
 * # Example
 *
 * ```
 * use std::path::PathBuf;
 *
 * fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     let path_from_dotfile: PathBuf = read_dotfile()?;
 *     println!("Path from dotfile: {:?}", path_from_dotfile);
 *     Ok(())
 * }
 * ```
 */
pub(crate) fn get_base_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Get the path to the dotfile
    let dotfile_path = get_dotfile_path()?;

    // Read the content of the dotfile
    let dotfile_content = std::fs::read_to_string(dotfile_path)?;

    // Convert the content to a PathBuf
    let base_dir = std::path::PathBuf::from(dotfile_content);

    // Return the resulting path on success
    Ok(base_dir)
}

/**
 * Reads configuration values from a TOML file and returns them.
 *
 * This function reads configuration values from a TOML file located at the provided path.
 * It returns a `Result` containing a tuple of configuration values on success, or a boxed
 * `dyn std::error::Error` on failure.
 *
 * # Arguments
 *
 * - `config_file_path`: A reference to a `std::path::PathBuf` representing the path to the
 *   directory containing the TOML file.
 *
 * # Returns
 *
 * - `Result<(String, String, String, String, String), Box<dyn std::error::Error>>`: A `Result`
 *   that contains a tuple of configuration values (location full name, location latitude,
 *   location longitude, timezone, and editor) on success, or an error on failure.
 *
 * # Example
 *
 * ```
 * use std::path::PathBuf;
 *
 * fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     let config_file_path: PathBuf = PathBuf::from("/path/to/config/file");
 *     let config_values = read_configfile(&config_file_path)?;
 *     println!("Location Full Name: {}", config_values.0);
 *     println!("Location Latitude: {}", config_values.1);
 *     // ...
 *     Ok(())
 * }
 * ```
 */
pub(crate) fn get_config_details(
) -> Result<(String, String, String, String, String), Box<dyn std::error::Error>> {
    let config_file_path = get_base_dir()?;

    // Read the TOML content from the config file as a string
    let toml_content_as_string = std::fs::read_to_string(config_file_path.join("config.toml"))?;

    // Deserialize the TOML content into a toml::Value
    let toml_value: toml::Value = toml::from_str(&toml_content_as_string)?;

    // Retrieve the "defaults" table from the TOML value
    let defaults_table = toml_value
        .get("defaults")
        .ok_or(toml::de::Error::custom("Missing 'defaults' table"))?
        .as_table()
        .ok_or(toml::de::Error::custom("Invalid 'defaults' table"))?;

    // Retrieve individual configuration values from the "defaults" table
    let location_full_name = get_string_value(defaults_table, "location_full_name")?;
    let location_latitude = get_string_value(defaults_table, "location_latitude")?;
    let location_longitude = get_string_value(defaults_table, "location_longitude")?;
    let timezone = get_string_value(defaults_table, "timezone")?;
    let editor = get_string_value(defaults_table, "editor")?;

    // Return the tuple of configuration values on success
    Ok((
        location_full_name,
        location_latitude,
        location_longitude,
        timezone,
        editor,
    ))
}

pub(crate) fn get_temp_file_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let config_file_path = get_base_dir()?;
    Ok(config_file_path.join(".temp_file"))
}

/**
 * Retrieves a string value from a TOML table based on the specified key.
 *
 * This function retrieves a string value associated with the specified key from a TOML table.
 * It returns a `Result` containing the string value on success, or a `toml::de::Error` on failure.
 *
 * # Arguments
 *
 * - `table`: A reference to a `toml::value::Table` from which to retrieve the value.
 * - `key`: The key to look up within the TOML table.
 *
 * # Returns
 *
 * - `Result<String, toml::de::Error>`: A `Result` that contains the retrieved string value on success,
 *   or a `toml::de::Error` indicating the type of failure.
 *
 * # Example
 *
 * ```no_run
 * use toml::value::Table;
 *
 * fn main() -> Result<(), toml::de::Error> {
 *     let mut table = Table::new();
 *     table.insert("location_full_name".to_string(), toml::Value::String("City".to_string()));
 *     let key = "location_full_name";
 *     let value: String = get_string_value(&table, key)?;
 *     println!("Value for '{}': {}", key, value);
 *     Ok(())
 * }
 * ```
 */
fn get_string_value(table: &toml::value::Table, key: &str) -> Result<String, toml::de::Error> {
    match table.get(key) {
        Some(value) => {
            if let Some(str_value) = value.as_str() {
                Ok(str_value.to_owned())
            } else {
                Err(toml::de::Error::custom(format!("Invalid '{}' field", key)))
            }
        }
        None => Err(toml::de::Error::custom(format!("Missing '{}' field", key))),
    }
}
