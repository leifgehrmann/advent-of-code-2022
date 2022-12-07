use crate::input_reader;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

struct File {
    name: String,
    size: i32
}

struct Dir {
    parent: Option<String>,
    name: String,
    dirs: Vec<String>,
    files: Vec<File>
}

// impl Hash for Dir {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//     }
// }

// impl PartialEq for Dir {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name
//     }
// }
// impl Eq for Dir {}

impl Dir {
    pub fn add_file(&mut self, file: File) {
        self.files.push(file)
    }

    pub fn add_dir(&mut self, dir: String) {
        self.dirs.push(dir)
    }
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_07.data");
    
    let root = Dir { parent: None, name: "/".to_string(), dirs: vec![], files: vec![] };
    let mut dirs: HashMap<&str, Rc<RefCell<Dir>>> = HashMap::new();
    let mut current_dir_name = "/";
    dirs.insert(&current_dir_name,Rc::new(RefCell::new(root)));


    // Split the input "$". Each "$" represents an executed command.
    let command_and_outputs: Vec<&str> = input.split("$ ").collect();
    // Using a casual iterator, because I don't know yet how to handle mutables
    // and closures at the same time...
    for c in 0..command_and_outputs.len() {
        let command_and_output = command_and_outputs[c];
        // The split token will be blank, because the input starts with "$ ",
        // so we skip it.
        if command_and_output == "" {
            return 
        }

        // The first line between each command execution is the command. The
        // the rest is the output of that command.
        let (command, output) = command_and_output.split_once("\n").unwrap();

        // Parse command
        let command_split: Vec<&str> = command.split(" ").collect();
        match command_split[0] {
            "cd" => {
                let new_dir_name = command_split[1];

                // Skip "/", as that's the initial state we are in.
                if new_dir_name == "/" {
                    return;
                }

                if new_dir_name == ".." {
                    let thing = dirs.get(current_dir_name).unwrap().borrow();
                    current_dir_name = thing.parent.as_ref().unwrap().as_str();
                    return;
                }

                current_dir_name = new_dir_name
            },
            "ls" => {
                let items: Vec<&str> = output.split("\n").collect();
                // Using a casual iterator, because I don't know yet how to
                // handle mutables and closures at the same time...
                for i in 0..items.len() {
                    let item_info: Vec<&str> = items[i].split(" ").collect();
                    if item_info[0] == "dir" {
                        let sub_dir_name = item_info[0];
                        // let sub_dir = &mut Dir { parent: Some(dir.to_string()), name: sub_dir_name.to_string(), dirs: vec![], files: vec![] };
                        let current_dir = dirs.get_mut(current_dir_name).unwrap().borrow_mut();
                        //current_dir.add_dir(sub_dir_name.to_string());
                        let sub_dir = Box::new(&mut Dir { parent: None, name: "/".to_string(), dirs: vec![], files: vec![] });
                        // let sub_dir = &mut Dir { parent: Some(current_dir_name.to_string()), name: sub_dir_name.to_string(), dirs: vec![], files: vec![] }
                        //dirs.insert(sub_dir_name, sub_dir);
                    } else {
                        let file_size = item_info[0].parse::<i32>().unwrap();
                        let file_name = item_info[1];
                        let new_file = File { size: file_size, name: file_name.to_string() };
                        //dirs.get_mut(current_dir_name).unwrap().add_file(new_file);
                    }
                }
            },
            _ => { /* Do nothing if command is not recognized */ }
        };
    }
}
