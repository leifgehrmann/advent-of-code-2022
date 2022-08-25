use crate::input_reader;

pub fn run() {
    println!("Sonar Sweep!");
    println!("{}", input_reader::read_file_in_cwd());
}
