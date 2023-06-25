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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn test_expand_file_path_absolute_path() {
        let file_name = "/path/to/file.txt";
        let expected_path = PathBuf::from("/path/to/file.txt");
        assert_eq!(expand_file_path(file_name).unwrap(), expected_path);
    }

    #[test]
    fn test_expand_file_path_relative_path() {
        let file_name = "relative/file.txt";
        let expected_path = PathBuf::from("relative/file.txt");
        assert_eq!(expand_file_path(file_name).unwrap(), expected_path);
    }

    #[test]
    fn test_expand_file_path_with_tilde() {
        let file_name = "~/path/to/file.txt";
        let expected_path = dirs::home_dir().unwrap().join("path/to/file.txt");
        assert_eq!(expand_file_path(file_name).unwrap(), expected_path);
    }

    #[test]
    fn test_expand_file_path_dirname_with_tilde_prefix() {
        let file_name = "~path/to/file.txt";
        let expected_path = PathBuf::from("~path/to/file.txt");
        assert_eq!(expand_file_path(file_name).unwrap(), expected_path);
    }

    #[test]
    fn test_mkdir_p_existing_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path().join("existing_dir");
        fs::create_dir(&dir_path).unwrap();

        let result = mkdir_p(dir_path.to_string_lossy().into_owned());
        assert!(result.is_ok());
    }

    #[test]
    fn test_mkdir_p_new_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path().join("new_dir");

        let result = mkdir_p(dir_path.to_string_lossy().into_owned());
        assert!(result.is_ok());

        assert!(Path::new(&dir_path).exists());
    }

    #[test]
    fn test_mkdir_p_invalid_path() {
        let invalid_path = String::from("/path/does/not/exist");

        let result = mkdir_p(invalid_path);
        assert!(result.is_err());
    }
}
