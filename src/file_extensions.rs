use std::{fs::{self, File}, io::{self, Read}};

use serde::de::DeserializeOwned;

pub fn check_file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

pub fn load_file<T: DeserializeOwned>(file_path: &str) -> Vec<T> {
    if check_file_exists(file_path) {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        serde_json::from_str(&contents).expect("Failed to deserialize JSON")
    } else {
        Vec::new()
    }
}

pub fn save_file(file_path: &str, data: String) -> Result<(), io::Error> {
    fs::write(file_path, data)?;
    println!("File saved successfully!");
    Ok(())
}
