use crate::term::Term;
use crate::utils::platform::{CurrentPlatform, Platform, SessionType};
use std::process::{exit, Command};

pub struct Clipboard;
impl Clipboard {
    pub fn set_clipboard(content: &str) {
        let session_type: SessionType = Platform::get_session_type();
        match session_type {
            SessionType::Unknown => {
                let system_type: CurrentPlatform = Platform::detect_platform();
                match system_type {
                    CurrentPlatform::Windows => {
                        let mut cmd = Command::new("cmd.exe");
                        cmd.args(vec![
                            "/C",
                            format!("\"echo \"{}\" | clip\"", content).as_str(),
                        ]);
                        let result = cmd.output();
                        if result.is_err() {
                            Term::fatal("Failed to copy content to clipboard.");
                            exit(1);
                        }
                    }
                    CurrentPlatform::Mac => {
                        let mut cmd = Command::new("zsh");
                        cmd.args(vec![
                            "-c",
                            format!("\"echo \"{}\" | pbcopy\"", content).as_str(),
                        ]);
                        let result = cmd.output();
                        if result.is_err() {
                            Term::fatal("Failed to copy content to clipboard via pbcopy.");
                            exit(1);
                        }
                    }
                    CurrentPlatform::Linux => {
                        Term::fatal("Kamiya cause in stupid situation.");
                        Term::fatal("Kamiya thinks that you are using NON UNIX PLATFORM but goes to the LINUX ENTRY FOR COPY.");
                        Term::fatal("What?...");
                        exit(1);
                    }
                    CurrentPlatform::Unknown => {
                        Term::fatal("Detected unknown platform. Cannot continue.");
                        exit(1);
                    }
                }
            }
            SessionType::X11 => {
                let mut cmd = Command::new("sh");
                cmd.args(vec![
                    "-c",
                    format!("\"echo \"{}\" | xclip -i -selection c -rmlastnl\"", content).as_str(),
                ]);
                let result = cmd.output();
                if result.is_err() {
                    Term::fatal(
                        "Failed to copy content to clipboard. Check if xclip installed properly.",
                    );
                    exit(1);
                }
            }
            SessionType::Wayland => {
                let mut cmd = Command::new("wl-copy");
                cmd.arg(format!("\"{}\"", content).as_str());
                let copy_result = cmd.output();
                if copy_result.is_err() {
                    Term::fatal("Failed to write content to clipboard. Check if wl-clipboard installed properly.");
                    exit(1);
                }
            }
        }
    }

    #[allow(unused_assignments)]
    pub fn get_clipboard() -> String {
        let session_type: SessionType = Platform::get_session_type();
        let mut buffer_content: String = String::new();
        match session_type {
            SessionType::Unknown => {
                let system_type: CurrentPlatform = Platform::detect_platform();
                match system_type {
                    CurrentPlatform::Windows => {
                        let mut cmd = Command::new("powershell.exe");
                        cmd.args(vec!["get-clipboard"]);
                        let result = cmd.output();
                        if result.is_err() {
                            Term::fatal("Failed to get content with PowerShell.");
                            exit(1);
                        }
                        buffer_content = String::from_utf8_lossy(&result.unwrap().stdout).to_string();
                    }
                    CurrentPlatform::Mac => {
                        let mut cmd = Command::new("pbpaste");
                        let result = cmd.output();
                        if result.is_err() {
                            Term::fatal("Failed to copy content to clipboard via pbcopy.");
                            exit(1);
                        }
                        buffer_content = String::from_utf8_lossy(&result.unwrap().stdout).to_string();
                    }
                    CurrentPlatform::Linux => {
                        Term::fatal("Kamiya cause in stupid situation.");
                        Term::fatal("Kamiya thinks that you are using NON UNIX PLATFORM but goes to the LINUX ENTRY FOR COPY.");
                        Term::fatal("What?...");
                        exit(1);
                    }
                    CurrentPlatform::Unknown => {
                        Term::fatal("Detected unknown platform. Cannot continue.");
                        exit(1);
                    }
                }
            }
            SessionType::X11 => {
                let mut cmd = Command::new("xclip");
                cmd.args(vec!["-o", "-selection", "c", "-rmlastnl"]);
                let result = cmd.output();
                if result.is_err() {
                    Term::fatal(
                        "Failed to get clipboard content. Check if xclip installed properly.",
                    );
                    exit(1);
                }

                buffer_content = String::from_utf8_lossy(&result.unwrap().stdout).to_string();
            }
            SessionType::Wayland => {
                let mut cmd = Command::new("wl-paste");
                let result = cmd.output();
                if result.is_err() {
                    Term::fatal(
                        "Failed to get clipboard content. Check if wl-clipboard installed properly.",
                    );
                    exit(1);
                }

                buffer_content = String::from_utf8_lossy(&result.unwrap().stdout).to_string();
            }
        }

        buffer_content
    }
}
