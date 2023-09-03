use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Options {
    name_template: String,
    editor: String,
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

    pub fn set_template(&mut self, new_template: &str) {
        self.options.name_template = String::from(new_template);
    }

    pub fn get_editor(&self) -> String {
        self.options.editor.clone()
    }

    pub fn set_editor(&mut self, editor: &str) {
        self.options.editor = String::from(editor);
    }
}
