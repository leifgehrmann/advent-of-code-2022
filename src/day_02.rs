use crate::input_reader;

#[derive(PartialEq, Eq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq)]
enum EndResult {
    Lose,
    Draw,
    Win,
}

struct UnexplainedStrat {
    opponent: Play,
    response: Play,
}

struct ExplainedStrat {
    opponent: Play,
    response: EndResult,
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

fn part2(strats: Vec<ExplainedStrat>) {
    let mut score = 0;
    for strat in strats {
        // Lose scenario.
        if strat.response == EndResult::Lose {
            let response_play: Play = match strat.opponent {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            };
            score += 0 + response_play as i32;
            continue;
        }
        // Draw scenario.
        if strat.response == EndResult::Draw {
            score += 3 + strat.opponent as i32;
            continue;
        }
        // Win scenario.
        let response_play: Play = match strat.opponent {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        };
        score += 6 + response_play as i32;
    }
    println!("Part 2: {}", score);
}

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_02.data");

    // Parse the instructions.
    let lines: Vec<&str> = input.split('\n').collect();
    let unexplained_strats: Vec<UnexplainedStrat> = lines.iter().map(|&val| {
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

    part1(unexplained_strats);

    // Now that the strat is explained, parse the instructions again.
    let lines: Vec<&str> = input.split('\n').collect();
    let explained_strats: Vec<ExplainedStrat> = lines.iter().map(|&val| {
        let line_split: Vec<&str> = val.split(' ').collect();
        let opponent: Play = match line_split[0] {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => Play::Scissors,
        };
        let response: EndResult = match line_split[1] {
            "X" => EndResult::Lose,
            "Y" => EndResult::Draw,
            "Z" => EndResult::Win,
            _ => EndResult::Win,
        };
        return ExplainedStrat {
            opponent,
            response
        }
    }).collect();

    part2(explained_strats);
}
