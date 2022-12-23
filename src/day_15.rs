use crate::input_reader;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Scan {
    sensor: Point,
    beacon: Point,
}

// Ranges are inclusive, meaning Range {x: 1, y: 5} = 1,2,3,4,5
#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

// Returns a range of x-cooridinates that have been scanned by the sensor.
//
// In the example below, the scanned line --RRRRRRRRR---- would be:
// Range { start: 2, end: 10 }
//
//           1
// 0    5    0
// ...............
// ......x........
// .....x.x.......
// ....B...x......
// ...x.....x.....
// --RRRRRRRRR----
// .x....X....x...
// ..x.......x....
// ...x.....x.....
// ....x...x......
// .....x.x.......
// ......x........
fn get_x_range(scan: &Scan, inspect_y: i32) -> Option<Range> {
    let distance = (scan.beacon.x - scan.sensor.x).abs() + (scan.beacon.y - scan.sensor.y).abs();
    if inspect_y > scan.sensor.y + distance || inspect_y < scan.sensor.y - distance {
        return None
    }

    let delta = (scan.sensor.y - inspect_y).abs();
    let mut start = scan.sensor.x - distance + delta;
    let mut end = scan.sensor.x + distance - delta;
    if inspect_y == scan.beacon.y {
        if scan.beacon.x == scan.sensor.x {
            
        } else if scan.beacon.x <= scan.sensor.x {
            start += 1;
        } else {
            end -= 1;
        }
    }
    if start == end {
        return None
    }
    return Some(Range {
        start,
        end,
    })
}

// Returns the range that comes after the position x. 
fn get_next_range(ranges: &Vec<Range>, x: i32) -> Option<Range> {
    let mut min_range: Option<Range> = None;
    for range in ranges {
        if range.end < x {
            continue;
        }
        if min_range.is_some() && range.start > min_range.unwrap().start {
            continue;
        }
        min_range = Some(range.clone());
    }
    return min_range
}

fn part1(scans: &Vec<Scan>, inspect_y: i32) {
    let mut ranges = vec![];
    let mut min_range_opt = None;
    let mut min_x = i32::MAX;
    for scan in scans {
        let range_opt = get_x_range(scan, inspect_y);
        if range_opt.is_some() {
            let range = range_opt.unwrap();
            ranges.push(range);
            if range.start < min_x {
                min_x = range.start;
                min_range_opt = Some(range); 
            }
        }
    }

    if min_range_opt.is_none() {
        println!("Part 1: 0");
    }
    let min_range = min_range_opt.unwrap();

    // Sweep through the ranges counting how many places that have been scanned.
    let mut cursor = min_x;
    let mut cursor_range = min_range;
    let mut count = 0;
    loop {
        // Sum up the current range.
        count += cursor_range.end - cursor + 1; // +1 to account for the fact that the ranges are "inclusive".
        cursor = cursor_range.end + 1;

        // The find the next range.
        let next_range = get_next_range(&ranges, cursor);
        if next_range.is_some() {
            cursor = i32::max(cursor, next_range.unwrap().start);
            cursor_range = next_range.unwrap();
            continue;
        }

        break;
    }

    println!("Part 1: {}", count);
}

pub fn run() {
    let mut input = input_reader::read_file_in_cwd("src/day_15.data");
    input = input
        .replace("=", " ")
        .replace(", ", " ")
        .replace(": ", " ");

    let scans_str: Vec<&str> = input.split("\n").collect();
    let scans: Vec<Scan> = scans_str.iter().map(|&val| {
        let scan_str: Vec<&str> = val.split(" ").collect();
        
        return Scan {
            sensor: Point {
                x: scan_str[3].parse::<i32>().unwrap(),
                y: scan_str[5].parse::<i32>().unwrap(),
            },
            beacon: Point {
                x: scan_str[11].parse::<i32>().unwrap(),
                y: scan_str[13].parse::<i32>().unwrap(),
            }
        };
    }).collect();

    part1(&scans, 2000000);
    // part1(&scans, 10);
}
