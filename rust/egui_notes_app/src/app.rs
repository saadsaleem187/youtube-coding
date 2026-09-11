use std::time::Instant;

use eframe::egui;

use crate::{db::Database, models::Note};

pub struct NotesApp {
    db: Database,
    notes: Vec<Note>,

    selected_note: Option<i64>,

    title: String,
    content: String,

    search: String,

    last_change: Option<Instant>,
    saving: bool,
}

impl NotesApp {
    pub fn new(db: Database) -> Self {
        let mut app = Self {
            db,
            notes: Vec::new(),

            selected_note: None,

            title: String::new(),
            content: String::new(),

            search: String::new(),

            last_change: None,
            saving: false,
        };

        app.load_notes();

        app
    }

    fn load_notes(&mut self) {
        match self.db.get_notes() {
            Ok(notes) => {
                self.notes = notes;
            }

            Err(error) => {
                eprintln!("Failed to load notes: {error}");
            }
        }
    }

    fn create_note(&mut self) {
        let title = if self.title.trim().is_empty() {
            "Untitled Note"
        } else {
            self.title.trim()
        };

        match self.db.create_note(title, &self.content) {
            Ok(id) => {
                self.load_notes();
                self.select_note(id);
            }

            Err(error) => {
                eprintln!("Failed to create a note: {error}");
            }
        }
    }

    fn select_note(&mut self, id: i64) {
        if let Some(note) = self.notes.iter().find(|note| note.id == id) {
            self.selected_note = Some(note.id);

            self.title = note.title.clone();
            self.content = note.content.clone();

            self.last_change = None;
            self.saving = false;
        }
    }

    fn save_current_note(&mut self) {
        let Some(id) = self.selected_note else {
            return;
        };

        self.saving = true;

        match self.db.update_note(id, &self.title, &self.content) {
            Ok(_) => {
                self.load_notes();
                self.last_change = None;
            }

            Err(error) => {
                eprintln!("Failed to save the note: {error}");
            }
        }

        self.saving = false;
    }

    fn delete_selected(&mut self) {
        let Some(id) = self.selected_note else {
            return;
        };

        match self.db.delete_note(id) {
            Ok(_) => {
                self.load_notes();
                self.selected_note = None;
                self.title.clear();
                self.content.clear();
                self.last_change = None;
                self.saving = false;
            }

            Err(error) => {
                eprintln!("Failed to delete note: {error}");
            }
        }
    }

    fn toggle_pin(&mut self, id: i64, pinned: bool) {
        match self.db.toggle_pin(id, !pinned) {
            Ok(_) => {
                self.load_notes();
                self.select_note(id);
            }

            Err(error) => {
                eprintln!("Failed to toggle pin: {error}");
            }
        }
    }

    fn mark_changed(&mut self) {
        self.last_change = Some(Instant::now())
    }

    fn filtered_notes(&self) -> Vec<Note> {
        let search = self.search.trim().to_lowercase();

        if search.is_empty() {
            return self.notes.clone();
        }

        self.notes
            .iter()
            .filter(|note| {
                note.title.to_lowercase().contains(&search)
                    || note.content.to_lowercase().contains(&search)
            })
            .cloned()
            .collect()
    }

    fn note_description(content: &str) -> String {
        let description = content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        if description.is_empty() {
            return "...".to_string();
        }

        let text = description.chars().take(60).collect::<String>();

        format!("{text}...")
    }
}
