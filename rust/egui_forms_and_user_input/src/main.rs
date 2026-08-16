use eframe::egui;

const FIELD_MARGIN: f32 = 10.0;

#[derive(Default)]
struct MyApp {
    name: String,
    email: String,
    age: u32,
    country: String,
    subscribe: bool,
    submitted: bool,
}

impl MyApp {
    fn validate(&self) -> bool {
        !self.name.trim().is_empty()
            && !self.email.trim().is_empty()
            && !self.country.trim().is_empty()
    }
}
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Frame::new()
            .inner_margin(20.0)
            .fill(egui::Color32::DARK_GRAY)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(
                        egui::RichText::new("User Profile")
                            .strong()
                            .size(30.0)
                            .color(egui::Color32::WHITE),
                    );
                });
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.add_space(20.0);
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.name);

            ui.add_space(FIELD_MARGIN);

            ui.label("Email:");
            ui.text_edit_singleline(&mut self.email);

            ui.add_space(FIELD_MARGIN);

            ui.add(egui::Slider::new(&mut self.age, 1..=100).text("Age"));

            ui.add_space(FIELD_MARGIN);

            egui::ComboBox::from_label("Country")
                .selected_text(if self.country.is_empty() {
                    "Select Country"
                } else {
                    &self.country
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.country, "Pakistan".to_string(), "Pakistan");
                    ui.selectable_value(&mut self.country, "India".to_string(), "India");
                    ui.selectable_value(
                        &mut self.country,
                        "United States".to_string(),
                        "United States",
                    );
                    ui.selectable_value(
                        &mut self.country,
                        "United Kingdom".to_string(),
                        "United Kingdom",
                    );
                });

            ui.add_space(FIELD_MARGIN);

            ui.checkbox(&mut self.subscribe, "Subscribe to newsletter");

            if ui.button("Submit").clicked() {
                if self.validate() {
                    self.submitted = true;
                }
            }

            if self.submitted {
                ui.add_space(50.0);
                ui.heading("Submitted Information");
                ui.label(format!("Name: {}", self.name));
                ui.label(format!("Email: {}", self.email));
                ui.label(format!("Age: {}", self.age));
                ui.label(format!("Country: {}", self.country));
                ui.label(format!(
                    "Newsletter: {}",
                    if self.subscribe { "Yes" } else { "No" }
                ));
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui User forms and input",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
