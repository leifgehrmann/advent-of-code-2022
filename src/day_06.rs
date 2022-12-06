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

fn part2(bytes: &[u8]) {
    let mut i = 0;
    let bytes_count = bytes.len();
    let window = 14;
    'outer: while i < bytes_count - window {
        for c1 in 0..window - 1 {
            for c2 in c1 + 1..window {
                if bytes[i + c1] == bytes[i + c2] {
                    i += 1;
                    continue 'outer
                }
            }
        }
        break
    }
    println!("Part 2: {}", i + window);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_06.data");

    part1(input.as_bytes());
    part2(input.as_bytes());
}
