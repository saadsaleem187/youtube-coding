use std::time::{Duration, Instant};

use eframe::egui;

use crate::{db::Database, models::Note};

const APP_BG: egui::Color32 = egui::Color32::from_rgb(18, 18, 20);
const SIDEBAR_BG: egui::Color32 = egui::Color32::from_rgb(22, 22, 25);
const PRIMARY_BLUE: egui::Color32 = egui::Color32::from_rgb(65, 105, 225);
const INPUT_BG: egui::Color32 = egui::Color32::from_rgb(30, 30, 34);
const SELECTED_NOTE_BG: egui::Color32 = egui::Color32::from_rgb(42, 67, 135);
const EDITOR_BG: egui::Color32 = egui::Color32::from_rgb(20, 20, 22);

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

// Eframe App
impl eframe::App for NotesApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Auto Save
        if let Some(last_change) = self.last_change {
            if last_change.elapsed() >= Duration::from_secs(2) {
                self.save_current_note();
            }

            ctx.request_repaint_after(Duration::from_millis(100));
        }

        // Left Sidebar
        egui::Panel::left("left_sidebar")
            .resizable(false)
            .exact_size(220.0)
            .frame(egui::Frame::new().fill(SIDEBAR_BG).inner_margin(20.0))
            .show(ui, |ui| {
                ui.add_space(10.0);

                ui.heading(egui::RichText::new("📝 Notes").size(30.0).strong());

                ui.add_space(5.0);

                ui.label(
                    egui::RichText::new("Your thoughts, organized").color(egui::Color32::ORANGE),
                );

                ui.add_space(50.0);

                // New Note Button
                let new_note_button = egui::Button::new(
                    egui::RichText::new("+ New Note")
                        .size(15.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                )
                .min_size(egui::vec2(180.0, 42.0))
                .fill(PRIMARY_BLUE)
                .corner_radius(8.0);

                if ui.add(new_note_button).clicked() {
                    self.title = "Untitled Note".to_string();
                    self.content.clear();
                    self.create_note();
                }

                ui.add_space(30.0);

                // SEARCH
                ui.label(
                    egui::RichText::new("SEARCH")
                        .size(12.0)
                        .strong()
                        .color(egui::Color32::from_rgb(185, 185, 190)),
                );

                ui.add_space(7.0);

                ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .desired_width(180.0)
                        .hint_text("Search notes")
                        .background_color(INPUT_BG)
                        .margin(egui::Margin::symmetric(10, 8)),
                );
            });

        // Notes List
        egui::Panel::left("notes_list_panel")
            .exact_size(320.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(APP_BG)
                    .inner_margin(egui::Margin::symmetric(18, 18)),
            )
            .show(ui, |ui| {
                ui.heading(egui::RichText::new("My Notes").size(22.0).strong());

                ui.add_space(20.0);

                let notes = self.filtered_notes();

                // Scrollable notes list
                egui::ScrollArea::vertical()
                    .id_salt("notes_scroll_area")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if notes.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.add_space(200.0);

                                ui.label(
                                    egui::RichText::new("No notes found")
                                        .size(16.0)
                                        .color(egui::Color32::GRAY),
                                );
                            });

                            return;
                        }

                        for note in notes {
                            let selected = self.selected_note == Some(note.id);

                            let background = if selected {
                                SELECTED_NOTE_BG
                            } else {
                                egui::Color32::from_rgb(32, 32, 36)
                            };

                            let response = egui::Frame::new()
                                .fill(background)
                                .corner_radius(egui::CornerRadius::same(10))
                                .inner_margin(egui::Margin::symmetric(14, 10))
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());

                                    ui.label(egui::RichText::new(&note.title).strong().size(14.0));

                                    ui.add_space(4.0);

                                    let description = Self::note_description(&note.content);

                                    ui.label(
                                        egui::RichText::new(description)
                                            .size(12.0)
                                            .color(egui::Color32::from_rgb(175, 175, 180)),
                                    );
                                })
                                .response;

                            if response.interact(egui::Sense::click()).clicked() {
                                self.select_note(note.id);
                            }

                            ui.add_space(8.0);
                        }

                        ui.add_space(8.0);
                    });
            });

        // Main editor
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(EDITOR_BG).inner_margin(35.0))
            .show(ui, |ui| {
                if self.selected_note.is_none() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(200.0);

                        ui.label(egui::RichText::new("📝").size(55.0));

                        ui.add_space(15.0);

                        ui.label("Select a note");
                        ui.label("Choose a note from the list or create a new one")
                    });

                    return;
                }

                // Toolbar
                ui.horizontal(|ui| {
                    if self.saving {
                        ui.label(
                            egui::RichText::new("Saving...")
                                .color(egui::Color32::from_rgb(150, 150, 155)),
                        );
                    } else if self.last_change.is_none() {
                        ui.label(
                            egui::RichText::new("Saved")
                                .color(egui::Color32::from_rgb(70, 220, 140)),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new("Unsaved changes")
                                .color(egui::Color32::from_rgb(235, 175, 70)),
                        );
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Delete button
                        let delete_button = egui::Button::new(
                            egui::RichText::new("Delete")
                                .strong()
                                .color(egui::Color32::WHITE),
                        )
                        .min_size(egui::vec2(90.0, 36.0))
                        .fill(egui::Color32::from_rgb(190, 55, 55))
                        .corner_radius(7.0);

                        let delete_clicked = ui.add(delete_button).clicked();

                        // Pin button
                        if let Some(id) = self.selected_note {
                            let pinned = self
                                .notes
                                .iter()
                                .find(|note| note.id == id)
                                .map(|note| note.pinned)
                                .unwrap_or(false);

                            let pin_text = if pinned { "📌 Unpin" } else { "📌 Pin" };

                            let pin_button = egui::Button::new(
                                egui::RichText::new(pin_text)
                                    .strong()
                                    .color(egui::Color32::from_rgb(255, 80, 80)),
                            )
                            .min_size(egui::vec2(90.0, 36.0))
                            .fill(egui::Color32::from_rgb(45, 45, 80))
                            .corner_radius(7.0);

                            if ui.add(pin_button).clicked() {
                                self.toggle_pin(id, pinned);
                            }
                        }

                        if delete_clicked {
                            self.delete_selected();
                        }
                    });
                });

                ui.add_space(25.0);

                // Title
                let response = ui.add_sized(
                    [ui.available_width(), 48.0],
                    egui::TextEdit::singleline(&mut self.title)
                        .font(egui::TextStyle::Heading)
                        .hint_text("Note title")
                        .background_color(INPUT_BG)
                        .margin(egui::Margin::symmetric(14, 10)),
                );

                if response.changed() {
                    self.mark_changed();
                }

                ui.add_space(20.0);

                // Content / Description
                let content_width = ui.available_width();
                let content_height = (ui.available_height() - 35.0).max(150.0);

                ui.allocate_ui_with_layout(
                    egui::vec2(content_width, content_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        egui::Frame::new()
                            .fill(INPUT_BG)
                            .corner_radius(8.0)
                            .show(ui, |ui| {
                                ui.set_min_size(egui::vec2(content_width, content_height));

                                // Content Scroller
                                egui::ScrollArea::vertical()
                                    .id_salt("content_scroll_area")
                                    .auto_shrink([false, false])
                                    .show(ui, |ui| {
                                        ui.add_space(18.0);

                                        ui.horizontal(|ui| {
                                            ui.add_space(16.0);

                                            let editor_width =
                                                (ui.available_width() - 24.0).max(100.0);

                                            let content_response = ui.add_sized(
                                                [editor_width, 500.0],
                                                egui::TextEdit::singleline(&mut self.content)
                                                    .id_salt("note_content_editor")
                                                    .desired_width(editor_width)
                                                    .hint_text("Write your note here...")
                                                    .background_color(egui::Color32::TRANSPARENT)
                                                    .frame(egui::Frame::NONE)
                                                    .margin(egui::Margin::same(0)),
                                            );

                                            if content_response.changed() {
                                                self.mark_changed();
                                            }
                                        });

                                        ui.add_space(20.0);
                                    });
                            });
                    },
                );

                ui.add_space(8.0);

                ui.label(
                    egui::RichText::new("Auto-save enabled. Changes are saved automcatically")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(130, 130, 135)),
                );
            });
    }
}
