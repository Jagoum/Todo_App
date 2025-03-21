use std::{
    fs::{self, OpenOptions},
    io::Read,
};

use serde_json::{json, Map, Value};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
    {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not Open or even create file");
            return Err("()").unwrap();
        }
    };

    let mut file_data = String::new();
    file.read_to_string(&mut file_data).unwrap();

    let json: Value = match serde_json::from_str(&file_data) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Failed To Parse Data {}", e);
            return Map::default();
        }
    };
    let state = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(), new_data.to_string()).expect("Failed to write to file");
}
