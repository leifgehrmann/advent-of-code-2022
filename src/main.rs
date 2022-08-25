mod input_reader;
mod day_01_sonar_sweep;
mod day_02_dive;

fn main() {
    let day: String = std::env::args().nth(1).expect(
        "No day given. Possible options are: 01-25."
    );
    let day_slice: &str = day.as_str();

    match day_slice {
        "01" => day_01_sonar_sweep::run(),
        "02" => day_02_dive::run(),
        _ => println!("No valid day given. Possible options are: 01-25."),
    };
}
