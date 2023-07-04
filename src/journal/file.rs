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

pub(crate) fn expand_file_path(file_name: &str) -> Result<std::path::PathBuf, FileError> {
    if file_name.is_empty() {
        return dirs::home_dir()
            .ok_or(FileError::HomeDirNotFound)
            .map(|home_path| home_path.join("journal"));
    }

    let file_path = std::path::Path::new(file_name).to_path_buf();

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

    Ok(file_path)
}

pub(crate) fn handle_file_exists(
    config_file_pathbuf: &std::path::PathBuf,
) -> Result<bool, std::io::Error> {
    // When a config.toml exists...
    if std::path::Path::new(&config_file_pathbuf).exists() {
        // ...ask the user if they want to overwrite it then...
        if !crate::journal::query::for_bool(&format!(
            "A config.toml already exists in {}.  Overwrite?",
            config_file_pathbuf.to_string_lossy()
        ))? {
            // ...cancel if they don't want to or...
            println!("Config init cancelled.");
            return Ok(false);
        }
    }
    // ...proceed with writing, if they do.
    Ok(true)
}

pub(crate) fn get_dotfile_path() -> Result<std::path::PathBuf, FileError> {
    dirs::home_dir()
        .ok_or(FileError::HomeDirNotFound)
        .map(|home_path| home_path.join(".journal"))
}

pub(crate) fn read_dotfile() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let dotfile_path = get_dotfile_path()?;
    let dotfile_content = std::fs::read_to_string(dotfile_path)?;
    let base_dir = std::path::PathBuf::from(dotfile_content);
    Ok(base_dir)
}

pub(crate) fn read_configfile(config_file_path: std::path::PathBuf) {
    // TODO
}
