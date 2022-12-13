use crate::input_reader;

// fn char_at(str: &str, n: usize) -> char {
//     return str.chars().nth(n).unwrap()
// }

// fn split(x: &str) -> Vec<&'static str> {
//     let mut depth
//     let mut commas: Vec<usize> = vec![];
//     return x.split;
// }

// fn is_right_order(a_str: &str, mut b_str: &str) -> bool {
//     println!("Compare {:?} vs {:?}", a_str, b_str);
//     let mut a_depth = 0;
//     let mut b_depth = 0;
//     let mut b_i = 0;
//     for a in a_str.chars() {
//         println!("- {}, {}", a, char_at(b_str, b_i));
//         if a == '[' {
//             a_depth += 1;
//             if char_at(b_str, b_i) == '[' {
//                 b_i += 1;
//             }
//             continue;
//         } else if a == ']' {
//             a_depth -= 1;
//             while char_at(b_str, b_i) != ']' {
//                 b_i += 1;
//             }
//             continue;
//         } else if a == ',' {
//             if char_at(b_str, b_i) == ']' {
//                 println!("Right side ran out of items, so inputs are not in the right order");
//                 return false;
//             } else {
//                 b_i += 1;
//             }
//             continue;
//         }
//         if char_at(b_str, b_i) == '[' {
//             println!("Mixed types: Convert left to [{}] and retry comparison", a);
//             while char_at(b_str, b_i) == '[' {
//                 b_depth += 1;
//                 b_i += 1;
//             }
//         }
//         let b = char_at(b_str, b_i);
//         println!("Compare {} vs {}", a, b);
//         if a < b {
//             println!("Left side is smaller, so inputs are in the right order");
//             return true;
//         } else if a > b {
//             println!("Right side is smaller, so inputs are not in the right order");
//             return true;
//         }
//         b_i += 1;
//     }
//     println!(" ");
//     return false;

// }

// fn part1(packet_pairs: &Vec<(&str, &str)>) {
//     let mut index_sum: usize = 0;
//     for i in 0..packet_pairs.len() {
//         // It'll be much easier to parse a sequence if all the control
//         // characters are 1 char in width.
//         let a_string = packet_pairs[i].0.replace("10", "A");
//         let b_string = packet_pairs[i].1.replace("10", "A");
//         if is_right_order(a_string.as_str(), b_string.as_str()) {
//             index_sum += i + 1;
//         }
//         println!("\n\n");
//     }
//     println!("Part 1: {}", index_sum);
// }

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_13.data");

    let packet_pairs_str: Vec<&str> = input.split("\n\n").collect();
    let packet_pairs: Vec<(&str, &str)> = packet_pairs_str.iter().map(|&val| {
        return val.split_once("\n").unwrap()
    }).collect();
    
    // part1(&packet_pairs);
}
