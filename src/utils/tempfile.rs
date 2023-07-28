use super::platform::{Platform, PlatformError};
use std::{fs, io::Error};

pub struct TempFile {
    path: String,
}

impl TempFile {
    pub fn new(file_name: &str) -> Result<Self, PlatformError> {
        match Platform::get_temp_dir() {
            Ok(path) => Result::Ok(Self {
                path: path + file_name,
            }),
            Err(e) => Result::Err(e),
        }
    }

    pub fn init(&self) -> Result<String, Error> {
        match fs::write(&self.path, "") {
            Ok(_) => Result::Ok(self.path.clone()),
            Err(e) => Result::Err(e),
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        match fs::remove_file(&self.path) {
            Ok(_) => Result::Ok(()),
            Err(e) => Result::Err(e),
        }
    }
}
