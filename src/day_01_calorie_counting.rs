use crate::input_reader;

fn part1(elfs_calories: Vec<Vec<i32>>) {
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
}

fn part2(depths: Vec<i32>) {
    // Compute the sum of depths in a sliding window of 3.
    let mut depth_sums: Vec<i32> = Vec::new();
    let window = 3;
    for i in (window - 1)..depths.len() {
        let mut sum = 0;
        for s in 0..window {
            sum += depths[i - s];
        }
        depth_sums.push(sum);
    }

    // Count sum-depths that increase.
    let mut count = 0;
    for i in 1..depth_sums.len() {
        if depth_sums[i] > depth_sums[i - 1] {
            count += 1;
        }
    }

    println!("Part 2: {}", count);
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

    part1(elfs_calories);
    // part2(depths.clone());
}
