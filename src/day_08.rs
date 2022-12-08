use crate::input_reader;

fn part1(map: Vec<Vec<u32>>) {
    let h = map.len(); // height
    let w = map[0].len(); // width

    // ℹ️ Below was my first attempt, which failed horrifically. The idea was
    // to map out from each viewpoint what the highest tree was. This solution
    // had trouble with trees that were the same height. Gave up because I
    // wasted so much time on it...

    // let mut map_down = vec![vec![0; w]; h];
    // let mut map_up = vec![vec![0; w]; h];
    // let mut map_to_right = vec![vec![0; w]; h];
    // let mut map_to_left = vec![vec![0; w]; h];

    // for y in 0..h {
    //     for x in 0..w {
    //         // Map trees going down.
    //         if y == 0 {
    //             map_down[y][x] = map[y][x]
    //         } else if map_down[y-1][x] > map[y][x] {
    //             map_down[y][x] = map_down[y-1][x]
    //         } else if map_down[y-1][x] == map[y][x] {
    //             map_down[y][x] = map[y-1][x] + 1
    //         } else {
    //             map_down[y][x] = map[y][x]
    //         }

    //         // Map trees going up.
    //         if y == 0 {
    //             map_up[h - y - 1][x] = map[h - y - 1][x]
    //         } else if map_up[h - y - 1 + 1][x] > map[h - y - 1][x] {
    //             map_up[h - y - 1][x] = map_up[h - y - 1 + 1][x]
    //         } else if map_up[h - y - 1 + 1][x] == map[h - y - 1][x] {
    //             map_up[h - y - 1][x] = map[h - y - 1 + 1][x] + 1
    //         } else {
    //             map_up[h - y - 1][x] = map[h - y - 1][x]
    //         }

    //         // // Map trees going right.
    //         if x == 0 {
    //             map_to_right[y][x] = map[y][x]
    //         } else if map_to_right[y][x - 1] > map[y][x] {
    //             map_to_right[y][x] = map_to_right[y][x - 1]
    //         } else if map_to_right[y][x - 1] == map[y][x] {
    //             map_to_right[y][x] = map[y][x - 1] + 1
    //         } else {
    //             map_to_right[y][x] = map[y][x]
    //         }

    //         // // Map trees going left.
    //         if x == 0 {
    //             map_to_left[y][w - x - 1] = map[y][w - x - 1]
    //         } else if map_to_left[y][w - x - 1 + 1] > map[y][w - x - 1] {
    //             map_to_left[y][w - x - 1] = map_to_left[y][w - x - 1 + 1]
    //         } else if map_to_left[y][w - x - 1 + 1] == map[y][w - x - 1] {
    //             map_to_left[y][w - x - 1] = map[y][w - x - 1 + 1] + 1
    //         } else {
    //             map_to_left[y][w - x - 1] = map[y][w - x - 1]
    //         }
    //     }
    // }

    // Print statements to debug the code...
    // let pos_y = 2;
    // let pos_x = 2;
    // // println!("Part 1: {}", map[pos_y][pos_x]);
    // println!("Part 1: {}", map_down[pos_y][pos_x]);
    // println!("Part 1: {}", map_up[pos_y][pos_x]);
    // println!("Part 1: {}", map_to_left[pos_y][pos_x]);
    // println!("Part 1: {}", map_to_right[pos_y][pos_x]);

    // let mut count = 0;
    // for y in 0..h {
    //     for x in 0..w {
    //         if map[y][x] >= map_down[y][x] ||
    //         map[y][x] >= map_up[y][x] ||
    //         map[y][x] >= map_to_right[y][x] ||
    //         map[y][x] >= map_to_left[y][x] {
    //             count += 1;
    //         }
    //     }
    // }
    // println!("Part 1: {}", count);

    // ℹ️ Second attempt, using a sub-optimal approach of analysing each tree
    // individually.
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            let cell = map[y][x];
            let mut visible_top = true;
            let mut visible_bottom = true;
            let mut visible_left = true;
            let mut visible_right = true;
            for dx in 0..x {
                if dx != x && map[y][dx] >= cell {
                    visible_top = false;
                    break;
                }
            }
            for dx in x+1..w {
                if dx != x && map[y][dx] >= cell {
                    visible_bottom = false;
                    break;
                }
            }
            for dy in 0..y {
                if dy != y && map[dy][x] >= cell {
                    visible_left = false;
                    break;
                }
            }
            for dy in y+1..h {
                if dy != y && map[dy][x] >= cell {
                    visible_right = false;
                    break
                }
            }
            if visible_top || visible_bottom || visible_left || visible_right {
                count += 1;
            }
        }
    }
    
    println!("Part 1: {}", count);
}

fn part2(map: Vec<Vec<u32>>) {
    let h = map.len(); // height
    let w = map[0].len(); // width
    let mut scenic_score_max = 0;
    for y in 0..h {
        for x in 0..w {
            let cell = map[y][x];
            let mut visible_top = 0;
            let mut visible_bottom = 0;
            let mut visible_left = 0;
            let mut visible_right = 0;
            for dx in (0..x).rev() {
                visible_top += 1;
                if map[y][dx] >= cell {
                    break
                }
            }
            for dx in x+1..w {
                visible_bottom += 1;
                if map[y][dx] >= cell {
                    break
                }
            }
            for dy in (0..y).rev() {
                visible_left += 1;
                if map[dy][x] >= cell {
                    break
                }
            }
            for dy in y+1..h {
                visible_right += 1;
                if map[dy][x] >= cell {
                    break
                }
            }
            let scenic_score = visible_top * visible_bottom * visible_left * visible_right;
            if scenic_score > scenic_score_max {
                scenic_score_max = scenic_score;
            }
        }
    }
    
    println!("Part 2: {}", scenic_score_max);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_08.data");

    let lines: Vec<&str> = input.split("\n").collect();
    let map: Vec<Vec<u32>> = lines.iter().map(|&line| {
        return line.chars().map(|cell| {
            return cell.to_digit(10).unwrap();
        }).collect();
    }).collect();

    part1(map.clone());
    part2(map.clone());
}
