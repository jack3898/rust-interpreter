use std::{fs, process::exit};

pub fn run_file(path: &String) -> String {
    fs::read_to_string(&path).unwrap_or_else(|_| {
        println!("Cannot read path: {}", path);

        exit(2);
    })
}
