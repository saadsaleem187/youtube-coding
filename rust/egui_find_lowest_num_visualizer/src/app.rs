use eframe::egui;
use rand::RngExt;
use std::time::{Duration, Instant};

pub struct MyApp {
    values: Vec<i32>,
    current: usize,
    min_index: usize,
    finished: bool,
    playing: bool,
    last_step: Instant,
    speed_ms: u64,
}

impl MyApp {
    pub fn new() -> Self {
        let mut app = Self {
            values: Vec::new(),
            current: 0,
            min_index: 0,
            finished: false,
            playing: false,
            last_step: Instant::now(),
            speed_ms: 500,
        };

        app.randomize();

        app
    }

    fn randomize(&mut self) {
        let mut rng = rand::rng();
        self.values = (0..15).map(|_| rng.random_range(5..100)).collect();
        self.reset();
    }

    fn reset(&mut self) {
        self.current = 0;
        self.min_index = 0;
        self.finished = false;
        self.playing = false;
    }

    fn step(&mut self) {
        if self.finished {
            return;
        }

        self.current += 1;

        if self.current >= self.values.len() {
            self.current = self.values.len() - 1;
            self.finished = true;
            self.playing = false;

            return;
        }

        if self.values[self.current] <= self.values[self.min_index] {
            self.min_index = self.current;
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        // Auto play timer
        if self.playing {
            if self.last_step.elapsed() >= Duration::from_millis(self.speed_ms) {
                self.step();
                self.last_step = Instant::now();
            }

            ui.ctx().request_repaint();
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Linear scan for the minimum");
            ui.add_space(6.0);

            // Controls
            ui.horizontal(|ui| {
                if ui
                    .button(if self.playing { "Pause" } else { "Play" })
                    .clicked()
                {
                    self.playing = !self.playing;
                    self.last_step = Instant::now();
                }

                if ui.button("Step").clicked() {
                    self.step();
                }

                if ui.button("Reset").clicked() {
                    self.reset();
                }

                if ui.button("New array").clicked() {
                    self.randomize();
                }

                ui.label("Speed:");

                ui.add(
                    egui::Slider::new(&mut self.speed_ms, 50..=1500)
                        .suffix(" ms")
                        .logarithmic(true),
                );
            });

            ui.add_space(5.0);

            let status = if self.finished {
                format!(
                    "Done! Lowest number is {} at index {}",
                    self.values[self.min_index], self.min_index,
                )
            } else {
                format!(
                    "Checking index {} (value {}) - lowest so far {} at index {}",
                    self.current,
                    self.values[self.current],
                    self.values[self.min_index],
                    self.min_index,
                )
            };

            ui.label(egui::RichText::new(status).size(16.0).strong());

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                legend(ui, egui::Color32::from_rgb(80, 160, 235), "Not checked yet");
                legend(
                    ui,
                    egui::Color32::from_rgb(150, 150, 150),
                    "Already checked",
                );
                legend(ui, egui::Color32::from_rgb(240, 160, 40), "Current");
                legend(ui, egui::Color32::from_rgb(60, 190, 110), "Lowest so far");
            });

            ui.add_space(10.0);

            // Bar chart
            let available = ui.available_size();
            let (rect, _) = ui.allocate_exact_size(available, egui::Sense::hover());
            let painter = ui.painter_at(rect);

            let n = self.values.len();
            let max_value = *self.values.iter().max().unwrap_or(&1) as f32;
            let gap = 6.0;
            let bar_w = (rect.width() - gap * (n as f32 + 1.0)) / n as f32;
            let chart_h = rect.height() - 30.0;

            for (i, &v) in self.values.iter().enumerate() {
                let h = (v as f32 / max_value) * (chart_h - 20.0);
                let x = rect.left() + gap + i as f32 * (bar_w + gap);
                let y = rect.top() + chart_h - h;

                let color = if i == self.min_index {
                    egui::Color32::from_rgb(60, 190, 110)
                } else if i == self.current && !self.finished {
                    egui::Color32::from_rgb(240, 160, 40)
                } else if i < self.current {
                    egui::Color32::from_rgb(150, 150, 150)
                } else {
                    egui::Color32::from_rgb(80, 160, 235)
                };

                let bar = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(bar_w, h));

                painter.rect_filled(bar, 3.0, color);

                // Value above the bar
                painter.text(
                    egui::pos2(x + bar_w / 2.0, y - 4.0),
                    egui::Align2::CENTER_BOTTOM,
                    v.to_string(),
                    egui::FontId::proportional(13.0),
                    ui.visuals().text_color(),
                );

                // Index below the bar
                painter.text(
                    egui::pos2(x + bar_w / 2.0, rect.top() + chart_h + 6.0),
                    egui::Align2::CENTER_TOP,
                    i.to_string(),
                    egui::FontId::proportional(12.0),
                    ui.visuals().weak_text_color(),
                );
            }
        });
    }
}

fn legend(ui: &mut egui::Ui, color: egui::Color32, label: &str) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());

    ui.painter().rect_filled(rect, 3.0, color);

    ui.label(label);

    ui.add_space(8.0);
}
