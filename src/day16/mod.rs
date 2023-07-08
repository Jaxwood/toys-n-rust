#![allow(dead_code)]

use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
struct State {
    name: String,
    score: u32,
    minute: u32,
    visited: HashSet<String>,
}

impl State {
    fn new(name: String, minute: u32) -> Self {
        Self {
            name,
            score: 0,
            minute,
            visited: HashSet::new(),
        }
    }

    fn visit(&mut self, valve: String) -> Self {
        self.visited.insert(valve);
        self.clone()
    }
}

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<String>,
}

impl Valve {
    fn new(name: String, rate: u32, tunnels: Vec<String>) -> Self {
        Self {
            name,
            rate,
            tunnels,
        }
    }

    fn connected_to(&self) -> Vec<String> {
        self.tunnels.clone()
    }
}

fn parse_tunnels(input: &str) -> IResult<&str, Vec<String>> {
    separated_list0(tag(", "), alpha1)(input)
        .map(|(input, tunnels)| (input, tunnels.iter().map(|t| t.to_string()).collect()))
}

fn parse_tunnel(input: &str) -> IResult<&str, Vec<String>> {
    alpha1(input).map(|(input, tunnel)| (input, vec![tunnel.to_string()]))
}

fn parse_valves(input: &str) -> IResult<&str, Valve> {
    tuple((
        tag("Valve "),
        alpha1,
        tag(" has flow rate="),
        nom::character::complete::u32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        alt((parse_tunnels, parse_tunnel)),
    ))(input)
    .map(|(input, (_, valve, _, rate, _, tunnels))| {
        (input, Valve::new(valve.to_string(), rate, tunnels))
    })
}

fn parse(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(newline, parse_valves)(input).map(|(input, readings)| (input, readings))
}

fn shortest_path(connections: &HashMap<&str, Vec<String>>, name: &str, target: &str) -> u32 {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue = Vec::new();
    let mut distance = HashMap::new();

    queue.push(name);
    distance.insert(name, 0);

    while !queue.is_empty() {
        let current = queue.remove(0);
        visited.insert(current.to_string());

        if current == target {
            return distance[current];
        }

        for neighbor in connections[current].iter() {
            if !visited.contains(neighbor) {
                queue.push(neighbor);
                distance.insert(neighbor, distance[current] + 1);
            }
        }
    }

    0
}

fn day16a(path: &str) -> u32 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, valves) = parse(content.as_str()).expect("parsing failed");

    let start = valves
        .iter()
        .find(|v| v.name == "AA")
        .expect("no start valve");

    let connections = valves
        .iter()
        .map(|v| (v.name.as_str(), v.connected_to()))
        .collect::<HashMap<&str, Vec<String>>>();

    let scores = valves
        .iter()
        .map(|v| (v.name.as_str(), v.rate))
        .collect::<HashMap<&str, u32>>();

    let can_be_opened = valves
        .iter()
        .filter(|v| v.rate > 0)
        .map(|v| v.name.clone())
        .collect::<HashSet<String>>();

    let mut weights = HashMap::new();
    for valve in valves.iter() {
        for valve_target in can_be_opened.iter() {
            if valve.name == *valve_target {
                continue;
            }
            let moves = shortest_path(&connections, &valve.name, valve_target);
            weights.insert(
                (valve.name.clone(), valve_target.clone()),
                (moves, scores[valve_target.as_str()]),
            );
        }
    }

    let mut best = 0;
    let mut queue = vec![State::new(start.name.clone(), 0)];
    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        let candidates: Vec<_> = can_be_opened
            .iter()
            .filter(|&v| weights.contains_key(&(state.name.clone(), v.clone())))
            .filter(|v| !state.visited.contains(*v))
            .map(|v| (v, weights[&(state.name.clone(), v.clone())]))
            .filter(|(_, (moves, _))| state.minute + moves + 1 <= 30)
            .map(|(n, (moves, rate))| {
                State {
                    name: n.clone(),
                    score: state.score + (rate * (30 - state.minute - moves - 1)),
                    minute: state.minute + moves + 1,
                    visited: state.visited.clone(),
                }
                .visit(n.clone())
            })
            .filter(|s| s.minute <= 30)
            .collect();

        for candidate in candidates.iter() {
            queue.push(candidate.clone());
            best = cmp::max(best, candidate.score);
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_pressure() {
        let actual = day16a("./data/day16.txt");
        assert_eq!(actual, 1651);
    }

    #[test]
    fn find_most_pressure_part_a() {
        let actual = day16a("./data/day16final.txt");
        assert_eq!(actual, 1792);
    }
}
