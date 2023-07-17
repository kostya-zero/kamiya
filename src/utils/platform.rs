use home::home_dir;
use std::{env, process::exit};

use crate::term::Term;

pub enum CurrentPlatform {
    Windows,
    Linux,
    Mac,
    Unknown,
}

pub struct Platform;
impl Platform {
    pub fn detect_platform() -> CurrentPlatform {
        match env::consts::OS {
            "linux" => CurrentPlatform::Linux,
            "macos" => CurrentPlatform::Mac,
            "windows" => CurrentPlatform::Windows,
            _ => CurrentPlatform::Unknown,
        }
    }

    pub fn get_user_home() -> String {
        match home_dir() {
            Some(path) => {
                return path.display().to_string();
            }
            None => {
                Term::fatal("Failed to get home directory. Maybe unsupported system.");
                exit(1);
            }
        }
    }

    pub fn get_temp_dir() -> String {
        let platform: CurrentPlatform = Self::detect_platform();
        let temp: String = match platform {
            CurrentPlatform::Windows => Self::get_user_home() + "\\AppData\\Local\\Temp\\",
            CurrentPlatform::Linux => "/tmp/".to_string(),
            CurrentPlatform::Mac => "/tmp/".to_string(),
            CurrentPlatform::Unknown => panic!("Unknown platform detected!"),
        };
        temp
    }
}
