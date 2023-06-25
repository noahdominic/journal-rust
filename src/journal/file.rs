use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn mkdir_p(config_file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = expand_file_path(&config_file_path)?;
    println!("{}", path.to_string_lossy());
    fs::create_dir_all(path)?;
    Ok(())
}

pub(crate) fn expand_file_path(file_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let file_path = Path::new(file_name).to_path_buf();

    if file_path.starts_with("~") {
        let home_path = dirs::home_dir().ok_or("Could not determine home directory")?;
        return Ok(home_path.join(file_path.strip_prefix("~")?));
    }

    Ok(file_path)
}
