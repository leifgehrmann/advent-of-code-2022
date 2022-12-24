use crate::input_reader;
use std::collections::HashMap;
use regex::Regex;

pub fn run() {
    let input = input_reader::read_file_in_cwd("src/day_16.data");

    let mut valves_flow: HashMap<&str, usize> = HashMap::new();
    let mut valves_tunnels: HashMap<&str, Vec<&str>> = HashMap::new();

    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9]+); tunnels lead to valves ([A-Z, ]+)").unwrap();
    let valves_str: Vec<&str> = input.split("\n").collect();
    valves_str.iter().for_each(|&valve_str| {
        let caps = re.captures(valve_str).unwrap();
        let valve = caps.get(1).map_or("", |m| m.as_str());
        let valve_flow = caps.get(2).map_or("0", |m| m.as_str()).parse::<usize>().unwrap();
        let valve_tunnels_str = caps.get(3).map_or("", |m| m.as_str());
        let valve_tunnels: Vec<&str> = valve_tunnels_str.split(", ").collect();

        valves_flow.insert(valve, valve_flow);
        valves_tunnels.insert(valve, valve_tunnels.clone());
    });
}
