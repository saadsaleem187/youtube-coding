use eframe::egui;
use egui_find_lowest_num_visualizer::app::MyApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Find the Lowest Number - Visualizer",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}
