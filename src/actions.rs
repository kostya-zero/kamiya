use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::{
    config::{Config, Manager, Note},
    term::Term, utils::tempfile::TempFile,
};
use std::{
    env, fs,
    path::Path,
    process::{exit, Command, Stdio},
};

pub struct Actions;

impl Actions {
    pub fn take(content: &str, name: &str, desc: &str) {
        let mut config: Config = Manager::load_config();
        let mut new_name: String = String::from(name);

        if name.is_empty() {
            if !config.get_template().contains("&i") {
                Term::fatal("You give empty name and your `name_template` option in config not contain `&i` symbol. Cannot continue.");
                exit(1);
            }
            new_name = config.generate_name();
        }

        if config.note_exists(name) {
            Term::fatal("Note with same name already exists!");
            exit(1);
        }

        Term::work("Adding note to storage...");
        let new_note: Note = Note {
            name: new_name.clone(),
            content: content.to_string(),
            description: desc.to_string(),
        };
        config.add_note(new_note);
        Manager::write_config(config);
        Term::success(&format!(
            "Note have been added to storage as '{}'.",
            new_name
        ));
    }

    pub fn desc(name: &str, desc: &str) {
        let mut config: Config = Manager::load_config();

        if !config.note_exists(name) {
            Term::fatal("Note with given name not found.");
            exit(1);
        }

        config.set_note_description(name, desc);
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
            description: String::new(),
        };
        config.add_note(new_note);
        Manager::write_config(config);
        Term::success(&format!(
            "Note have been recorded to storage as '{}'.",
            new_name
        ));
    }

    pub fn rename(old_name: &str, new_name: &str) {
        let mut config: Config = Manager::load_config();
        if !config.note_exists(old_name) {
            Term::fatal("Cannot find note to rename");
            exit(1);
        }

        config.set_note_name(old_name, new_name);
        Term::work("Writing changes to storage...");
        Manager::write_config(config);
        Term::success(&format!(
            "Note '{}' now have name '{}'.",
            old_name, new_name
        ));
    }

    pub fn editor(editor: &str) {
        let mut config: Config = Manager::load_config();

        if editor.is_empty() {
            if config.get_editor().is_empty() {
                Term::info(
                    "Editor not set. Please add name of executable that you want to use as editor.",
                );
                Term::hint("Example: kamiya editor vim");
                exit(1)
            }
            Term::info(&format!("Current editor: {}", config.get_editor()));
        } else {
            config.set_editor(editor);
            Manager::write_config(config);
            Term::success(&format!("Editor changed to {}", editor));
        }
    }

    pub fn list() {
        let config: Config = Manager::load_config();
        let notes: Vec<Note> = config.get_notes();
        if notes.is_empty() {
            Term::fatal("No note added to storage.");
            exit(1);
        }

        Term::title(format!("Total notes: {}", notes.len()).as_str());
        for i in &notes {
            if i.description.is_empty() {
                Term::list_item(&i.name, "");
            } else {
                Term::list_item(&i.name, &i.description.clone());
            }
        }
    }

    pub fn search(pattern: &str) {
        let config: Config = Manager::load_config();
        let mut found_notes: Vec<String> = vec![];

        for i in config.get_notes().iter() {
            if i.name.contains(pattern) {
                found_notes.push(format!("\x1b[4m{}\x1b[0m\x1b[1m", pattern));
            }
        }

        Term::title(format!("Found {} notes.", found_notes.len()).as_str());
        for a in found_notes {
            Term::message(&a);
        }
    }

    #[allow(unused_assignments)]
    pub fn save(name: &str, filename: &String) {
        let config: Config = Manager::load_config();

        if !config.note_exists(name) {
            Term::fatal("Note not found!");
            exit(1);
        }
        let mut new_filename = String::new();
        if filename.is_empty() {
            new_filename = name.to_string() + ".txt";
        } else {
            new_filename = filename.to_string();
        }

        Term::work("Writing note content to file...");
        let note = config.get_note(name);
        let write_result: Result<(), std::io::Error> = fs::write(new_filename, &note.content);
        match write_result {
            Ok(_s) => {
                Term::success("Done.");
            }
            Err(_err) => {
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
        
        let note = config.get_note(name);
        let tmpfile: TempFile = TempFile::new(name);
        let tmpfile_path: String = tmpfile.init().unwrap();
        fs::write(&tmpfile_path, note.content.clone()).expect("Failed to write content of note to temporary file.");
        let mut editor_name: String = config.get_editor().to_string();
        if editor_name.is_empty() {
            if env::var("EDITOR").is_err() {
                Term::fatal("Editor not specified! Set 'editor' option in config or set EDITOR environment variable.");
                exit(1);
            }
            editor_name = env::var("EDITOR").expect("Cannot get environment variable.");
        }

        Term::work(format!("Launching {}", editor_name).as_str());

        let mut cmd = Command::new(editor_name);
        cmd.args([&tmpfile_path])
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
            tmpfile.destroy().unwrap();
            exit(1);
        }

        Term::work("Recording changes...");
        let new_content: String = fs::read_to_string(&tmpfile_path).expect("Error");
        tmpfile.destroy().unwrap();
        config.set_note_content(name, &new_content);
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

        let notes: Vec<Note> = config.get_notes();

        for i in notes {
            if i.name == *name.to_owned() {
                content = i.content.clone();
                break;
            }
        }

        println!("{}", content);
    }

    pub fn rm(name: &str) {
        let mut config: Config = Manager::load_config();

        if !config.get_notes().iter().any(|i| i.name == *name.to_owned()) {
            Term::fatal("Note not found!");
            exit(1);
        }

        config.remove_note(name);
        Manager::write_config(config);
        Term::success("Note deleted!");
    }

    pub fn export(path: &str) {
        let config: Config = Manager::load_config();

        if Path::new(path).exists() {
            Term::fatal(&format!(
                "'{}' already exists. Specify new path or remove if its not needed.",
                path
            ));
            exit(1);
        }

        Term::work("Exporting database...");
        let backup_config = serde_yaml::to_string(&config).expect("Failed to format config.");
        match fs::write(path, backup_config) {
            Ok(_) => Term::success("File saved!"),
            Err(i) => {
                Term::fatal(&format!("Failed to write content to file. Error: {}", i));
            }
        }
        Term::hint(&format!("Database exported as '{}'.", path));
    }

    pub fn db() {
        let config: Config = Manager::load_config();

        let file_size = fs::metadata(Manager::get_config_path())
            .expect("Failed to get metadata about config.")
            .len();
        let notes_count = config.notes_count();

        Term::title("Information about storage.");
        Term::message_with_icon(&format!("Storage size: {} bytes", file_size), "󰖡");
        Term::message_with_icon(&format!("Notes in storage: {}", notes_count), "󰏓");
        Term::hint("Storage size displayed as nubmer of bytes.");
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
        for i in new_config.get_notes() {
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
                config.add_note(i);
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
        let note: &Note = config.get_note(name);
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(note.content.to_string()).expect("Failed to set content to clipboard.");
        Term::success("Copied to the clipboard.");
    }

    pub fn insert() {
        let mut config: Config = Manager::load_config();
        let clipboard_content: String = ClipboardProvider::new().and_then(|mut ctx: ClipboardContext| ctx.get_contents()).unwrap();
        let note_name: String = config.generate_name();
        let new_note: Note = Note {
            name: note_name.clone(),
            content: clipboard_content,
            description: String::new(),
        };
        config.add_note(new_note);
        Manager::write_config(config);
        Term::success(format!("Clipboard content saved as note called '{}'", note_name).as_str());
    }
}
