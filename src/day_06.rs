use crate::input_reader;

fn part1(bytes: &[u8]) {
    let mut i = 0;
    let bytes_count = bytes.len();
    while i < bytes_count - 4 {
        if bytes[i] == bytes[i + 1] || 
            bytes[i] == bytes[i + 2] || 
            bytes[i] == bytes[i + 3] || 
            bytes[i + 1] == bytes[i + 2] || 
            bytes[i + 1] == bytes[i + 3] || 
            bytes[i + 2] == bytes[i + 3]
        {
            i += 1;
            continue
        }
        break
    }
    println!("Part 1: {}", i + 4);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_06.data");

    part1(input.as_bytes());
}
