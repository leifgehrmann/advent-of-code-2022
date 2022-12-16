use crate::input_reader;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Bounds {
    min_x: i32,
    // max_x: i32,
    min_y: i32,
    // max_y: i32,
}

fn point_to_arr(point: &Point, bounds: &Bounds) -> (usize, usize) {
    return (
        (point.y - bounds.min_y) as usize,
        (point.x - bounds.min_x) as usize
    )
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Cell {
    Air,
    Wall,
    Sand
}

fn part1(walls: &Vec<Vec<Point>>, sand_origin: Point) {
    let mut min_x = sand_origin.x;
    let mut max_x = sand_origin.x;
    let mut min_y = sand_origin.y;
    let mut max_y = sand_origin.y;

    for wall in walls {
        for point in wall {
            if point.x < min_x {
                min_x = point.x
            }
            if point.x > max_x {
                max_x = point.x
            }
            if point.y < min_y {
                min_y = point.y
            }
            if point.y > max_y {
                max_y = point.y
            }
        }
    }
    // Allow the sand to fall on the sides
    min_x -= 2;
    max_x += 2;
    max_y += 1;

    // let bounds = Bounds { min_x, max_x, min_y, max_y };
    let bounds = Bounds { min_x, min_y };

    let w = max_x - min_x;
    let h = max_y - min_y;

    let mut map = vec![vec![Cell::Air; w as usize]; h as usize];
    for wall in walls {
        for p_i in 1..wall.len() {
            let point_1 = wall[p_i - 1];
            let point_2 = wall[p_i];
            if point_1.x == point_2.x {
                for p_y in std::cmp::min(point_1.y, point_2.y)..=std::cmp::max(point_1.y, point_2.y) {
                    let arr_i = point_to_arr(&Point {x: point_1.x, y: p_y}, &bounds);
                    map[arr_i.0][arr_i.1] = Cell::Wall
                }
            } else {
                for p_x in std::cmp::min(point_1.x, point_2.x)..=std::cmp::max(point_1.x, point_2.x) {
                    let arr_i = point_to_arr(&Point {x: p_x, y: point_1.y}, &bounds);
                    map[arr_i.0][arr_i.1] = Cell::Wall
                }
            }
        }
    }

    let mut drops = 0;
    let mut sand = sand_origin.clone();
    loop {
        if sand.y >= max_y - 1 {
            break;
        }
        let arr_i = point_to_arr(&sand, &bounds);
        // println!("i: {}, {}, {:?}", sand.x, sand.y, map[arr_i.0][arr_i.1]);
        // println!("L: {:?}, M: {:?}, R: {:?}", map[arr_i.0 + 1][arr_i.1 - 1], map[arr_i.0 + 1][arr_i.1], map[arr_i.0 + 1][arr_i.1 + 1]);
        if map[arr_i.0 + 1][arr_i.1] == Cell::Air {
            sand.y += 1;
        } else if map[arr_i.0 + 1][arr_i.1 - 1] == Cell::Air {
            sand.x -= 1;
            sand.y += 1;
        } else if map[arr_i.0 + 1][arr_i.1 + 1] == Cell::Air {
            sand.x += 1;
            sand.y += 1;
        } else {
            // print_map(&map);
            // println!("Drop!: {:?}", sand);
            drops += 1;
            map[arr_i.0][arr_i.1] = Cell::Sand;
            sand = sand_origin.clone();
            // println!("w!: {}, {}, {}-{}, {:?}, {:?}", w, h, bounds.min_x, bounds.max_x, sand_origin, point_to_arr(&sand, &bounds));
        }
    }

    println!("Part 1: {}", drops)
}

// fn print_map(map: &Vec<Vec<Cell>>) {
//     for row in map {
//         print!("|");
//         for col in row {
//             match col {
//                 Cell::Air => print!(" "),
//                 Cell::Sand => print!("O"),
//                 Cell::Wall => print!("#"),
//             }
//         }
//         print!("|");
//         println!();
//     }
//     println!();
// }

part2

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_14.data");

    let walls_str: Vec<&str> = input.split("\n").collect();
    let walls: Vec<Vec<Point>> = walls_str.iter().map(|&val| {
        let wall_str: Vec<&str> = val.split(" -> ").collect();
        let wall: Vec<Point> = wall_str.iter().map(|&val| {
            let (x_str, y_str) =  val.split_once(",").unwrap();
            return Point {
                x: x_str.parse::<i32>().unwrap(),
                y: y_str.parse::<i32>().unwrap()
            }
        }).collect();
        return wall;
    }).collect();

    part1(&walls, Point { x: 500, y: 0})
}
