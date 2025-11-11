use figlet_rs::FIGfont;

fn main() {
    let standard = FIGfont::standard().unwrap();
    let slant = FIGfont::from_file("Slant.flf").unwrap();

    load_font(standard);
    load_font(slant);
}

fn load_font(font: FIGfont) {
    let figure = font.convert("Hello Rust");
    print!("{}", figure.unwrap());
}
