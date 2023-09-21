use std::{
    io::ErrorKind,
    process::{Command, Stdio},
};

pub enum ProcessError {
    BadExitCode,
    Interrupted,
    ExecutableNotFound,
    Unknown,
}

pub fn run_editor(editor: &str, path: &str) -> Result<(), ProcessError> {
    let mut cmd = Command::new(editor);
    cmd.arg(path);
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());
    match cmd.output() {
        Ok(res) => {
            if res.status.code().unwrap() != 0 {
                return Err(ProcessError::BadExitCode);
            }

            Ok(())
        }
        Err(e) => match e.kind() {
            ErrorKind::Interrupted => Err(ProcessError::Interrupted),
            ErrorKind::NotFound => Err(ProcessError::ExecutableNotFound),
            _ => Err(ProcessError::Unknown),
        },
    }
}
