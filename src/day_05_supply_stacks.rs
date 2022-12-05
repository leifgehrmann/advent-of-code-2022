use crate::input_reader;
use regex::Regex;

struct Procedure {
    crates: i32,
    from: i32,
    to: i32
}

fn part1(procedures: Vec<Procedure>) {
    
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_05.data");

    // Parse input into arrangement and procedures.
    let input_str_pieces: Vec<&str> = input.split("\n\n").collect();

    // Parse arrangement data.
    let arrangement_str: Vec<&str> = input_str_pieces[0].split("\n").collect();
    let stacks_height = arrangement_str.len() - 1;
    // Use the last line to count how many stacks there are, and create a list.
    let stacks_count = (arrangement_str.last().unwrap().len() + 1) / 4;
    println!("Part 1: {}", stacks_count);
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stacks_count];
    let stack_regex = Regex::new(r"\[[A-Z]\]|    ").unwrap();
    let crate_regex = Regex::new(r"[A-Z]").unwrap();
    for stack_height_index in (0..stacks_height).rev() {
        let stack_height_str = arrangement_str[stack_height_index];
        let mut stack_index = 0;
        for capture in stack_regex.captures_iter(stack_height_str) {
            if capture[0].starts_with(" ") {
                stack_index += 1;
                continue;
            }
            let container = &crate_regex.captures(&capture[0]).unwrap()[0];
            stacks[stack_index].push(container.chars().next().unwrap());
            println!("Part 1: {:?}", stacks[stack_index]);
            stack_index += 1;
        }
    }

    // Parse procedures.
    let procedures_str: Vec<&str> = input_str_pieces[0].split("\n").collect();
    let procedures: Vec<Procedure> = procedures_str.iter().map(|&val| {
        let split_procedure_str: Vec<&str> = val.split(" ").collect();
        let crates = split_procedure_str[1].parse::<i32>().unwrap();
        let from = split_procedure_str[3].parse::<i32>().unwrap();
        let to = split_procedure_str[5].parse::<i32>().unwrap();
        return Procedure { crates, from, to }
    }).collect();

    part1(procedures);
}
