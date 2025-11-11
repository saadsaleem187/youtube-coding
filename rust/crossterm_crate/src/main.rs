use std::{io::{self, stdout}, time::Duration};

use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode}};

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    enable_raw_mode()?;
    execute!(stdout, EnableMouseCapture)?;
    
    println!("Press any key or q to quit. Resize Termainal and Mouse Clicks");

    loop {
        if event::poll(Duration::from_millis(500))? {
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Char('q') {
                        println!("Goodbye");
                        break;
                    } else {
                        println!("Key Pressed: {:?}", key_event.code);
                    }
                },
                Event::Resize(cols, rows) => {
                    println!("The terminal resize: {}x{}", cols, rows);
                },
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::Down(btn) => match btn {
                        MouseButton::Left => println!("Left button clicked"),
                        MouseButton::Right => println!("Right button clicked"),
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }    
    }

    disable_raw_mode()?;
    execute!(stdout, DisableMouseCapture)?;

    Ok(())
}
