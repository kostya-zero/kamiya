use std::env;
use home::home_dir;

pub enum CurrentPlatform {
    Windows,
    Linux,
    Mac,
    Unknown
}

pub struct Utils;
impl Utils {
    pub fn detect_platform() -> CurrentPlatform {
        match env::consts::OS {
            "linux" => CurrentPlatform::Linux,
            "macos" => CurrentPlatform::Mac,
            "windows" => CurrentPlatform::Windows,
            _ => CurrentPlatform::Unknown
        }
    }

    pub fn get_user_home() -> String {
        home_dir().expect("Failed to get user directory (why).").display().to_string()
    }

    pub fn get_temp_dir() -> String {
        let platform: CurrentPlatform = Self::detect_platform();
        let temp: String = match platform {
            CurrentPlatform::Windows => Self::get_user_home() + "\\AppData\\Local\\Temp\\",
            CurrentPlatform::Linux => "/tmp/".to_string(),
            CurrentPlatform::Mac => "/tmp/".to_string(),
            CurrentPlatform::Unknown => panic!("Unknown platform detected!")
        };
        temp
    }
}
