use crate::{
    config::Config,
    database::{Database, Note},
    manager::Manager,
    term::{AskDefaultAnswers, Term},
    utils::tempfile::TempFile,
};
use std::{
    fs, mem,
    path::Path,
    process::{exit, Command, Stdio},
};

pub struct Actions;

impl Actions {
    pub fn take(content: &str, name: &mut String, desc: &str) {
        let config: Config = Manager::load_config();
        let mut database: Database = Manager::load_database();

        if name.is_empty() {
            if !config.get_template().contains("&i") {
                Term::fatal("You give empty name and your `name_template` option in config not contain `&i` symbol. Cannot continue.");
                exit(1);
            }
            name.push_str(&database.generate_name(&config.get_template()));
        }
        mem::forget(config);

        if database.note_exists(name) {
            Term::fatal("Note with same name already exists!");
            exit(1);
        }

        let new_note: Note = Note {
            name: name.clone(),
            content: content.to_string(),
            description: desc.to_string(),
        };

        database.add_note(new_note);
        Manager::write_database(database);
        Term::success(&format!("Note have been added to database as '{}'.", name));
    }

    pub fn desc(name: &str, desc: &str) {
        let mut database: Database = Manager::load_database();

        if !database.note_exists(name) {
            Term::fatal("Note with given name not found.");
            exit(1);
        }

        database.set_note_description(name, desc);
        Manager::write_database(database);
        Term::success("Description changed.");
    }

    pub fn add(filename: &str, name: &mut String) {
        let mut database: Database = Manager::load_database();

        if !Path::new(filename).exists() {
            Term::fatal("File not found!");
            exit(1)
        }

        if name.is_empty() {
            name.push_str(Path::new(filename).file_stem().unwrap().to_str().unwrap());
        }

        let file_content: String = fs::read_to_string(filename).expect("Failed to read file.");
        let new_note: Note = Note {
            name: name.clone(),
            content: file_content,
            description: String::new(),
        };
        database.add_note(new_note);
        Manager::write_database(database);
        Term::success(&format!("Note have been added to database as '{}'.", name));
    }

    pub fn rename(old_name: &str, new_name: &str) {
        let mut database: Database = Manager::load_database();
        if !database.note_exists(old_name) {
            Term::fatal("Cannot find note to rename");
            exit(1);
        }

        database.set_note_name(old_name, new_name);
        Manager::write_database(database);
        Term::success(&format!(
            "Note '{}' now have name '{}'.",
            old_name, new_name
        ));
    }

    pub fn editor(editor: &str) {
        let mut config: Config = Manager::load_config();

        if editor.is_empty() {
            if config.get_editor().is_empty() {
                Term::info("Editor not set. Please set name or path to executable of the editor.");
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
        let database: Database = Manager::load_database();
        let notes: Vec<Note> = database.get_notes();
        if notes.is_empty() {
            Term::fatal("Noting added to storage!");
            exit(1);
        }

        Term::title("Notes in storage:");
        for i in &notes {
            if i.description.is_empty() {
                Term::list_item(&i.name, "");
            } else {
                Term::list_item(&i.name, &i.description.clone());
            }
        }
    }

    pub fn search(pattern: &str) {
        let database: Database = Manager::load_database();
        let mut found_notes: Vec<String> = vec![];

        for i in database.get_notes().iter() {
            if i.name.contains(pattern) {
                found_notes.push(format!("\x1b[4m{}\x1b[0m\x1b[1m", pattern));
            }
        }

        Term::title(format!("Found {} notes.", found_notes.len()).as_str());
        for a in found_notes {
            Term::message(&a);
        }
    }

    pub fn save(name: &str, filename: &str) {
        let database: Database = Manager::load_database();

        if !database.note_exists(name) {
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
        let note = database.get_note(name);
        match fs::write(new_filename, &note.content) {
            Ok(_s) => {
                Term::success(
                    format!("Note content saved as file called '{}'.", filename).as_str(),
                );
            }
            Err(_err) => {
                Term::fatal("Failed to write to file. Maybe permissions issue?");
                exit(1);
            }
        }
    }

    pub fn edit(name: &str) {
        let config: Config = Manager::load_config();
        let mut database: Database = Manager::load_database();

        if !database.note_exists(name) {
            Term::fatal("Note not found!");
            exit(1);
        }

        let note = database.get_note(name);
        let tmpfile_initializer = TempFile::new(name);
        let tmpfile = match tmpfile_initializer {
            Ok(provider) => provider,
            Err(_) => {
                Term::fatal("Failed initialize temporary file due to unknown error.");
                exit(1);
            }
        };

        let tmpfile_path: &str = tmpfile.init().unwrap();
        fs::write(tmpfile_path, note.content.clone())
            .expect("Failed to write content of note to temporary file.");
        let editor_name: String = config.get_editor().to_string();
        if editor_name.is_empty() {
            Term::fatal("Edtior not set properly. Please run Kamiya with `editor` command and see if it's set or not.");
            Term::hint("If not or set not correctly, use `editor` command to specify it. Example: `kamiya editor vim`");
            exit(1);
        }

        Term::work(format!("Launching {}", editor_name).as_str());

        let mut cmd = Command::new(editor_name);
        cmd.args([tmpfile_path])
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
        let new_content: String = fs::read_to_string(tmpfile_path).expect("Error");
        tmpfile.destroy().unwrap();
        database.set_note_content(name, &new_content);
        Manager::write_database(database);
        Term::success("Changes have been saved.");
    }

    pub fn get(name: &str) {
        let database: Database = Manager::load_database();

        if !database.note_exists(name) {
            Term::fatal("Note not found!");
            exit(1);
        }
        let note = database.get_note(name);

        println!("{}", note.content.trim_end());
    }

    pub fn rm(name: &str) {
        let _config: Config = Manager::load_config();
        let mut database: Database = Manager::load_database();

        if !database.note_exists(name) {
            Term::fatal("Note not found!");
            exit(1);
        }

        database.remove_note(name);
        Manager::write_database(database);
        Term::success("Note deleted!.");
    }

    pub fn export(path: &str) {
        let database: Database = Manager::load_database();

        if Path::new(path).exists() {
            Term::fatal(&format!(
                "'{}' already exists. Specify new path or remove if its not needed.",
                path
            ));
            exit(1);
        }

        Term::work("Exporting database...");
        let backup_config = serde_json::to_string(&database).unwrap();
        match fs::write(path, backup_config) {
            Ok(_) => {
                Term::success("File saved!");
                Term::hint(&format!("Database exported as '{}'.", path));
            }
            Err(i) => {
                Term::fatal(&format!("Failed to write content to file. Error: {}", i));
            }
        }
    }

    pub fn db() {
        let database: Database = Manager::load_database();

        let file_size = fs::metadata(Manager::get_config_path())
            .expect("Failed to get metadata about config.")
            .len();
        let notes_count = database.notes_count();

        Term::title("Information about storage.");
        Term::display_data("Storage size", file_size.to_string().as_str());
        Term::display_data("Notes in storage", notes_count.to_string().as_str());
        Term::hint("Storage size are displayed as nubmer of bytes.");
    }

    pub fn import(filename: &str, replace: bool, interactive: bool) {
        let config: Config = Manager::load_config();
        let mut database: Database = Manager::load_database();

        if !Path::new(filename).exists() {
            Term::fatal("Cant find new database.");
            exit(1);
        }

        Term::work("Getting new database content...");
        let new_db_file: String = fs::read_to_string(filename).expect("Failed to read file.");
        let new_db: Database = serde_json::from_str(new_db_file.as_str())
            .expect("Failed to import notes. Maybe, bad config formatting.");
        Term::work("Importing...");
        for i in new_db.get_notes() {
            if database.note_exists(&i.name) {
                if replace {
                    Term::work(&format!(
                        "Replacing data of `{}` with from new one.",
                        &i.name
                    ));
                    database.set_note_content(&i.name, &i.content);
                    database.set_note_description(&i.name, &i.description);
                }

                if interactive {
                    let answer = Term::ask_yn(
                        &format!(
                            "Note with name `{}` found in current storage. Do you want to replace?",
                            &i.name
                        ),
                        AskDefaultAnswers::Yes,
                    );
                    match answer {
                        AskDefaultAnswers::Yes => {
                            Term::work(&format!(
                                "Replacing data of `{}` with from new one.",
                                &i.name
                            ));
                            database.set_note_content(&i.name, &i.content);
                            database.set_note_description(&i.name, &i.description);
                        }
                        AskDefaultAnswers::No => Term::warn("Skipping..."),
                    }
                }
                if !replace && !interactive {
                    Term::warn(
                        format!(
                            "Note with name '{}' already exists in database.",
                            &i.name.clone()
                        )
                        .as_str(),
                    );
                }
            } else {
                Term::work(format!("Adding new note: {}", &i.name.clone()).as_str());
                database.add_note(i);
            }
        }
        let config_content: String =
            serde_json::to_string(&config).expect("Failed to format config.");
        fs::write(Manager::get_config_path(), config_content)
            .expect("Failed to write content to file.");
        Term::success("Import finished.");
    }
}
