use anyhow::Result;
use std::path::PathBuf;

const DATA_DIRECTORY_NAME: &str = "data";
const DATABASE_FILENAME: &str = "subtracker.db";

pub fn get_database_path() -> Result<PathBuf> {
    Ok(std::env::current_dir()?
        .join(DATA_DIRECTORY_NAME)
        .join(DATABASE_FILENAME))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_database_path_filename() {
        let database_path = get_database_path().unwrap();
        assert_eq!(
            database_path.file_name(),
            Some(OsStr::new(DATABASE_FILENAME))
        );
    }

    #[test]
    fn test_database_path_parent_is_data() {
        let database_path = get_database_path().unwrap();
        let parent_name = database_path.parent().and_then(|p| p.file_name());
        assert_eq!(parent_name, Some(OsStr::new(DATA_DIRECTORY_NAME)));
    }

    #[test]
    fn test_database_path_is_absolute() {
        let database_path = get_database_path().unwrap();
        assert!(database_path.is_absolute());
    }
}
