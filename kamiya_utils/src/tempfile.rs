use super::platform::{Platform, PlatformError};
use std::{fs, io::Error};

pub struct TempFile {
    path: String,
}

impl TempFile {
    pub fn new(file_name: &str) -> Result<Self, PlatformError> {
        match Platform::get_temp_dir() {
            Ok(path) => Ok(Self {
                path: path + file_name,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn destroy(&self) -> Result<(), Error> {
        match fs::remove_file(&self.path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
