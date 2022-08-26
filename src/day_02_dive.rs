use crate::input_reader;

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Clone)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

fn part1(instructions: Vec<Instruction>) {
    let mut horizontal = 0;
    let mut vertical = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => {
                vertical -= instruction.distance;
            },
            Direction::Down => {
                vertical += instruction.distance;
            },
            Direction::Forward => {
                horizontal += instruction.distance;
            },
        }
    }
    println!("Part 1: {}", (horizontal * vertical));
}

fn part2(instructions: Vec<Instruction>) {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => {
                aim -= instruction.distance;
            },
            Direction::Down => {
                aim += instruction.distance;
            },
            Direction::Forward => {
                horizontal += instruction.distance;
                vertical += aim * instruction.distance;
            },
        }
    }
    println!("Part 2: {}", (horizontal * vertical));
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_02_input.txt");

    // Parse the instructions.
    let lines: Vec<&str> = input.split('\n').collect();
    let instructions: Vec<Instruction> = lines.iter().map(|&val| {
        let instruction_split: Vec<&str> = val.split(' ').collect();
        let direction: Direction = match instruction_split[0] {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => Direction::Forward,
        };
        let distance = instruction_split[1].parse::<i32>().unwrap();
        return Instruction {
            direction,
            distance
        }
    }).collect();

    part1(instructions.clone());
    part2(instructions.clone());
}
