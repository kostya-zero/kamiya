use std::process::exit;

use serde::{Serialize, Deserialize};

use crate::term::Term;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Note {
    pub name: String,
    pub content: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Database {
    notes: Vec<Note>
}
impl Database {
    pub fn note_exists(&self, name: &str) -> bool {
        self.notes
            .iter()
            .any(|item| item.name == *name.to_owned())
    }

    pub fn remove_note(&mut self, name: &str) {
        self.notes.remove(self.get_note_index(name));
    }

    pub fn get_notes(&self) -> Vec<Note> {
        self.notes.clone()
    }

    pub fn add_note(&mut self, new_note: Note) {
        self.notes.push(new_note);
    }

    pub fn notes_count(&self) -> usize {
        self.notes.len()
    }

    pub fn set_note_name(&mut self, note_name: &str, new_name: &str) {
        let index = self.get_note_index(note_name);
        self.notes[index].name = new_name.to_string();
    }

    pub fn set_note_content(&mut self, note_name: &str, new_content: &str) {
        let index = self.get_note_index(note_name);
        self.notes[index].content = new_content.to_string();
    }

    pub fn set_note_description(&mut self, note_name: &str, new_desc: &str) {
        let index = self.get_note_index(note_name);
        self.notes[index].description = new_desc.to_string();
    }

    pub fn get_note_index(&self, name: &str) -> usize {
        match self
            .notes
            .iter()
            .position(|item| item.name == *name.to_owned())
        {
            Some(index) => index,
            None => panic!("Note not found!"),
        }
    }

    pub fn get_note(&self, name: &str) -> &Note {
        let index: usize = self.get_note_index(name);
        return self
            .notes
            .get(index)
            .expect("Failed to find required note.");
    }

    pub fn generate_name(&self, template: &str) -> String {
        if !template.contains("&i") {
            Term::fatal("You give empty name and your `name_template` option in config not contain `&i` symbol. Cannot continue.");
            exit(1);
        }
        let note_number = self.notes.len() + 1;
        let new_name: String = template.replace("&i", &note_number.to_string());
        new_name
    }
}
