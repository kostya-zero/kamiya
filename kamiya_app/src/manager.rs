use home::home_dir;
use kamiya_config::Config;
use kamiya_database::Database;
use std::{fs, path::Path};

pub enum ManagerError {
    BadFormat,
    WriteError,
    UnknownError,
}

pub struct Manager;
impl Manager {
    pub fn get_config_path() -> String {
        home_dir().unwrap().display().to_string() + "/.config/kamiya/config.toml"
    }

    pub fn get_old_config_path() -> String {
        home_dir().unwrap().display().to_string() + "/.config/kamiya.yaml"
    }

    pub fn get_database_path() -> String {
        home_dir().unwrap().display().to_string() + "/.config/kamiya/database.json"
    }

    pub fn get_config_dir() -> String {
        home_dir().unwrap().display().to_string() + "/.config/kamiya"
    }

    pub fn load_config() -> Config {
        let content =
            fs::read_to_string(Self::get_config_path()).expect("Unable to read the file.");
        match toml::from_str(&content) {
            Ok(cfg) => cfg,
            Err(_) => panic!("Failed to parse configuration file."),
        }
    }

    pub fn load_database() -> Database {
        let content =
            fs::read_to_string(Self::get_database_path()).expect("Unable to read the file.");
        match toml::from_str(&content) {
            Ok(cfg) => cfg,
            Err(_) => panic!("Failed to parse database file."),
        }
    }

    pub fn write_config(cfg: Config) {
        let config_string =
            toml::to_string(&cfg).expect("Error when parsing the configuration file.");
        fs::write(Self::get_config_path(), config_string).expect("Unable to write data to file.");
    }

    pub fn write_database(db: Database) {
        let config_string = toml::to_string(&db).expect("Error when parsing the database file.");
        fs::write(Self::get_database_path(), config_string).expect("Unable to write data to file.");
    }

    pub fn check_config() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }

    pub fn check_db() -> bool {
        Path::new(&Self::get_database_path()).exists()
    }

    pub fn make_default() {
        if !Path::new(&Self::get_config_dir()).exists() {
            fs::create_dir_all(Self::get_config_dir()).expect("Failed to create new directories.");
        }

        if !Path::new(&Self::get_config_path()).exists() {
            fs::write(
                Self::get_config_path(),
                toml::to_string(&Config::default()).unwrap(),
            )
            .expect("Failed to create new config file.");
        }

        if !Path::new(&Self::get_database_path()).exists() {
            fs::write(
                Self::get_database_path(),
                toml::to_string(&Database::default()).unwrap(),
            )
            .expect("Failed to create new database file.");
        }
    }
}
