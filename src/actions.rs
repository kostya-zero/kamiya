use crate::{
    config::{Config, Manager, Note},
    term::Term,
    utils::clipboard::Clipboard,
    utils::platform::Platform,
};
use std::{
    env, fs,
    path::Path,
    process::{exit, Command, Stdio},
};

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
            new_name = config.generate_name();
        }

        if config.note_exists(name) {
            Term::fatal("Note with same name already exists!");
            exit(1);
        }

        Term::work("Recording note to database...");
        let new_note: Note = Note {
            name: new_name.clone(),
            content: content.to_string(),
            description: Some(String::new()),
        };
        config.entries.push(new_note);
        Manager::write_config(config);
        Term::success(format!("Note have been recorded to storage as '{}'.", new_name).as_str());
    }

    pub fn desc(name: &str, desc: &str) {
        let mut config: Config = Manager::load_config();

        if !config.note_exists(name) {
            Term::fatal("Note with given name not found.");
            exit(1);
        }

        let note_index: usize = config.get_note_index(name);
        config.entries[note_index].description = Some(desc.to_string());
        Term::work("Writing changes to database...");
        Manager::write_config(config);
        Term::success("Description changed.");
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

        Term::work("Recording note to database...");
        let file_content: String = fs::read_to_string(filename).expect("Failed to read file.");
        let new_note: Note = Note {
            name: new_name.clone(),
            content: file_content,
            description: Some(String::new()),
        };
        config.entries.push(new_note);
        Manager::write_config(config);
        Term::success(format!("Note have been recorded to storage as '{}'.", new_name).as_str());
    }

    pub fn list() {
        let config: Config = Manager::load_config();
        if config.entries.is_empty() {
            Term::fatal("No note added to storage.");
            exit(1);
        }

        Term::title(format!("Total notes: {}", config.entries.len()).as_str());
        for i in &config.entries {
            if i.description.is_none() {
                Term::sub_message(&i.name);
            } else {
                Term::sub_message(
                    format!("{} \x1b[0m{}", i.name, i.description.clone().unwrap()).as_str(),
                );
            }
        }
    }

    pub fn save(name: &str, filename: &String) {
        let config: Config = Manager::load_config();

        if !config.note_exists(name) {
            Term::fatal("Note not found!");
            exit(1);
        }

        Term::work("Writing note content to file...");
        let note_number = &config
            .entries
            .iter()
            .position(|p| p.name == *name.to_owned())
            .unwrap();
        let note = &config.entries[*note_number];
        let res = fs::write(filename.clone(), &note.content);
        match res {
            Ok(s) => {
                Term::success("Done.");
            }
            Err(err) => {
                Term::fatal("Failed to write to file. Maybe permissions issue?");
                exit(1);
            }
        }
        Term::success(format!("Note content saved as file called '{}'.", filename).as_str());
    }

    pub fn edit(name: &String) {
        let mut config: Config = Manager::load_config();

        if !config.note_exists(name) {
            Term::fatal("Note not found!");
            exit(1);
        }

        let note_number = config.get_note_index(name);
        let temp_note_path: String = format!("{}{}", Platform::get_temp_dir(), &name);
        fs::write(&temp_note_path, &config.entries[note_number].content).expect("Error");
        let mut editor_name: String = config.options.editor.to_string();
        if editor_name.is_empty() {
            if env::var("EDITOR").is_err() {
                Term::fatal("Editor not specified! Set 'editor' option in config or set EDITOR environment variable.");
                exit(1);
            }
            editor_name = env::var("EDITOR").expect("Cannot get environment variable.");
        }

        match editor_name.as_str() {
            "nvim" => Term::work("Launching Neovim to edit note..."),
            "vim" => Term::work("Launching Vim to edit note..."),
            "nano" => Term::work("Launching Nano to edit note..."),
            "gnome-text-editor" => Term::work("Launching GNOME Text Editor to edit note..."),
            "kate" => Term::work("Launching Kate to edit note..."),
            _ => Term::work("Launching editor to edit note..."),
        }

        let mut cmd = Command::new(editor_name);
        cmd.args([&temp_note_path])
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit());

        let result = cmd.output();

        if result.is_err() {
            Term::fatal("Failed to launch editor!");
            exit(1);
        }

        let status = result.unwrap();

        if !status.status.success() {
            Term::fatal("Editor finished their work with bad exit code.");
            fs::remove_file(&temp_note_path).expect("Error");
            exit(1);
        }

        Term::work("Recording changes...");
        let new_content: String = fs::read_to_string(&temp_note_path).expect("Error");
        fs::remove_file(&temp_note_path).expect("Error");
        config.entries[note_number].content = new_content;
        Manager::write_config(config);
        Term::success("Changes have been saved.");
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

        let note_number = config.get_note_index(name);
        config.entries.remove(note_number);
        Manager::write_config(config);
        Term::success("Note deleted!");
    }

    pub fn export() {
        let config: Config = Manager::load_config();

        Term::work("Exporting database...");
        let backup_config = serde_yaml::to_string(&config).expect("Failed to format config.");
        fs::write("kamiya_exported.yml", backup_config).expect("Failed to write content to file.");
        Term::success("Database exported as 'kamiya_exported.yml'.");
    }

    pub fn import(filename: &str) {
        let mut config: Config = Manager::load_config();

        if !Path::new(filename).exists() {
            Term::fatal("Cant find new database.");
            exit(1);
        }

        Term::work("Getting new database content...");
        let new_db: String = fs::read_to_string(filename).expect("Failed to read file.");
        let new_config: Config = serde_yaml::from_str(new_db.as_str())
            .expect("Failed to import notes. Maybe, bad config formatting.");
        Term::work("Importing...");
        for i in new_config.entries {
            if config.note_exists(&i.name) {
                Term::warn(
                    format!(
                        "Note with name '{}' already exists in database.",
                        &i.name.clone()
                    )
                    .as_str(),
                );
            } else {
                Term::work(format!("Adding new note: {}", &i.name.clone()).as_str());
                config.entries.push(i);
            }
        }
        Term::work("Writing database changes...");
        let config_content: String =
            serde_yaml::to_string(&config).expect("Failed to format config.");
        fs::write(Manager::get_config_path(), config_content)
            .expect("Failed to write content to file.");
        Term::success("New notes has been imported.");
    }

    pub fn copy(name: &str) {
        let config: Config = Manager::load_config();
        let note: &Note = config.get_note_by_name(name);
        Clipboard::set_clipboard(&note.content);
        Term::success("Copied to the clipboard.");
    }

    pub fn insert() {
        let mut config: Config = Manager::load_config();
        let clipboard_content: String = Clipboard::get_clipboard();
        let note_name: String = config.generate_name();
        let new_note: Note = Note {
            name: note_name.clone(),
            content: clipboard_content,
            description: Some(String::new()),
        };
        config.entries.push(new_note);
        Manager::write_config(config);
        Term::success(format!("Clipboard content saved as note called '{}'", note_name).as_str());
    }
}
