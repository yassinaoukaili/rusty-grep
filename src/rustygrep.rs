use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use regex::Regex;

pub struct RustyGrep {
    regex: Regex,
    starting_dir: PathBuf,
}

impl RustyGrep {
    /// Creates a new RustyGrep instance
    pub fn new(pattern: &str, starting_dir: &str) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern)?;
        Ok(RustyGrep {
            regex,
            starting_dir: PathBuf::from(starting_dir),
        })
    }

    /// Initiates the search process
    pub fn search(&self) -> io::Result<()> {
        if !self.starting_dir.exists() {
            eprintln!(
                "Error: The starting directory '{}' does not exist.",
                self.starting_dir.display()
            );
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Starting directory not found",
            ));
        }

        if !self.starting_dir.is_dir() {
            eprintln!(
                "Error: The starting path '{}' is not a directory.",
                self.starting_dir.display()
            );
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Starting path is not a directory",
            ));
        }

        println!(
            "Searching for pattern '{}' starting from '{}'",
            self.regex, self.starting_dir.display()
        );
        self.search_in_directory(&self.starting_dir)
    }

    /// Recursively searches for files matching the regex in the given directory
    fn search_in_directory(&self, dir: &Path) -> io::Result<()> {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!(
                    "Warning: Skipping directory '{}': {}",
                    dir.display(),
                    err
                );
                return Ok(());
            }
        };

        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();

                    if path.is_dir() {
                        // Recursively search in subdirectories
                        if let Err(err) = self.search_in_directory(&path) {
                            eprintln!(
                                "Warning: Failed to process directory '{}': {}",
                                path.display(),
                                err
                            );
                        }
                    } else if path.is_file() {
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_name_str) = file_name.to_str() {
                                if self.regex.is_match(file_name_str) {
                                    println!("Found: {}", path.display());
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Warning: Failed to read an entry in '{}': {}", dir.display(), err);
                }
            }
        }

        Ok(())
    }
}
