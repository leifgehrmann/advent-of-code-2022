use std::{env, path::PathBuf};
use std::fs;

fn get_current_working_dir() -> PathBuf {
    return env::current_dir().unwrap();
}

pub fn read_file_in_cwd() -> String {
    let file_path = get_current_working_dir().join("src/day_01_input.txt");
    return fs::read_to_string(file_path).unwrap();
}
