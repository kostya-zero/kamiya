use std::{env, process::{Command, exit}, io::Read};
use clipboard::{ClipboardContext, ClipboardProvider};
use home::home_dir;
use wl_clipboard_rs::{copy::{MimeType, Options, Source}, paste::get_contents};
use crate::term::Term;

pub enum CurrentPlatform {
    Windows,
    Linux,
    Mac,
    Unknown
}

pub enum SessionType {
    X11,
    Wayland,
    NonUnix
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

    pub fn get_session_type() -> SessionType {
        let session_type = env::var("XDG_SESSION_TYPE");
        if session_type.is_err() {
            return SessionType::NonUnix;
        }

        let session: &str = &session_type.unwrap();
        
        match session {
            "x11" => SessionType::X11,
            "wayland" => SessionType::Wayland,
            &_ => panic!("Unknown type of session!")
        }
    }

    pub fn set_clipboard(content: &str) {
        let session_type: SessionType = Utils::get_session_type();
        match session_type {
            SessionType::NonUnix => {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(content.to_string()).expect("Failed to push content to the clipboard.");
            }
            SessionType::X11 => {
                let mut cmd = Command::new("sh");
                cmd.args(vec!["-c", format!("\"echo \"{}\" | xclip -i -selection c -rmlastnl\"", content).as_str()]);
                let result = cmd.output();
                if result.is_err() {
                    Term::fatal("Failed to copy content to clipboard. Check if xclip installed properly.");
                    exit(1);
                }
            }
            SessionType::Wayland => {
                let opts = Options::new();
                let copy_result = opts.copy(Source::Bytes(content.to_string().into_bytes().into()), MimeType::Text);
                if copy_result.is_err() {
                    Term::fatal("Failed to write content to clipboard. Check if wl-clipboard installed properly.");
                    exit(1);
                }
            }
        }
    }
    
    #[allow(unused_assignments)]
    pub fn get_clipboard() -> String {
        let session_type: SessionType = Utils::get_session_type();
        let mut buffer_content: String = String::new();
        match session_type {
            SessionType::NonUnix => {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                buffer_content = ctx.get_contents().expect("Failed to get content of clipboard.");
            }
            SessionType::X11 => {
                let mut cmd = Command::new("xclip");
                cmd.args(vec!["-o", "-selection", "c", "-rmlastnl"]);
                let result = cmd.output();
                if result.is_err() {
                    Term::fatal("Failed to get clipboard content. Check if xclip installed properly.");
                    exit(1);
                }

                buffer_content = String::from_utf8_lossy(&result.unwrap().stdout).to_string();
            }
            SessionType::Wayland => {
                let copy_result = get_contents(wl_clipboard_rs::paste::ClipboardType::Regular, wl_clipboard_rs::paste::Seat::Unspecified, wl_clipboard_rs::paste::MimeType::Text);
                if copy_result.is_err() {
                    Term::fatal("Failed to get content of clipboard. Check if wl-clipboard installed properly.");
                    exit(1);
                }
                let mut pipe_content = vec![];
                copy_result.unwrap().0.read_to_end(&mut pipe_content).expect("Failed to read pipe stream.");
                buffer_content = String::from_utf8_lossy(&pipe_content).to_string();
            }
        }

        buffer_content
    }
}
