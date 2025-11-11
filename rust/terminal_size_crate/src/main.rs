use terminal_size::{terminal_size, Height, Width};

fn main() {
    if let Some((Width(w), Height(_h))) = terminal_size() {
        let message = "Hello Rust";
        let left_padding = (w as usize / 2).saturating_sub(message.len() / 2);
        println!("\n");
        println!("{:width$}{}", "", message, width = left_padding);
        println!("\n");
    } else {
       println!("Unable to get the size");
    }
}
