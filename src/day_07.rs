use crate::input_reader;
use std::{collections::HashMap};

struct File {
    size: usize
}

fn part1and2(
    sub_dirs: HashMap<Vec<&str>, Vec<Vec<&str>>>,
    files: HashMap<Vec<&str>, Vec<File>>,
    root: Vec<&str>
) {
    let mut dir_size: HashMap<Vec<&str>, usize> = HashMap::new();
    let cursor: &mut Vec<&str> = &mut root.clone();

    'map_sub_dir: while dir_size.get(&root).is_none() {

        let mut sum: usize = 0;

        // If there are sub_dirs, get the size of each sub_dir. If the size has
        // not been calculated, move the cursor into the map and restart the
        // count.
        for sub_dir in sub_dirs.get(cursor).unwrap() {
            if dir_size.contains_key(sub_dir) {
                sum += dir_size.get(sub_dir).unwrap();
            } else {
                cursor.push(sub_dir.last().unwrap());
                continue 'map_sub_dir;
            }
        }

        // Compute files
        for file in files.get(cursor).unwrap() {
            sum += file.size;
        }
        dir_size.insert(cursor.clone(), sum);

        // Now that we've mapped this dir, let's continue mapping the 
        // parent.
        cursor.pop();
    }

    let mut sum: usize = 0;
    for d in &dir_size {
        if *d.1 <= 100000 {
            sum += *d.1;
        }
    }

    println!("Part 1: {}", sum);

    let unused_space = 70000000 - *dir_size.get(&root).unwrap();
    let to_free = 30000000 - unused_space;
    let mut min: usize = std::usize::MAX;
    for d in &dir_size {
        if *d.1 < min && *d.1 >= to_free {
            min = *d.1;
        }
    }   
    
    println!("Part 2: {}", min);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_07.data");
    
    let mut dirs: HashMap<Vec<&str>, Vec<Vec<&str>>> = HashMap::new();
    let mut files: HashMap<Vec<&str>, Vec<File>> = HashMap::new();
    let mut path: Vec<&str> = vec![];

    // Split the input "$". Each "$" represents an executed command.
    let command_and_outputs: Vec<&str> = input.split("$ ").collect();
    // Using a casual iterator, because I don't know yet how to handle mutables
    // and closures at the same time...
    for c in 0..command_and_outputs.len() {
        let command_and_output = command_and_outputs[c];
        // The split token will be blank, because the input starts with "$ ",
        // so we skip it.
        if command_and_output == "" {
            continue;
        }

        // The first line between each command execution is the command. The
        // the rest is the output of that command.
        let (command, output) = command_and_output.split_once("\n").unwrap();

        // Parse command
        let command_split: Vec<&str> = command.split(" ").collect();
        match command_split[0] {
            "cd" => {
                let new_dir_name = command_split[1];

                if new_dir_name == ".." {
                    path.pop();
                    continue;
                }

                path.push(new_dir_name);
            },
            "ls" => {
                let mut dir_dirs = vec![];
                let mut dir_files = vec![];
                let items: Vec<&str> = output.split("\n").collect();
                // Using a casual iterator, because I don't know yet how to
                // handle mutables and closures at the same time...
                for i in 0..items.len() {
                    let item_info: Vec<&str> = items[i].split(" ").collect();
                    if item_info[0] == "dir" {
                        let sub_dir_name = item_info[1];
                        let mut path_clone = path.clone();
                        path_clone.push(sub_dir_name);
                        dir_dirs.push(path_clone);
                    } else if let Some(file_size) = item_info[0].parse::<usize>().ok() {
                        let file = File { size: file_size };
                        dir_files.push(file);
                    }
                }

                dirs.insert(path.clone(), dir_dirs);
                files.insert(path.clone(), dir_files);
            },
            _ => { /* Do nothing if command is not recognized */ }
        };
    }
    part1and2(dirs, files, vec!["/"]);
}
