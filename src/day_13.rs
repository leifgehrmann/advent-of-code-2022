use crate::input_reader;

fn char_at(str: &str, n: usize) -> char {
    return str.chars().nth(n).unwrap()
}

fn split(str: &str) -> Vec<&str> {
    let mut depth = 0;
    let mut split_index_start: Vec<usize> = vec![0];
    let mut split_index_end: Vec<usize> = vec![];
    for i in 0..str.len() {
        if char_at(str, i) == '[' {
            depth += 1
        } else if char_at(str, i) == ']' {
            depth -= 1
        }
        if char_at(str, i) == ',' && depth == 0 {
            split_index_end.push(i);
            split_index_start.push(i+1);
        }
    }
    split_index_end.push(str.len());
    println!("str {}", str);
    println!("slice start {:?}", split_index_start);
    println!("slice start {:?}", split_index_end);
    let mut result: Vec<&str> = vec![];
    for s_i in 0..split_index_start.len() {
        if split_index_start[s_i] == split_index_end[s_i] {
            continue;
        }
        result.push(&str[split_index_start[s_i]..split_index_end[s_i]]);
    }
    return result;
}

fn is_right_order(a_str: &str, b_str: &str) -> Option<bool> {
    println!("{} vs {}", a_str, b_str);
    // If both values are integers, the lower integer should come first.
    // If the left integer is lower than the right integer, the inputs are in the right order.
    // If the left integer is higher than the right integer, the inputs are not in the right order.
    // Otherwise, the inputs are the same integer; continue checking the next part of the input.
    if a_str.len() == 1 && b_str.len() == 1 {
        if a_str.as_bytes()[0] > b_str.as_bytes()[0] {
            return Some(false);
        } else if a_str.as_bytes()[0] < b_str.as_bytes()[0] {
            return Some(true);
        }
        return None;
    }

    // If exactly one value is an integer, convert the integer to a list which contains that
    // integer as its only value, then retry the comparison.
    // For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list 
    // containing 2); the result is then found by instead comparing [0,0,0] and [2].
    if a_str.len() > 1 && b_str.len() == 1 {
        return is_right_order(a_str, format!("[{}]", b_str).as_str())
    }
    if a_str.len() == 1 && b_str.len() > 1 {
        return is_right_order(format!("[{}]", a_str).as_str(), b_str)
    }

    let a_split = split(&a_str[1..a_str.len()-1]);
    let b_split = split(&b_str[1..b_str.len()-1]);

    println!("a split = {:?}", a_split);
    println!("b split = {:?}", b_split);

    for a_split_i in 0..a_split.len() {
        // If the right list runs out of items first, the inputs are not in the right order.
        if b_split.len() == 0 || a_split_i > b_split.len() - 1 {
            return Some(false);
        }
        let result = is_right_order(a_split[a_split_i], b_split[a_split_i]);
        if result.is_some() {
            return result;
        }
    }

    if a_split.len() < b_split.len() {
        // If the left list runs out of items first, the inputs are in the right order.
        return Some(true);
    }

    // If the lists are the same length and no comparison makes a decision about the order, 
    // continue checking the next part of the input.
    return None;
}

fn part1(packet_pairs: &Vec<(&str, &str)>) {
    let mut index_sum: usize = 0;
    for i in 0..packet_pairs.len() {
        // It'll be much easier to parse a sequence if all the control
        // characters and values are 1 char in width.
        let a_string = packet_pairs[i].0.replace("10", "A");
        let b_string = packet_pairs[i].1.replace("10", "A");
        let is_right_order = is_right_order(a_string.as_str(), b_string.as_str());
        if is_right_order.is_none() || is_right_order.unwrap() {
            println!("Result is in right order");
            index_sum += i + 1;
        } else {
            println!("Result is NOT in right order");
        }
        println!("\n\n");
    }
    println!("Part 1: {}", index_sum);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_13.data");

    let packet_pairs_str: Vec<&str> = input.split("\n\n").collect();
    let packet_pairs: Vec<(&str, &str)> = packet_pairs_str.iter().map(|&val| {
        return val.split_once("\n").unwrap()
    }).collect();
    
    part1(&packet_pairs);
}
