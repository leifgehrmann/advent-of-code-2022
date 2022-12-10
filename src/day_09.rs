use crate::input_reader;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

struct Movement {
    direction: Direction,
    distance: i32
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn to_hashable(&self) -> String {
        return format!("{},{}", self.x, self.y);
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Position {}

fn move_head(head: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::U => return Position { x: head.x + 1, y: head.y },
        Direction::D => return Position { x: head.x - 1, y: head.y },
        Direction::L => return Position { x: head.x, y: head.y - 1 },
        Direction::R => return Position { x: head.x, y: head.y + 1 }
    }
}

fn follow_head(head: &Position, tail: &Position) -> Position {
    if (head.x - tail.x).abs() >= 2 {
        // Two steps left or right...
        if head.x > tail.x {
            return Position { x: head.x - 1, y: head.y }
        } else {
            return Position { x: head.x + 1, y: head.y }
        }
    } else if (head.y - tail.y).abs() >= 2 {
        // Two steps up or down...
        if head.y > tail.y {
            return Position { x: head.x, y: head.y - 1 }
        } else {
            return Position { x: head.x, y: head.y + 1 }
        }
    }
    return tail.clone()
}

fn part1(movements: &Vec<Movement>) {
    let mut tail_visits = HashMap::new();

    let mut head_position = Position { x: 0, y:0 };
    let mut tail_position = Position { x: 0, y:0 };

    tail_visits.insert(tail_position.to_hashable(), 0);

    for movement in movements {
        // Simulate each step of each movement instruction.
        for _ in 0..movement.distance {
            head_position = move_head(&head_position, &movement.direction);
            tail_position = follow_head(&head_position, &tail_position);

            tail_visits.insert(tail_position.to_hashable(), 0);
        }
    }

    println!("Part 1: {}", tail_visits.len());
}

fn part2(movements: &Vec<Movement>) {
    let mut tail_visits = HashMap::new();

    let mut knots: Vec<Position> = vec![Position { x: 0, y: 0 }; 10];

    for movement in movements {
        // Simulate each step of each movement instruction.
        for _ in 0..movement.distance {
            knots[0] = move_head(&knots[0], &movement.direction);

            for k in 1..knots.len() {
                knots[k] = follow_head(&knots[k-1], &knots[k]);
            }

            println!("Part 2: {}", knots.last().unwrap().to_hashable());
            tail_visits.insert(knots.last().unwrap().to_hashable(), 0);
        }
    }

    println!("Part 2: {}", tail_visits.len());
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_09.data");

    let lines: Vec<&str> = input.split('\n').collect();
    let movements: Vec<Movement> = lines.iter().map(|&val| {
        let line_split: Vec<&str> = val.split(' ').collect();
        let direction: Direction = match line_split[0] {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => Direction::R,
        };
        let distance = line_split[1].parse::<i32>().unwrap();
        return Movement {
            direction,
            distance
        }
    }).collect();

    part1(&movements);
    part2(&movements);
}
