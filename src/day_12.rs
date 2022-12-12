use crate::input_reader;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

type Cost = usize;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Pos {}

#[derive(Debug, Eq)]
struct Processable {
    pos: Pos,
    net_cost: Cost
}

impl Processable {
    pub fn new(pos: Pos, net_cost: Cost) -> Self {
        Processable {
            pos,
            net_cost
        }
    }
}

impl PartialOrd for Processable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Processable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.net_cost.cmp(&other.net_cost)
    }
}

impl PartialEq for Processable {
    fn eq(&self, other: &Self) -> bool {
        self.net_cost == other.net_cost
    }
}

fn get_neighbors(map: &Vec<Vec<i32>>, pos: Pos) -> Vec<Pos> {
    let w = map[0].len() as i32;
    let h = map.len() as i32;
    let points: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    return points.iter().map(|&point| {
        let new_pos = Pos { x: pos.x + point.0, y: pos.y + point.1 };
        if new_pos.x < 0 ||
           new_pos.x >= w ||
           new_pos.y < 0 ||
           new_pos.y >= h
        {
            return None;
        }
        let a = map[pos.y as usize][pos.x as usize];
        let b = map[new_pos.y as usize][new_pos.x as usize];
        if a + 1 < b {
            return None;
        }
        return Some(new_pos);
    }).filter_map(|e| e).collect()
}

fn compute_cost(map: &Vec<Vec<i32>>, start: Pos) -> HashMap<Pos, Cost> {
    // A simplified version of Dijkstra.
    let mut processed: HashMap<Pos, Cost> = HashMap::new();
    let mut to_process: VecDeque<Processable> = VecDeque::new();
    to_process.push_back(Processable::new(start, 0));
    processed.insert(start, 0);

    while to_process.len() > 0 {
        // Ensure elements are from least costly to most costly.
        to_process.make_contiguous().sort();
        let processable = to_process.pop_front().unwrap();
        for neighbor in get_neighbors(map, processable.pos) {
            let new_cost = processable.net_cost + 1;
            // Skip points that have already been processed
            if processed.contains_key(&neighbor) && processed[&neighbor] <= new_cost {
                continue;
            }
            processed.insert(neighbor, new_cost);
            to_process.push_back(Processable::new(neighbor, new_cost));
        }
    }
    return processed;
}

fn part1(map: &Vec<Vec<i32>>, start: Pos, end: Pos) {
    let processed = compute_cost(&map, start);
    println!("Part 1: {}", processed.get(&end).unwrap());
}

fn part2(map: &Vec<Vec<i32>>, end: Pos) {
    // Let's cheese this solution be observing that the only 'b' steps are
    // on the left hand side of the map. Therefore one can only get up
    // the hill from the left-most positions.
    let mut lowest_cost: usize = std::usize::MAX;
    for y in 0..map.len() {
        let start = Pos { x: 0, y: y as i32 };
        let processed = compute_cost(map, start);
        // println!("best 1: {}", y);
        let cost = *processed.get(&end).unwrap();
        // println!("best 1: {}", cost);
        if cost < lowest_cost {
            // println!("best 2: {}", y);
            lowest_cost = cost;
        }
    }
    
    println!("Part 2: {}", lowest_cost);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_12.data");

    let lines: Vec<&str> = input.split("\n").collect();
    let mut start_y = 0;
    let mut start_x = 0;
    let mut end_y = 0;
    let mut end_x = 0;
    let mut y = 0;
    let mut x = 0;
    let map: Vec<Vec<i32>> = lines.iter().map(|&val| {
        y += 1;
        x = 0;
        let cells: Vec<char> = val.chars().collect();
        return cells.iter().map(|&val| {
            x += 1;
            // Make comparing values easier by converting special cases like S and E
            // as numeric values.
            if val == 'S' {
                start_x = x - 1;
                start_y = y - 1;
                return 'a' as i32 - 1
            } else if val == 'E' {
                end_x = x - 1;
                end_y = y - 1;
                return 'z' as i32 + 1
            }
            return val as i32
        }).collect();
    }).collect();

    let start = Pos { x: start_x, y: start_y };
    let end = Pos { x: end_x, y: end_y };
    
    part1(&map, start, end);
    part2(&map, end);
}
