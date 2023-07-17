use std::{fs, io::Error};
use super::platform::Platform;

pub struct TempFile {
    path: String
}

impl TempFile {
    pub fn new(file_name: &str) -> Self {
        let temp_dir_path: String = Platform::get_temp_dir() + &file_name;
        Self {
            path: temp_dir_path
        }
    }

    pub fn init(&self) -> Result<String, Error> {
        match fs::write(&self.path, "") {
            Ok(_) => Ok(self.path.clone()),
            Err(e) => panic!("{}", e)
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        match fs::remove_file(&self.path) {
            Ok(_) => Ok(()),
            Err(e) => panic!("{}", e)
        }
    }
}
