use std::{env, path::PathBuf};
use std::fs;

fn get_current_working_dir() -> PathBuf {
    return env::current_dir().unwrap();
}

pub fn read_file_in_cwd(file: &str) -> String {
    let file_path = get_current_working_dir().join(file);
    return fs::read_to_string(file_path).unwrap();
}
