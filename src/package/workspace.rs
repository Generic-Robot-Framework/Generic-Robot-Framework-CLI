use std::fs;
use std::path::PathBuf;
use crate::package::{PackageFile};

pub fn parse_workspace(file_path: &String) -> PackageFile {
    let path = PathBuf::from(&file_path);

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(error) => {
            eprintln!("Error: {}", error.to_string());
            panic!("Could not open workspace: {}", file_path);
        }
    };

    let package: PackageFile = match toml::from_str(content.as_str()) {
        Ok(pkg) => pkg,
        Err(error) => {
            eprintln!("Error: {}", error.message());
            panic!("Could not parse workspace: {}", file_path);
        }
    };

    return package;
}