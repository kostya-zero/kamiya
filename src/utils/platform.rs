use home::home_dir;
use std::env;


pub enum CurrentPlatform {
    Windows,
    Linux,
    Mac,
}

pub enum PlatformError {
    UnknownPlatform,
    UnsupportedSystem
}

pub struct Platform;
impl Platform {
    pub fn detect_platform() -> Result<CurrentPlatform, PlatformError> {
        match env::consts::OS {
            "linux" => Result::Ok(CurrentPlatform::Linux),
            "macos" => Result::Ok(CurrentPlatform::Mac),
            "windows" => Result::Ok(CurrentPlatform::Windows),
            _ => Result::Err(PlatformError::UnknownPlatform),
        }
    }

    pub fn get_user_home() -> Result<String, PlatformError> {
        match home_dir() {
            Some(path) => Result::Ok(path.display().to_string()),
            None => Result::Err(PlatformError::UnsupportedSystem)
        }
    }

    pub fn get_temp_dir() -> Result<String, PlatformError> {
        match Self::detect_platform() {
            Ok(platform) => {
                match platform {
                    CurrentPlatform::Mac => Result::Ok(String::from("/tmp/")),
                    CurrentPlatform::Linux => Result::Ok(String::from("/tmp/")),
                    CurrentPlatform::Windows => {
                        let user_home_dir: String = match Self::get_user_home() {
                            Ok(path) => path,
                            Err(e) => panic!("Failed to get user home directory.")
                        };
                        Result::Ok(String::from(user_home_dir + "\\AppData\\Local\\Temp\\"))
                    }
                }
            },
            Err(e) => {
                Result::Err(e)
            }
        }
    }
}
