use crate::input_reader;

fn part1(input: String) {
    // Parse the depths.
    let lines: Vec<&str> = input.split('\n').collect();
    let depths: Vec<i32> = lines.iter().map(|&val| {
        return val.parse::<i32>().unwrap();
    }).collect();

    // Count depths that increase.
    let mut count = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            count += 1;
        }
    }

    println!("Part 1: {}", count);
}

fn part2(input: String) {
    // Parse the depths.
    let lines: Vec<&str> = input.split('\n').collect();
    let depths: Vec<i32> = lines.iter().map(|&val| {
        return val.parse::<i32>().unwrap();
    }).collect();

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
    part1(input.clone());
    part2(input.clone());
}
