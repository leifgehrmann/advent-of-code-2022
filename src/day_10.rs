use crate::input_reader;

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_10.data");

    let lines: Vec<&str> = input.split("\n").collect();
}
