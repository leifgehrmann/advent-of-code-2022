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

impl Movement {
    pub fn dx(&self) -> i32 {
        if self.direction == Direction::R {
            return self.distance
        } else if self.direction == Direction::L {
            return -self.distance
        }
        return 0
    }

    pub fn dy(&self) -> i32 {
        if self.direction == Direction::U {
            return self.distance
        } else if self.direction == Direction::D {
            return -self.distance
        }
        return 0
    }
}


struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn distance_sq(&self, position: &Position) -> i32 {
        // Returns the distance squared.
        let dx = (self.x - position.x).abs();
        let dy = (self.y - position.y).abs();
        return dx * dx + dy * dy
    }

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

fn move_positions(head: &Position, tail: &Position, movement: &Movement) -> (Position, Position) {
    let new_head = Position { x: head.x + movement.dx(), y: head.y + movement.dy() };
    let mut new_tail = Position { x: tail.x, y: tail.y };
    if new_head.distance_sq(tail) > 2 {
        new_tail = match movement.direction {
            Direction::U => Position { x: new_head.x, y: new_head.y - 1 },
            Direction::D => Position { x: new_head.x, y: new_head.y + 1 },
            Direction::L => Position { x: new_head.x + 1, y: new_head.y },
            Direction::R => Position { x: new_head.x - 1, y: new_head.y }
        };
    }
    return (new_head, new_tail)
}

fn part1(movements: Vec<Movement>) {
    let mut tail_visits = HashMap::new();

    let mut head_position = Position { x: 0, y:0 };
    let mut tail_position = Position { x: 0, y:0 };

    tail_visits.insert(tail_position.to_hashable(), 0);

    for movement in movements {
        // Simulate each step of each movement instruction.
        for _ in 0..movement.distance {
            (head_position, tail_position) = move_positions(
                &head_position,
                &tail_position,
                &Movement { direction: movement.direction, distance: 1 }
            );

            println!("{}", tail_position.to_hashable());

            tail_visits.insert(tail_position.to_hashable(), 0);
        }
    }

    println!("Part 1: {}", tail_visits.len());
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

    part1(movements)
}
