use crate::input_reader;

fn part1(instructions: Vec<&str>) -> Vec<i32> {
    let mut x = 1;
    let mut x_states: Vec<i32> = vec![];

    x_states.push(x);

    for instruction in instructions {
        let split: Vec<&str> = instruction.split(" ").collect();
        let op = split[0];
        if op == "noop" {
            x_states.push(x);
        } else if op == "addx" {
            x_states.push(x);
            x_states.push(x);
            x += split[1].parse::<i32>().unwrap();
        }
    }

    let mut interesting_signal_strength = 0;
    for c in (20..240).step_by(40) {
        interesting_signal_strength += c as i32 * x_states[c];
    }
    println!("Part 1: {}", interesting_signal_strength);

    return x_states
}

fn part2(states: Vec<i32>) {
    println!("Part 2:");
    for c in 1..states.len() {
        let x_pos = ((c - 1) % 40) as i32;
        if x_pos >= states[c] - 1 && x_pos <= states[c] + 1 {
            print!("#")
        } else {
            print!(".")
        }

        if (c - 1) % 40 == 39 {
            print!("\n")
        }
    }
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_10.data");

    let lines: Vec<&str> = input.split("\n").collect();

    let states = part1(lines);
    part2(states);
}
