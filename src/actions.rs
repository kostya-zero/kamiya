use std::{process::{exit, Command, Stdio}, fs, env, path::Path};
use crate::{config::{Manager, Config, Note}, utils::Utils, term::Term};

pub struct Actions;

impl Actions {
    pub fn take(content: &String, name: &String) {
        let mut config: Config = Manager::load_config();
        let mut new_name: String = name.to_string();

        if name.is_empty() {
            if !config.options.name_template.contains("&i") {
                Term::fatal("You give empty name and your `name_template` option in config not contain `&i` symbol. Cannot continue.");
                exit(1);
            }
            let note_number = config.entries.len() + 1;
            new_name = config.options.name_template.replace("&i", &note_number.to_string());
        }

        if config.note_exists(name) {
            Term::fatal("Note with same name already exists!");
            exit(1);
        }

        let new_note: Note = Note { name: new_name.clone(), content: content.to_string() };
        config.entries.push(new_note);
        Manager::write_config(config);
        Term::message(format!("Note have been recorded to storage as '{}'.", new_name).as_str());
    }

    pub fn record(filename: &str, name: &str) {
        let mut config: Config = Manager::load_config();
        let mut new_name: String = String::new();

        if !Path::new(filename).exists() {
            Term::fatal("File not found!");
            exit(1)
        }

        if name.is_empty() {
            new_name = config.generate_name();
        }

        if !name.is_empty() {
            if config.note_exists(name) {
                new_name = config.generate_name();
            } else {
                new_name = name.to_string();
            }
        }

        let file_content: String = fs::read_to_string(filename).expect("Failed to read file.");
        let new_note: Note = Note { name: new_name.clone(), content: file_content };
        config.entries.push(new_note);
        Manager::write_config(config);
        Term::message(format!("Note have been recorded to storage as '{}'.", new_name).as_str());
    }

    pub fn list() {
        let config: Config = Manager::load_config();
        if config.entries.is_empty() {
            Term::fatal("No note added to storage.");
            exit(1);
        }

        Term::message(format!("Total notes: {}", config.entries.len()).as_str());
        for i in &config.entries {
            Term::sub_message(&i.name);
        }
    }

    pub fn save(name: &str, filename: &String) {
        let config: Config = Manager::load_config();

        if !config.note_exists(name) {
            Term::fatal("Note not found!");
            exit(1);
        }

        let note_number = &config.entries.iter().position(|p| p.name == *name.to_owned()).unwrap();
        let note = &config.entries[*note_number];
        fs::write(filename.clone(), &note.content).expect("Failed to write note content into file.");
        Term::message(format!("Note content saved as file called '{}'.", filename).as_str());
    }

    pub fn edit(name: &String) {
        let mut config: Config = Manager::load_config();

        if !config.note_exists(name) {
            Term::fatal("Note not found!");
            exit(1);
        }

        let note_number = &config.entries.iter().position(|p| p.name == name.clone()).unwrap();
        let temp_dir: String = Utils::get_temp_dir();
        let temp_note_path: String = format!("{}{}",&temp_dir ,&name);
        fs::write(&temp_note_path, &config.entries[*note_number].content).expect("Error");
        let mut editor_name: String = config.options.editor.to_string();
        if editor_name.is_empty() {
            if env::var("EDITOR").is_err() {
                Term::fatal("Editor not specified! Set 'editor' option in config or set EDITOR environment variable.");
                exit(1);
            }

            editor_name = env::var("EDITOR").unwrap();
        }

        match editor_name.as_str() {
            "nvim" => Term::message("Launching Neovim to edit note..."),
            "vim" => Term::message("Launching Vim to edit note..."),
            "nano" => Term::message("Launching Nano to edit note..."),
            "gnome-text-editor" => Term::message("Launching GNOME Text Editor to edit note..."),
            "kate" => Term::message("Launching Kate to edit note..."),
            _ => Term::message("Launching editor to edit note...")
        }
        
        let status = Command::new(editor_name)
            .args([&temp_note_path])
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to launch editor. Might be executable of them not exists or permision to execute denied.");

        if !status.status.success() {
            Term::fatal("Editor finished their work with bad exit code.");
            fs::remove_file(&temp_note_path).expect("Error");
            exit(1);
        }
        
        let new_content: String = fs::read_to_string(&temp_note_path).expect("Error");
        fs::remove_file(&temp_note_path).expect("Error");
        config.entries[*note_number].content = new_content;
        Manager::write_config(config);
        Term::message("Changes have been saved.");
    }

    pub fn get(name: &str) {
        let config: Config = Manager::load_config();
        let mut content: String = "".to_string();

        if !config.note_exists(name) {
            Term::fatal("Note not found!");
            exit(1);
        }

        for i in &config.entries {
            if i.name == *name.to_owned() {
                content = i.content.clone();
                break;
            }
        }

        println!("{}", content);
    }

    pub fn rm(name: &str) {
        let mut config: Config = Manager::load_config();

        if !&config.entries.iter().any(|i| i.name == *name.to_owned()) {
            Term::fatal("Note not found!");
            exit(1);
        }

        let note_number = &config.entries.iter().position(|item| item.name == *name.to_owned()).expect("Note not found!");
        config.entries.remove(*note_number);
        Manager::write_config(config);
        Term::message("Note deleted!");
    }
}
