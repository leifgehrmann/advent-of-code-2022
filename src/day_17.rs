use crate::input_reader;

enum Push {
    Left,
    Right,
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_17.data");

    let pushes_str: Vec<&str> = input.split("").collect();
    let pushes: Vec<Push> = pushes_str.iter().map(|&val| {
        return match val {
            "<" => Push::Left,
            ">" => Push::Right,
            _ => Push::Left
        }
    }).collect();
}
