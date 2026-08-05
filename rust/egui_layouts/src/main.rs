use eframe::egui;

#[derive(Default)]
struct MyApp {
    username: String,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("egui Layouts");

            // Vertical Layout
            ui.label("First");
            ui.label("Second");
            ui.label("Third");

            ui.separator();

            // Horizontal Layout
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    println!("Save button clicked");
                }
                if ui.button("Cancel").clicked() {
                    println!("Cancel button clicked");
                }
                if ui.button("Reset").clicked() {
                    println!("Reset button clicked");
                }
            });

            ui.separator();

            // Combined Layout
            ui.horizontal(|ui| {
                ui.label("Username: ");

                ui.vertical(|ui| {
                    ui.text_edit_singleline(&mut self.username);
                    ui.label(format!("Your username is: {}", self.username));
                });
            });

            ui.separator();

            // Grid Layout
            egui::Grid::new("user_info").show(ui, |ui| {
                ui.label("Name");
                ui.label("Saad");
                ui.end_row();

                ui.label("Age");
                ui.label("35");
                ui.end_row();

                ui.label("Profession");
                ui.label("Computer Programming");
                ui.end_row();
            });

            // Center Widgets
            ui.vertical_centered(|ui| {
                ui.label("Center Label");
            });

            // Alignment
            ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                if ui.button("Logout").clicked() {
                    println!("Logout button clicked");
                }
                if ui.button("About").clicked() {
                    println!("About button clicked");
                }
                if ui.button("Profile").clicked() {
                    println!("Profile button clicked");
                }
            });

            ui.label("Another widget");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Layouts",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
