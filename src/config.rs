use std::{fs, path::Path};
use serde::{Serialize, Deserialize};
use serde_json;
use home::home_dir;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name_template: String,
    pub entries: Vec<Note>,
    pub editor: String
}

impl Config {
}

pub struct Manager;
impl Manager {
    pub fn get_config_path() -> String {
        home_dir().expect("Unable to retrieve the path to the user's home directory.").display().to_string() + "/.config/kamiya.json"
    }

    pub fn load_config() -> Config {
        let content = fs::read_to_string(Self::get_config_path()).expect("Unable to read the file.");
        let cfg: Config = serde_json::from_str(&content).expect("Error when parsing the configuration file.");
        return cfg;
    }

    pub fn write_config(cfg: Config) {
        let config_string = serde_json::to_string(&cfg).expect("Error when parsing the configuration file.");
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
            let config: Config = Config { name_template: "NewNote&i".to_string(), entries: vec![], editor: "nano".to_string()};
            let content = serde_json::to_string(&config).expect("Error when parsing the configuration file.");
            fs::write(&Self::get_config_path(), content).expect("Unable to write data to file.");
        }
    }
}
