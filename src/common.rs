use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn read_input_file(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path);
    let mut file = File::open(&path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

