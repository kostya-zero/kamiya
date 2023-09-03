use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Note {
    pub name: String,
    pub content: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Database {
    notes: Vec<Note>,
}

#[derive(Debug)]
pub enum DatabaseError {
    NoteNotFound,
    BadTemplate,
}

impl Database {
    pub fn note_exists(&self, name: &str) -> bool {
        self.notes.iter().any(|item| item.name == *name.to_owned())
    }

    pub fn remove_note(&mut self, name: &str) -> Result<(), DatabaseError> {
        match self.get_note_index(name) {
            Ok(index) => {
                self.notes.remove(index);
                Ok(())
            }
            Err(e) => Err(e),
        }
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

    pub fn set_note_name(&mut self, note_name: &str, new_name: &str) -> Result<(), DatabaseError> {
        match self.get_note_index(note_name) {
            Ok(index) => {
                self.notes[index].name = new_name.to_string();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn set_note_content(
        &mut self,
        note_name: &str,
        new_content: &str,
    ) -> Result<(), DatabaseError> {
        match self.get_note_index(note_name) {
            Ok(index) => {
                self.notes[index].content = new_content.to_string();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn set_note_description(
        &mut self,
        note_name: &str,
        new_desc: &str,
    ) -> Result<(), DatabaseError> {
        match self.get_note_index(note_name) {
            Ok(index) => {
                self.notes[index].description = new_desc.to_string();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_note_index(&self, name: &str) -> Result<usize, DatabaseError> {
        match self
            .notes
            .iter()
            .position(|item| item.name == *name.to_owned())
        {
            Some(index) => Ok(index),
            None => Err(DatabaseError::NoteNotFound),
        }
    }

    pub fn get_note(&self, name: &str) -> Result<Note, DatabaseError> {
        match self.get_note_index(name) {
            Ok(index) => match self.notes.get(index) {
                Some(i) => Ok(i.clone()),
                None => Err(DatabaseError::NoteNotFound),
            },
            Err(e) => Err(e),
        }
    }

    pub fn generate_name(&self, template: &str) -> Result<String, DatabaseError> {
        if !template.contains("&i") {
            return Err(DatabaseError::BadTemplate);
        }
        let note_number = self.notes.len() + 1;
        let new_name: String = template.replace("&i", &note_number.to_string());
        Ok(new_name)
    }
}
