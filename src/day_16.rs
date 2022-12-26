use crate::input_reader;
use std::collections::HashMap;
use regex::Regex;

pub fn part1(
    valves_flow: HashMap<&str, usize>,
    valves_tunnels: HashMap<&str, Vec<&str>>,
    start: &str,
    minutes: usize
) {
    // Reduce the problem by calculating the costs between each valve that
    // has a non-zero flow value.

    // Calculate the cost from the `start` valve to all other valves.

    // We're gonna assume that we want to visit valves with the highest flow
    // first, and visit the valves with less flow last. So we compute an array
    // of valves sorted by flow.

    // Create combinations of solutions. 
    
}

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

        if valve_flow != 0 {
            println!("  {}[{} flow={}]", valve, valve, valve_flow);
        }
        for valve_tunnel in valve_tunnels.clone() {
            println!("  {} --> {}", valve, valve_tunnel);
        }
    });

    let start = "AA";
    let minutes = 30;

    part1(valves_flow, valves_tunnels, start, minutes)
}
