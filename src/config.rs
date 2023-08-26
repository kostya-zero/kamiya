
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Options {
    pub name_template: String,
    pub editor: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    options: Options,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            options: Options {
                name_template: String::from("Note&i"),
                editor: String::from("nano"),
            },
        }
    }
}

impl Config {
    pub fn get_template(&self) -> String {
        self.options.name_template.clone()
    }
    
    pub fn get_editor(&self) -> String {
        self.options.editor.clone()
    }

    pub fn set_editor(&mut self, editor: &str) {
        self.options.editor = String::from(editor);
    }
}

