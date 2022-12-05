use crate::input_reader;

fn char_to_priority(c: char) -> i32 {
    let ascii_code = c as i32;
    // 65-90 = A-Z
    if ascii_code >= 65 && ascii_code <= 90 {
        return ascii_code - 65 + 27;
    }
    // 97-122 = a-z
    return ascii_code - 97 + 1;
}

fn part1(rucksacks: &Vec<&str>) {
    let mut priority_sum = 0;

    for rucksack in rucksacks {
        let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);
        for c in compartment_2.chars() {
            if compartment_1.contains(c) {
                priority_sum += char_to_priority(c);
                break
            }
        }
    }

    println!("Part 1: {}", priority_sum);
}

fn part2(rucksacks: &Vec<&str>) {
    let mut priority_sum = 0;
    
    for group_index in (0..rucksacks.len()).step_by(3) {
        let rucksack_1 = rucksacks[group_index];
        let rucksack_2 = rucksacks[group_index + 1];
        let rucksack_3 = rucksacks[group_index + 2];
        for c in rucksack_1.chars() {
            if rucksack_2.contains(c) && rucksack_3.contains(c) {
                priority_sum += char_to_priority(c);
                break
            }
        }
    }

    println!("Part 2: {}", priority_sum);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_03_input.txt");

    let rucksacks: Vec<&str> = input.split('\n').collect();

    part1(&rucksacks);
    part2(&rucksacks);
}
