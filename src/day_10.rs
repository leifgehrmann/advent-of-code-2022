use crate::input_reader;

fn part1(instructions: Vec<&str>) {
    let mut x = 1;
    let mut x_state: Vec<i32> = vec![];

    x_state.push(x);

    for instruction in instructions {
        let split: Vec<&str> = instruction.split(" ").collect();
        let op = split[0];
        if op == "noop" {
            x_state.push(x);
        } else if op == "addx" {
            x_state.push(x);
            x_state.push(x);
            x += split[1].parse::<i32>().unwrap();
        }
    }

    let mut interesting_signal_strength = 0;
    for c in (20..240).step_by(40) {
        interesting_signal_strength += c as i32 * x_state[c];
    }
    println!("Part 1: {}", interesting_signal_strength);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_10.data");

    let lines: Vec<&str> = input.split("\n").collect();

    part1(lines)
}
