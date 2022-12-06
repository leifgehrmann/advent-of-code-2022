mod input_reader;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn main() {
    let day: String = std::env::args().nth(1).expect(
        "No day given. Possible options are: 01-25."
    );
    let day_slice: &str = day.as_str();

    match day_slice {
        "01" => day_01::run(),
        "02" => day_02::run(),
        "03" => day_03::run(),
        "04" => day_04::run(),
        "05" => day_05::run(),
        "06" => day_06::run(),
        _ => println!("No valid day given. Possible options are: 01-25."),
    };
}
