use std::{fs, path::Path, process::exit};
use serde::{Serialize, Deserialize};
use home::home_dir;

use crate::term::Term;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub name_template: String,
    pub editor: String
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub options: Options,
    pub entries: Vec<Note>
}

impl Default for Config {
    fn default() -> Self {
        Config { 
            options: Options { 
                name_template: "NewNote&i".to_string(), 
                editor: "nano".to_string() 
            },
            entries: vec![] 
        }
    }
}

impl Config {
    pub fn note_exists(&self, name: &str) -> bool {
        if self.entries.iter().any(|item| item.name == *name.to_owned()) {
            return true;
        }
        false
    }

    pub fn get_note_index(&self, name: &str) -> usize {
        return self.entries.iter().position(|item| item.name == *name.to_owned()).expect("Note not found!");
    }

    pub fn get_note_by_name(&self, name: &str) -> &Note {
        let index: usize = self.get_note_index(name);
        return self.entries.get(index).expect("Failed to find required note.");
    }

    pub fn generate_name(&self) -> String {
        if !self.options.name_template.contains("&i") {
            Term::fatal("You give empty name and your `name_template` option in config not contain `&i` symbol. Cannot continue.");
            exit(1);
        }
        let note_number = self.entries.len() + 1;
        let new_name: String = self.options.name_template.replace("&i", &note_number.to_string());
        new_name
    }
}

pub struct Manager;
impl Manager {
    pub fn get_config_path() -> String {
        home_dir().expect("Unable to retrieve the path to the user's home directory.").display().to_string() + "/.config/kamiya.yml"
    }

    pub fn load_config() -> Config {
        let content = fs::read_to_string(Self::get_config_path()).expect("Unable to read the file.");
        let cfg: Config = serde_yaml::from_str(&content).expect("Error when parsing the configuration file.");
        cfg
    }

    pub fn write_config(cfg: Config) {
        let config_string = serde_yaml::to_string(&cfg).expect("Error when parsing the configuration file.");
        fs::write(Self::get_config_path(), config_string).expect("Unable to write data to file.");
    }

    pub fn check_config() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }

    pub fn make_default() {
        let config_dir = home_dir().expect("Unable to retrieve the path to the user's home directory.").display().to_string() + "/.config";
        if !Path::new(&config_dir).exists() {
            fs::create_dir(config_dir).expect("Unable to create a directory.");
        }

        if !Path::new(&Self::get_config_path()).exists() {
            let config: Config = Config { ..Default::default() };
            let content = serde_yaml::to_string(&config).expect("Error when parsing the configuration file.");
            fs::write(Self::get_config_path(), content).expect("Unable to write data to file.");
        }
    }
}
