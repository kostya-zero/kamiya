use std::io::{self, Write};

pub enum AskDefaultAnswers {
    Yes,
    No,
}

pub struct Term;
impl Term {
    pub fn success(msg: &str) {
        println!("\x1b[1m\x1b[92m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn title(msg: &str) {
        println!("\x1b[1m\x1b[92m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn list_item(name: &str, desc: &str) {
        println!("  \x1b[1m {}\x1b[0m {}", name, desc);
    }

    pub fn message(msg: &str) {
        println!(" \x1b[1m 󰍡 {}\x1b[0m", msg);
    }

    pub fn hint(msg: &str) {
        println!("\x1b[1m 󰌵 {}\x1b[0m", msg);
    }

    pub fn ask_yn(msg: &str, default_answer: AskDefaultAnswers) -> AskDefaultAnswers {
        let default_answer_display = match default_answer {
            AskDefaultAnswers::Yes => "(Y/n)",
            AskDefaultAnswers::No => "(y/N)",
        };

        print!("  \x1b[1m{} {}:\x1b[0m ", msg, default_answer_display);
        io::stdout()
            .flush()
            .expect("Failed to push text to stdout.");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read stdin.");
        answer = answer.to_lowercase().trim().to_string();
        if answer.is_empty() {
            return default_answer;
        }

        match answer.as_str() {
            "y" => AskDefaultAnswers::Yes,
            "n" => AskDefaultAnswers::No,
            _ => default_answer,
        }
    }

    pub fn info(msg: &str) {
        println!("\x1b[1m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn work(msg: &str) {
        println!("\x1b[1m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn warn(msg: &str) {
        println!("\x1b[1m\x1b[93m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn fatal(msg: &str) {
        println!("\x1b[1m\x1b[91m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }
}
