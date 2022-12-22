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

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}


// ...............
// ......x........
// .....x.x.......
// ....B...x......
// ...x.....x.....
// --x-------x----
// .x....X....x...
// ..x.......x....
// ...x.....x.....
// ....x...x......
// .....x.x.......
// ......x........

fn get_x_range(scan: &Scan, inspect_y: i32) -> Option<Range> {
    let distance = (scan.beacon.x - scan.sensor.x).abs() + (scan.beacon.y - scan.sensor.y).abs();
    println!("- distance {}, {}, {}", distance, scan.sensor.y + distance, scan.sensor.y - distance);
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

fn part1(scans: &Vec<Scan>, inspect_y: i32) {
    let ranges = 
    for scan in scans {
        let range = get_x_range(scan, inspect_y);
        if range.is_some() {
            println!("{},{}: {}-{}", scan.sensor.x, scan.sensor.y, range.unwrap().start, range.unwrap().end);
        } else {
            println!("{},{}: No intercept", scan.sensor.x, scan.sensor.y);
        }
    }
    let mut min_x = 
    if 
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

    part1(&scans, 10);
}
