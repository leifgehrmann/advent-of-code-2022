use crate::input_reader;

fn part1(elfs_calories: &Vec<Vec<i32>>) {
    // Elf with the most calories.
    let mut max_total_calories = 0;
    for e in 0..elfs_calories.len() {
        let mut total_calories_for_elf = 0;
        for c in 0..elfs_calories[e].len() {
            total_calories_for_elf += elfs_calories[e][c];
        }
        if max_total_calories < total_calories_for_elf {
            max_total_calories = total_calories_for_elf;
        }
    }
    println!("Part 1: {}", max_total_calories);
}

fn part2(elfs_calories: &Vec<Vec<i32>>) {
    // Calculate totals for all elves first.
    let mut elfs_total: Vec<i32> = elfs_calories.iter().map(|elf_calories| {
        let mut total_calories_for_elf = 0;
        for c in 0..elf_calories.len() {
            total_calories_for_elf += elf_calories[c];
        }
        return total_calories_for_elf;
    }).collect();
    // Sort elf from highest to lowest total calorie count. (Not efficient if 
    // input is billions of entries, but who's trying to be optimal here?)
    elfs_total.sort();
    elfs_total.reverse();
    println!("Part 2: {}", (elfs_total[0] + elfs_total[1] + elfs_total[2]));
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_01_input.txt");

    // Parse the depths.
    let elfs_calories_str: Vec<&str> = input.split("\n\n").collect();
    let elfs_calories: Vec<Vec<i32>> = elfs_calories_str.iter().map(|&val| {
        let elf_calories_str: Vec<&str> = val.split("\n").collect();
        let elf_calories: Vec<i32> = elf_calories_str.iter().map(|&val| {
            return val.parse::<i32>().unwrap();
        }).collect();
        return elf_calories;
    }).collect();

    part1(&elfs_calories);
    part2(&elfs_calories);
}
