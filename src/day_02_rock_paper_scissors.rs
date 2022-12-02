use crate::input_reader;

#[derive(Clone, PartialEq, Eq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct UnexplainedStrat {
    opponent: Play,
    response: Play,
}

fn part1(strats: Vec<UnexplainedStrat>) {
    let mut score = 0;
    for strat in strats {
        // Lose scenario.
        if (strat.opponent == Play::Paper && strat.response == Play::Rock) ||
            (strat.opponent == Play::Scissors && strat.response == Play::Paper) ||
            (strat.opponent == Play::Rock && strat.response == Play::Scissors) {
            score += 0 + strat.response as i32;
            continue;
        }
        // Draw scenario.
        if strat.opponent == strat.response {
            score += 3 + strat.response as i32;
            continue;
        }
        // Win scenario.
        score += 6 + strat.response as i32
    }
    println!("Part 1: {}", score);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_02_input.txt");

    // Parse the instructions.
    let lines: Vec<&str> = input.split('\n').collect();
    let encrypted_strats: Vec<UnexplainedStrat> = lines.iter().map(|&val| {
        let line_split: Vec<&str> = val.split(' ').collect();
        let opponent: Play = match line_split[0] {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => Play::Scissors,
        };
        let response: Play = match line_split[1] {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => Play::Scissors,
        };
        return UnexplainedStrat {
            opponent,
            response
        }
    }).collect();

    part1(encrypted_strats);
}
