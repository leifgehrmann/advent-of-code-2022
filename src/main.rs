fn main() {
    let day: String = std::env::args().nth(1).expect("no day given");

    let mut message = "Hello, ".to_owned();
    message = format!("{message}{day}");

    println!("{}", message);
}
