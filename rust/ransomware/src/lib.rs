use std::fs;
use std::io;

pub fn get_filenames() -> io::Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();

    for entry in fs::read_dir(".")? {
        let dir = entry?;
        let filename = dir.file_name().to_string_lossy().into_owned();

        if filename == "Cargo.lock" || filename == "Cargo.toml" || filename == "my_key.key" {
            continue;
        }

        if dir.path().is_file() {
            files.push(filename);
        }
    }

    Ok(files)
}
