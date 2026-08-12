use eframe::egui;

const BUTTON_WIDHT: f32 = 100.0;
const BUTTON_HEIGHT: f32 = 45.0;
const BUTTON_TEXT_SIZE: f32 = 20.0;

#[derive(Default)]
struct MyApp {
    counter: i32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Frame::new()
            .fill(egui::Color32::BLACK)
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(
                        egui::RichText::new("Counter App")
                            .color(egui::Color32::WHITE)
                            .strong()
                            .size(30.0),
                    );
                });
            });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(format!("{}", self.counter))
                        .color(egui::Color32::BLUE)
                        .size(100.0),
                );

                ui.add_space(50.0);

                let spacing = ui.spacing().item_spacing.x;
                let total_btns_width = BUTTON_WIDHT * 3.0 + spacing * 2.0;

                ui.allocate_ui_with_layout(
                    egui::vec2(total_btns_width, BUTTON_HEIGHT),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        if ui
                            .add_sized(
                                [BUTTON_WIDHT, BUTTON_HEIGHT],
                                egui::Button::new(
                                    egui::RichText::new("+")
                                        .color(egui::Color32::WHITE)
                                        .size(BUTTON_TEXT_SIZE),
                                )
                                .fill(egui::Color32::BLACK),
                            )
                            .clicked()
                        {
                            self.counter += 1;
                        }

                        if ui
                            .add_sized(
                                [BUTTON_WIDHT, BUTTON_HEIGHT],
                                egui::Button::new(
                                    egui::RichText::new("Reset")
                                        .color(egui::Color32::WHITE)
                                        .size(BUTTON_TEXT_SIZE),
                                )
                                .fill(egui::Color32::BLACK),
                            )
                            .clicked()
                        {
                            self.counter = 0;
                        }

                        if ui
                            .add_sized(
                                [BUTTON_WIDHT, BUTTON_HEIGHT],
                                egui::Button::new(
                                    egui::RichText::new("-")
                                        .color(egui::Color32::WHITE)
                                        .size(BUTTON_TEXT_SIZE),
                                )
                                .fill(egui::Color32::BLACK),
                            )
                            .clicked()
                        {
                            self.counter -= 1;
                        }
                    },
                );
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui Counter Application",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
