use std::env;
use std::fs;

pub fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("error");
    println!("[READING_FILE]");
    return content;
}
