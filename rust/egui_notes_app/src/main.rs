use eframe::egui;
use egui_notes_app::{app::NotesApp, db::Database};

fn main() -> eframe::Result<()> {
    let database = Database::new().expect("Failed to initialize the database");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 750.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
}
