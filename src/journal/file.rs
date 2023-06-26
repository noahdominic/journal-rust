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

pub(crate) fn mkdir_p(
    config_file_path: String,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let path = expand_file_path(&config_file_path)?;
    println!("{}", path.to_string_lossy());
    std::fs::create_dir_all(path.clone())?;
    Ok(path)
}

pub(crate) fn expand_file_path(
    file_name: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    if file_name.is_empty() {
        let home_path = dirs::home_dir().ok_or("Could not determine home directory")?;
        return Ok(home_path.join("journal"));
    }

    let file_path = std::path::Path::new(file_name).to_path_buf();

    if file_path.starts_with("~") {
        let home_path = dirs::home_dir().ok_or("Could not determine home directory")?;
        return Ok(home_path.join(file_path.strip_prefix("~")?));
    }

    Ok(file_path)
}
