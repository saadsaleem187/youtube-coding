use eframe::egui;

#[derive(Default)]
struct MyApp {
    selected_page: String,
    show_settings: bool,
    dark_mode: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let panel_frame =
            egui::Frame::side_top_panel(ui.style()).inner_margin(egui::Margin::symmetric(16, 12));

        // Top Panel
        egui::Panel::top("top_panel")
            .exact_size(50.0)
            .frame(panel_frame)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("My Application");

                    ui.separator();

                    ui.label("egui Demo");
                });
            });

        // Bottom Panel
        egui::Panel::bottom("bottom_panel")
            .frame(panel_frame)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Built with rust + egui");
                });
            });

        // Left Panel
        egui::Panel::left("left_panel")
            .resizable(false)
            .show(ui, |ui| {
                ui.add_space(10.0);

                ui.heading("Navigation");

                ui.separator();

                ui.add_space(20.0);

                if ui.button("Dashboard").clicked() {
                    self.selected_page = "Dashboard".to_string();
                }

                if ui.button("Settings").clicked() {
                    self.show_settings = true;
                }

                if ui.button("About").clicked() {
                    self.selected_page = "About".to_string();
                }
            });

        // Main Content
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);

                ui.heading(if self.selected_page.trim().is_empty() {
                    "Dashboard"
                } else {
                    &self.selected_page
                });

                ui.add_space(20.0);

                ui.label("This is the main content area");
            });
        });

        // Settings Window
        if self.show_settings {
            egui::Window::new("Settings")
                .collapsible(false)
                .open(&mut self.show_settings)
                .show(ui, |ui| {
                    ui.add_space(10.0);

                    ui.checkbox(&mut self.dark_mode, "Dark mode");

                    ui.add_space(20.0);

                    ui.label(format!(
                        "Dark Mode: {}",
                        if self.dark_mode {
                            "Enabled"
                        } else {
                            "Disabled"
                        }
                    ));
                });
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui Panels and Windows",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
