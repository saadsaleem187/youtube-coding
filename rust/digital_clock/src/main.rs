use std::{
    io::{stdout, Write},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use figlet_rs::FIGfont;
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let mut stdout = stdout();

    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let total_secs = now.as_secs() % 84600;
        let hour = (total_secs / 3600) % 24;
        let minute = (total_secs % 3600) / 60;
        let second = total_secs % 60;

        let time_str = format!("{:02}:{:02}:{:02}", hour, minute, second);
        let figure = standard_font.convert(&time_str).unwrap();

        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        if let Some((Width(w), Height(h))) = terminal_size() {
            let figure_str = figure.to_string();
            let lines: Vec<&str> = figure_str.lines().collect();

            // Center Vertically
            let vertical_padding = h.saturating_sub(lines.len() as u16) / 2;
            for _ in 0..vertical_padding {
                println!();
            }

            // Center Horizontally
            for line in lines {
                let left_padding = (w as usize / 2).saturating_sub(line.len() / 2);
                print!("{:width$}", "", width = left_padding);
                print!("{}{}{}", SetForegroundColor(Color::Blue), line, ResetColor);
                println!();
            }

            stdout.flush().unwrap();
        }

        thread::sleep(Duration::from_secs(1));
    }
}
