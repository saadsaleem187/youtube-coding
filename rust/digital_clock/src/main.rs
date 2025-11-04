use std::{process::Command, thread, time::Duration};

use chrono::{Local, Timelike};

fn main() {
    loop {
        let now = Local::now();

        clear_screen();

        println!("Digital Clock");
        println!("---------");
        println!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        println!("---------");
    
        thread::sleep(Duration::from_secs(1));
    }
} 

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}
