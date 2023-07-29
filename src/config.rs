use crate::term::Term;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, process::exit};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Note {
    pub name: String,
    pub content: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub name_template: String,
    pub editor: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    options: Options,
    entries: Vec<Note>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            options: Options {
                name_template: String::from("Note&i"),
                editor: String::from("nano"),
            },
            entries: vec![],
        }
    }
}

impl Config {
    pub fn note_exists(&self, name: &str) -> bool {
        self.entries
            .iter()
            .any(|item| item.name == *name.to_owned())
    }

    pub fn remove_note(&mut self, name: &str) {
        self.entries.remove(self.get_note_index(name));
    }

    pub fn get_notes(&self) -> Vec<Note> {
        self.entries.clone()
    }

    pub fn add_note(&mut self, new_note: Note) {
        self.entries.push(new_note);
    }

    pub fn get_template(&self) -> String {
        self.options.name_template.clone()
    }

    pub fn get_editor(&self) -> String {
        self.options.editor.clone()
    }

    pub fn set_editor(&mut self, editor: &str) {
        self.options.editor = String::from(editor);
    }

    pub fn notes_count(&self) -> usize {
        self.entries.len()
    }

    pub fn set_note_name(&mut self, note_name: &str, new_name: &str) {
        let index = self.get_note_index(note_name);
        self.entries[index].name = new_name.to_string();
    }

    pub fn set_note_content(&mut self, note_name: &str, new_content: &str) {
        let index = self.get_note_index(note_name);
        self.entries[index].content = new_content.to_string();
    }

    pub fn set_note_description(&mut self, note_name: &str, new_desc: &str) {
        let index = self.get_note_index(note_name);
        self.entries[index].description = new_desc.to_string();
    }

    pub fn get_note_index(&self, name: &str) -> usize {
        match self
            .entries
            .iter()
            .position(|item| item.name == *name.to_owned()) {
                Some(index) => index,
                None => panic!("Note not found!"),
            }
    }

    pub fn get_note(&self, name: &str) -> &Note {
        let index: usize = self.get_note_index(name);
        return self
            .entries
            .get(index)
            .expect("Failed to find required note.");
    }

    pub fn generate_name(&self) -> String {
        if !self.get_template().contains("&i") {
            Term::fatal("You give empty name and your `name_template` option in config not contain `&i` symbol. Cannot continue.");
            exit(1);
        }
        let note_number = self.entries.len() + 1;
        let new_name: String = self.get_template().replace("&i", &note_number.to_string());
        new_name
    }
}

pub struct Manager;
impl Manager {
    pub fn get_config_path() -> String {
        home_dir()
            .expect("Unable to retrieve the path to the user's home directory.")
            .display()
            .to_string()
            + "/.config/kamiya.yml"
    }

    pub fn load_config() -> Config {
        let content =
            fs::read_to_string(Self::get_config_path()).expect("Unable to read the file.");
        match serde_yaml::from_str(&content) {
            Ok(cfg) => cfg,
            Err(_) => panic!("Failed to parse configuration file."),
        }
    }

    pub fn write_config(cfg: Config) {
        let config_string =
            serde_yaml::to_string(&cfg).expect("Error when parsing the configuration file.");
        fs::write(Self::get_config_path(), config_string).expect("Unable to write data to file.");
    }

    pub fn check_config() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }

    pub fn make_default() {
        let config_dir = home_dir()
            .expect("Unable to retrieve the path to the user's home directory.")
            .display()
            .to_string()
            + "/.config";
        if !Path::new(&config_dir).exists() {
            fs::create_dir(config_dir).expect("Unable to create a directory.");
        }

        if !Path::new(&Self::get_config_path()).exists() {
            let config: Config = Config {
                ..Default::default()
            };
            let content =
                serde_yaml::to_string(&config).expect("Error when parsing the configuration file.");
            fs::write(Self::get_config_path(), content).expect("Unable to write data to file.");
        }
    }
}
