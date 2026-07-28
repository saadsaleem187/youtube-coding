use eframe::egui;

const SPACING_TOP: f32 = 10.0;

#[derive(PartialEq)]
enum Position {
    First,
    Second,
    Third,
}

struct MyApp {
    username: String,
    dark_mode: bool,
    position: Position,
    age: u32,
}

impl MyApp {
    fn new() -> Self {
        Self {
            username: String::new(),
            dark_mode: true,
            position: Position::First,
            age: 1,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Heading Widget
            ui.heading("Widgets Demo");
            ui.heading(
                egui::RichText::new("Widgets Demo")
                    .color(egui::Color32::RED)
                    .size(30.0),
            );
            ui.separator();

            ui.add_space(SPACING_TOP);

            // Label Widget
            ui.label("Enter your username");
            // Input field widget
            ui.text_edit_singleline(&mut self.username);

            ui.add_space(SPACING_TOP);
            ui.label(format!("Your username is: {}", self.username));

            ui.separator();

            ui.spinner();

            ui.separator();

            ui.add_space(SPACING_TOP);

            ui.hyperlink("https://www.youtube.com");
            ui.hyperlink_to("Youtube", "https://www.youtube.com");

            ui.add_space(SPACING_TOP);

            ui.checkbox(&mut self.dark_mode, "Dark Mode");

            ui.add_space(SPACING_TOP);

            ui.radio_value(&mut self.position, Position::First, "First");
            ui.radio_value(&mut self.position, Position::Second, "Second");
            ui.radio_value(&mut self.position, Position::Third, "Third");

            ui.add_space(SPACING_TOP);

            ui.add(egui::Slider::new(&mut self.age, 1..=100).text("Age"));

            if ui.button("Submit").clicked() {
                println!("Submit button clicked");
            };

            ui.add_space(SPACING_TOP);

            if ui
                .add(
                    egui::Button::new(egui::RichText::new("Submit").color(egui::Color32::WHITE))
                        .fill(egui::Color32::BLUE),
                )
                .clicked()
            {
                println!("Name: {}, Age: {}", self.username, self.age);
            };
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Widgets",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}
