/**
 * File finder module
 * 
 * Provides functionality for searching files in a directory tree
 * and opening them in VS Code
 */

use std::path::{Path, PathBuf};
use walkdir::WalkDir;


/**
 * FileFinder is responsible for locating files that match search criteria
 * and providing relative paths for display
 */

pub struct FileFinder {
    base_directory: PathBuf,
}

impl FileFinder {
    /// Create a new FileFinder with the given base directory
    pub fn new<P: AsRef<Path>>(base_directory: P) -> Self {
        FileFinder {
            base_directory: base_directory.as_ref().to_path_buf(),
        }
    }

    /// Find all files in the base directory that match the given keyword
    pub fn find_files(&self, keyword: &str) -> Vec<PathBuf> {
        WalkDir::new(&self.base_directory)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .to_lowercase()
                    .contains(&keyword.to_lowercase())
            })
            .map(|entry| entry.path().to_path_buf())
            .collect()
    }

    /// Get path relative to the base directory
    pub fn get_relative_path(&self, path: &Path) -> PathBuf {
        pathdiff::diff_paths(path, &self.base_directory)
            .unwrap_or_else(|| path.to_path_buf())
    }

    /// Open a file in VS Code
    pub fn open_in_vscode(&self, file_path: &Path) -> Result<(), std::io::Error> {
        use std::process::Command;
        
        // First try the standard 'code' command
        let result = Command::new("code")
            .arg(file_path)
            .spawn();
            
        if result.is_err() {
            // If that fails, try through PowerShell
            Command::new("powershell")
                .args(["-Command", &format!("code \"{}\"", file_path.display())])
                .spawn()?;
        }
        
        Ok(())
    }
}