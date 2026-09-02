use eframe::egui;

use crate::{db::Database, models::Todo};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

pub struct MyApp {
    db: Database,
    todos: Vec<Todo>,

    new_todo: String,
    filter: Filter,

    total_count: usize,
    active_count: usize,
    completed_count: usize,
}

impl MyApp {
    pub fn new(ctx: &egui::Context, db: Database) -> Self {
        setup_theme(ctx);

        let mut app = Self {
            db,
            todos: Vec::new(),
            new_todo: String::new(),
            filter: Filter::All,
            total_count: 0,
            active_count: 0,
            completed_count: 0,
        };

        app.load_todos();

        app
    }

    fn load_todos(&mut self) {
        if let Ok(todos) = self.db.get_todos() {
            self.todos = todos;
            self.total_count = self.todos.len();
            self.completed_count = self.todos.iter().filter(|todo| todo.completed).count();
            self.active_count = self.total_count - self.completed_count;
        }
    }

    fn add_todo(&mut self) {
        let title = self.new_todo.trim();

        if title.is_empty() {
            return;
        }

        if self.db.add_todos(title).is_ok() {
            self.new_todo.clear();
            self.load_todos();
        }
    }

    fn filtered_todos(&self) -> Vec<Todo> {
        self.todos
            .iter()
            .filter(|todo| match self.filter {
                Filter::All => true,
                Filter::Active => !todo.completed,
                Filter::Completed => todo.completed,
            })
            .cloned()
            .collect()
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Left Sidebar
        egui::Panel::left("left_sidebar")
            .exact_size(220.0)
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(20, 20, 27))
                    .inner_margin(20.0),
            )
            .show(ui, |ui| {
                ui.add_space(10.0);

                ui.heading(egui::RichText::new("Todo").size(30.0).strong());

                ui.label(
                    egui::RichText::new("Stay Organized")
                        .color(egui::Color32::from_rgb(150, 150, 165)),
                );

                ui.add_space(35.0);

                ui.label(
                    egui::RichText::new("TASKS")
                        .strong()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(130, 130, 150)),
                );

                ui.add_space(10.0);

                if sidebar_button(ui, "All Tasks", self.filter == Filter::All) {
                    self.filter = Filter::All;
                }

                if sidebar_button(ui, "Active", self.filter == Filter::Active) {
                    self.filter = Filter::Active;
                }

                if sidebar_button(ui, "Completed", self.filter == Filter::Completed) {
                    self.filter = Filter::Completed;
                }

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(20.0);

                ui.label(
                    egui::RichText::new("STATISTICS")
                        .strong()
                        .size(12.0)
                        .color(egui::Color32::from_rgb(130, 130, 150)),
                );

                ui.add_space(15.0);

                ui.label(format!("Total:       {}", self.total_count));
                ui.add_space(8.0);
                ui.label(format!("Active:      {}", self.active_count));
                ui.add_space(8.0);
                ui.label(format!("Completed:   {}", self.completed_count));
            });

        // Main Content
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(14, 14, 19))
                    .inner_margin(35.0),
            )
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(
                        egui::RichText::new(match self.filter {
                            Filter::All => "All Tasks",
                            Filter::Active => "Active",
                            Filter::Completed => "Completed",
                        })
                        .strong()
                        .size(30.0),
                    );
                });

                ui.add_space(5.0);

                ui.label(
                    egui::RichText::new("Manage your tasks and stay productive.")
                        .color(egui::Color32::from_rgb(145, 145, 160)),
                );

                ui.add_space(25.0);

                // Add todo
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(25, 25, 33))
                    .corner_radius(egui::CornerRadius::same(12))
                    .inner_margin(15.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let response = ui.add_sized(
                                [ui.available_width() - 115.0, 40.0],
                                egui::TextEdit::singleline(&mut self.new_todo)
                                    .hint_text("What's need to be done?"),
                            );

                            if response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            {
                                self.add_todo();
                            }

                            if ui
                                .add_sized(
                                    [100.0, 40.0],
                                    egui::Button::new(egui::RichText::new("+ Add Task").strong()),
                                )
                                .clicked()
                            {
                                self.add_todo();
                            }
                        });
                    });

                ui.add_space(25.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let todos = self.filtered_todos();

                        if todos.is_empty() {
                            empty_state(ui);
                        } else {
                            for todo in todos {
                                let id = todo.id;
                                let completed = todo.completed;
                                let mut toggle = false;
                                let mut delete = false;

                                egui::Frame::new()
                                    .fill(egui::Color32::from_rgb(25, 25, 33))
                                    .corner_radius(egui::CornerRadius::same(10))
                                    .inner_margin(10.0)
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            let mut checked = completed;

                                            if ui.checkbox(&mut checked, "").changed() {
                                                toggle = true;
                                            }

                                            let text = if completed {
                                                egui::RichText::new(&todo.title)
                                                    .strikethrough()
                                                    .color(egui::Color32::from_rgb(120, 120, 130))
                                            } else {
                                                egui::RichText::new(&todo.title).size(16.0)
                                            };

                                            ui.label(text);

                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    if ui.button("Delete").clicked() {
                                                        delete = true;
                                                    }
                                                },
                                            );
                                        });
                                    });

                                if toggle {
                                    let _ = self.db.toggle_todo(id, !completed);

                                    self.load_todos();
                                }

                                if delete {
                                    let _ = self.db.delete_todo(id);

                                    self.load_todos();
                                }

                                ui.add_space(8.0);
                            }
                        }
                    });
            });
    }
}

fn empty_state(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(80.0);
        ui.heading("No tasks here");
        ui.label(
            egui::RichText::new("Add a task above to get started.")
                .color(egui::Color32::from_rgb(140, 140, 150)),
        );
    });
}

fn sidebar_button(ui: &mut egui::Ui, text: &str, selected: bool) -> bool {
    let fill = if selected {
        egui::Color32::from_rgb(91, 55, 170)
    } else {
        egui::Color32::TRANSPARENT
    };

    ui.add_sized(
        [180.0, 42.0],
        egui::Button::new(egui::RichText::new(text).size(15.0).strong()).fill(fill),
    )
    .clicked()
}

fn setup_theme(ctx: &egui::Context) {
    ctx.set_visuals(egui::Visuals::dark());

    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.spacing.button_padding = egui::vec2(14.0, 8.0);

        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(35, 35, 45);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(65, 45, 115);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(85, 55, 150);
    });
}
