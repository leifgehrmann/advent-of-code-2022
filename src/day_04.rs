use crate::input_reader;

struct SectionAssignment {
    pair_1_start: i32,
    pair_1_end: i32,
    pair_2_start: i32,
    pair_2_end: i32,
}

fn part1(section_assignments: &Vec<SectionAssignment>) {
    let mut overlapping_count = 0;
    for s in section_assignments {
        // Is Pair2 inside Pair 1 OR Pair1 inside Pair2?
        if (s.pair_1_start <= s.pair_2_start && s.pair_1_end >= s.pair_2_end) ||
           (s.pair_2_start <= s.pair_1_start && s.pair_2_end >= s.pair_1_end) {
            overlapping_count += 1
        }
    }
    println!("Part 1: {}", overlapping_count);
}

fn part2(section_assignments: &Vec<SectionAssignment>) {
    let mut overlapping_count = 0;
    for s in section_assignments {
        // Is Pair2 in front of Pair1 OR Pair1 in front of Pair2
        if (s.pair_1_end >= s.pair_2_start && s.pair_1_start <= s.pair_2_start) ||
           (s.pair_2_end >= s.pair_1_start && s.pair_2_start <= s.pair_1_start) {
            overlapping_count += 1
        }
    }
    println!("Part 2: {}", overlapping_count);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_04.data");

    // Parse the input into pairs of ranges.
    let lines: Vec<&str> = input.split('\n').collect();
    let section_assignments: Vec<SectionAssignment> = lines.iter().map(|&val| {
        let split_pairs: Vec<&str> = val.split(',').collect();
        let pair_1: Vec<&str> = split_pairs[0].split('-').collect();
        let pair_2: Vec<&str> = split_pairs[1].split('-').collect();
        let pair_1_start = pair_1[0].parse::<i32>().unwrap();
        let pair_1_end = pair_1[1].parse::<i32>().unwrap();
        let pair_2_start = pair_2[0].parse::<i32>().unwrap();
        let pair_2_end = pair_2[1].parse::<i32>().unwrap();
        return SectionAssignment {
            pair_1_start,
            pair_1_end,
            pair_2_start,
            pair_2_end,
        }
    }).collect();

    part1(&section_assignments);
    part2(&section_assignments);
}
