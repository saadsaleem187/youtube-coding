use eframe::egui;
use egui_todo_app::{app::MyApp, db::Database};

fn main() -> eframe::Result<(), eframe::Error> {
    let database = Database::new().expect("Failed to initialize the database");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 650.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Todo App",
        native_options,
        Box::new(move |cc| Ok(Box::new(MyApp::new(&cc.egui_ctx, database)))),
    )
}
