use eframe::egui;

#[derive(Default)]
struct MyApp {
    dark_mode: bool,
    show_settings: bool,
}

impl MyApp {
    fn setup_theme(&self, ctx: &egui::Context) {
        // Dark/Light Mode
        let mut visuals = if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        // Background
        visuals.panel_fill = if self.dark_mode {
            egui::Color32::DARK_GRAY
        } else {
            egui::Color32::LIGHT_GRAY
        };

        // Window
        visuals.window_fill = if self.dark_mode {
            egui::Color32::GRAY
        } else {
            egui::Color32::WHITE
        };

        // Widget Background
        visuals.widgets.inactive.bg_fill = egui::Color32::LIGHT_BLUE;
        visuals.widgets.hovered.bg_fill = egui::Color32::BLUE;
        visuals.widgets.active.bg_fill = egui::Color32::DARK_BLUE;

        ctx.set_visuals(visuals);

        // Styles
        ctx.all_styles_mut(|style| {
            // Spacing
            style.spacing.item_spacing = egui::vec2(22.0, 22.0);
            style.spacing.button_padding = egui::vec2(26.0, 20.0);

            // Text Style
            style.text_styles.insert(
                egui::TextStyle::Heading,
                egui::FontId::new(30.0, egui::FontFamily::Monospace),
            );
            style.text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::new(50.0, egui::FontFamily::Monospace),
            );
        });
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.setup_theme(ui.ctx());

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("egui Styling and Themes");

            ui.add_space(20.0);

            ui.label("This is the default egui theme.");

            ui.add_space(30.0);

            if ui.add(egui::Button::new("Settings")).clicked() {
                self.show_settings = true;
            }

            if ui.add(egui::Button::new("Settings")).clicked() {
                self.show_settings = true;
            }

            if ui.add(egui::Button::new("Settings")).clicked() {
                self.show_settings = true;
            }

            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);

                ui.label("This text is red in color");

                if ui.button("Styled button").clicked() {
                    println!("Styled button clicked");
                }
            });

            if self.show_settings {
                egui::Window::new("Settings")
                    .collapsible(false)
                    .open(&mut self.show_settings)
                    .show(ui, |ui| {
                        ui.checkbox(&mut self.dark_mode, "Dark Mode");
                    });
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui Styling and Themes",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
