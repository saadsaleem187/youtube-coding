use eframe::egui;
use std::time::{Duration, Instant};

const CELL_SIZE: f32 = 25.0;
const MOVE_INTERVAL: Duration = Duration::from_millis(150);

#[derive(Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeGame {
    snake: Vec<Point>,
    direction: Direction,
    food: Point,
    last_move: Instant,
    game_over: bool,
    score: usize,
}

impl Default for SnakeGame {
    fn default() -> Self {
        Self {
            snake: vec![
                Point { x: 10, y: 10 },
                Point { x: 9, y: 10 },
                Point { x: 8, y: 10 },
            ],
            direction: Direction::Right,
            food: Point { x: 5, y: 5 },
            last_move: Instant::now(),
            game_over: false,
            score: 0,
        }
    }
}

impl SnakeGame {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn update_game(&mut self, board_width: i32, board_height: i32) {
        if self.game_over {
            return;
        }

        if self.last_move.elapsed() < MOVE_INTERVAL {
            return;
        }

        self.last_move = Instant::now();

        let mut head = self.snake[0];

        match self.direction {
            Direction::Up => head.y -= 1,
            Direction::Down => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Left => head.x -= 1,
        }

        if head.x < 0 || head.x > board_width || head.y < 0 || head.y > board_height {
            self.game_over = true;

            return;
        }

        if self.snake.contains(&head) {
            self.game_over = true;

            return;
        }

        self.snake.insert(0, head);

        if head == self.food {
            self.score += 1;

            self.food = Point {
                x: (head.x * 7 + 3) % board_width,
                y: (head.y * 11 + 5) % board_height,
            };
        } else {
            self.snake.pop();
        }
    }

    fn handle_input(&mut self, ctx: &egui::Context) {
        ctx.input(|input| {
            if input.key_pressed(egui::Key::ArrowUp) && self.direction != Direction::Down {
                self.direction = Direction::Up;
            }
            if input.key_pressed(egui::Key::ArrowDown) && self.direction != Direction::Up {
                self.direction = Direction::Down;
            }
            if input.key_pressed(egui::Key::ArrowLeft) && self.direction != Direction::Right {
                self.direction = Direction::Left;
            }
            if input.key_pressed(egui::Key::ArrowRight) && self.direction != Direction::Left {
                self.direction = Direction::Right;
            }
        });
    }
}

impl eframe::App for SnakeGame {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.handle_input(ui.ctx());

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Score: {}", self.score));

                if self.game_over {
                    ui.colored_label(egui::Color32::RED, "GAME OVER");

                    if ui.button("Reset").clicked() {
                        self.reset();
                    }
                }
            });

            ui.add_space(5.0);

            // Everything below is the board area
            let board_rect = ui.available_rect_before_wrap();

            // Calculate how many cells fit in the available area
            let board_width = (board_rect.width() / CELL_SIZE).floor().max(1.0) as i32;
            let board_height = (board_rect.height() / CELL_SIZE).floor().max(1.0) as i32;

            // Actual visible board
            let board_size = egui::vec2(
                board_width as f32 * CELL_SIZE,
                board_height as f32 * CELL_SIZE,
            );

            let board_rect = egui::Rect::from_min_size(board_rect.min, board_size);

            self.update_game(board_width, board_height);

            // Border of the playing area
            ui.painter().rect_stroke(
                board_rect,
                0.0,
                egui::Stroke::new(2.0, egui::Color32::WHITE),
                egui::StrokeKind::Outside,
            );

            // Draw Snake
            for segment in &self.snake {
                let x = board_rect.left() + segment.x as f32 * CELL_SIZE;
                let y = board_rect.top() + segment.y as f32 * CELL_SIZE;

                ui.painter().rect_filled(
                    egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(CELL_SIZE, CELL_SIZE)),
                    2.0,
                    egui::Color32::GREEN,
                );
            }

            // Draw food
            let food_x = board_rect.left() + self.food.x as f32 * CELL_SIZE;
            let food_y = board_rect.top() + self.food.y as f32 * CELL_SIZE;

            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(food_x, food_y),
                    egui::vec2(CELL_SIZE, CELL_SIZE),
                ),
                2.0,
                egui::Color32::RED,
            );
        });

        ui.ctx().request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Snake Game",
        native_options,
        Box::new(|_cc| Ok(Box::new(SnakeGame::default()))),
    )
}
