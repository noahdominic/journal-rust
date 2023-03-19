use std::io::Write;
use dirs;

/// Expands a given file name to a full file path by replacing the `~` symbol with the user's home directory.
///
/// # Arguments
///
/// * `file_name` - A string slice that represents the file name to expand.
///
/// # Returns
///
/// A `Result` object that contains a `std::path::PathBuf` type if the file path is successfully expanded, or a `Box<dyn std::error::Error>` type if an error occurs.
///
pub(crate) fn expand_file_path(file_name: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let file_path = std::path::Path::new(file_name);
    let home_dir = dirs::home_dir().ok_or("Could not determine home directory")?;
    let file_path = home_dir.join(file_path.strip_prefix("~")?);
    Ok(file_path)
}

/// Creates a directory path for a given file path.
///
/// # Arguments
///
/// * `file_path` - A `std::path::Path` type that represents the file path to create a directory path for.
///
/// # Returns
///
/// A `Result` object that contains `()` if the directory path is successfully created, or a `Box<dyn std::error::Error>` type if an error occurs.
///
pub(crate) fn create_file(file_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(file_path.parent().ok_or("Could not determine parent directory")?)?;
    Ok(())
}

/// Writes the preamble to a file at a given file path after the user is prompted and gives a positive response
///
/// # Arguments
///
/// * `file_path` - A `std::path::Path` type that represents the file path to write the preamble to.
/// * `preamble` - A string slice that represents the preamble to write to the file.
///
/// # Returns
///
/// A `Result` object that contains `()` if the preamble is successfully written to the file, or a `Box<dyn std::error::Error>` type if an error occurs.
///
pub(crate) fn write_preamble(file_path: &std::path::Path, preamble: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", preamble);
    if crate::journal::query::for_bool("Does everything look correct?  This will print in the file if yes.")? {
        let mut file = std::fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&file_path)?;
        writeln!(file, "{}", preamble)?;
    } else {
        println!("OK.  File not written.")
    }
    Ok(())
}