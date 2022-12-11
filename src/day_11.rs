use crate::input_reader;

#[derive(PartialEq, Eq, Debug)]
enum Operator {
    Add,
    Mul
}

struct Monkey {
    starting_items: Vec<i32>,
    operator: Operator,
    operand: Option<i32>, // If null, the operand is 'old'.
    test_divisible: i32,
    throw_to_monkey_if_true: usize,
    throw_to_monkey_if_false: usize
}

fn part1(monkeys: &Vec<Monkey>) {
    let mut monkey_states: Vec<Vec<i32>> = vec![vec![]; monkeys.len()];
    let mut monkey_inspections: Vec<i32>  = vec![0; monkeys.len()];
    // copy starting items into state
    for m in 0..monkeys.len() {
        let cloned_state = &mut monkeys[m].starting_items.clone();
        monkey_states[m].append(cloned_state)
    }
    
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let monkey = &monkeys[m];
            let monkey_items = monkey_states[m].clone();
            for item in monkey_items {
                // Increase monkey inspection.
                monkey_inspections[m] += 1;

                // Calculate new worry.
                let mut worry = item.clone();
                let operand = monkey.operand.unwrap_or(worry);
                match monkey.operator {
                    Operator::Add => worry = worry + operand,
                    Operator::Mul => worry = worry * operand
                }
                worry = worry / 3;

                // Throw to monkey.
                if worry % monkey.test_divisible == 0 {
                    monkey_states[monkey.throw_to_monkey_if_true].push(worry);
                } else {
                    monkey_states[monkey.throw_to_monkey_if_false].push(worry);
                }
            }

            monkey_states[m] = vec![]
        }
    }

    monkey_inspections.sort();
    monkey_inspections.reverse();
    println!("Part 1: {}", monkey_inspections[0] * monkey_inspections[1]);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_11.data");

    let lines: Vec<&str> = input.split("\n\n").collect();
    let monkeys: Vec<Monkey> = lines.iter().map(|&val| {
        let lines: Vec<&str> = val.split("\n").collect();
        // Parse items
        let starting_items: Vec<i32> = lines[1]
            .replace(",", "")
            .split(" ").skip(4)
            .map(|i| { i.parse::<i32>().unwrap() }).collect();
        // Parse operation
        let operation_str: Vec<&str> = lines[2].split(" ").collect();
        let operator = match operation_str[6] {
            "+" => Operator::Add,
            _ => Operator::Mul
        };
        let operand = operation_str[7].parse::<i32>().ok();
        // Parse test
        let test_str: Vec<&str> = lines[3].split(" ").collect();
        let test_divisible = test_str[5].parse::<i32>().unwrap();
        // Parse throws
        let throw_true_str: Vec<&str> = lines[4].split(" ").collect();
        let throw_false_str: Vec<&str> = lines[5].split(" ").collect();
        let throw_true = throw_true_str[9].parse::<usize>().unwrap();
        let throw_false = throw_false_str[9].parse::<usize>().unwrap();
        
        return Monkey {
            starting_items,
            operator,
            operand, 
            test_divisible,
            throw_to_monkey_if_true: throw_true,
            throw_to_monkey_if_false: throw_false
        }
    }).collect();

    part1(&monkeys);
}
