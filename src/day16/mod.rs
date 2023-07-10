#![allow(dead_code)]

use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs,
};

use itertools::iproduct;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ElephantState {
    name: String,
    minute: u32,
    pub elephant: String,
    elephant_minute: u32,
    score: u32,
    visited: HashSet<String>,
}

impl ElephantState {
    fn closed(&self, valve: &str) -> bool {
        !self.visited.contains(valve)
    }

    fn score(&mut self, until: u32, minute: u32, scores: &HashMap<&str, u32>) {
        // score self
        if minute == self.minute && self.closed(self.name.as_str()) {
            self.score += (until - minute) * scores[&self.name.as_str()];
            self.visited.insert(self.name.clone());
            self.minute += 1;
        }

        // score elephant
        if minute == self.elephant_minute && self.closed(&self.elephant.as_str()) {
            self.score += (until - minute) * scores[&self.elephant.as_str()];
            self.visited.insert(self.elephant.clone());
            self.elephant_minute += 1;
        }
    }

    fn try_move(
        &self,
        minute: u32,
        can_be_opened: &HashSet<String>,
        weights: &HashMap<(String, String), (u32, u32)>,
    ) -> Vec<Option<ElephantState>> {
        if minute != self.minute && minute != self.elephant_minute {
            return vec![None];
        }

        if self.visited.is_superset(can_be_opened) {
            return vec![None];
        }

        let mut result = vec![];

        match (minute == self.minute, minute == self.elephant_minute) {
            (true, true) => {
                let candidates = can_be_opened
                    .difference(&self.visited)
                    .collect::<Vec<&String>>();

                if candidates.len() == 1 {
                    let (first, _) = weights[&(self.name.clone(), candidates[0].clone())];
                    let (second, _) = weights[&(self.elephant.clone(), candidates[0].clone())];

                    match first <= second {
                        true => {
                            let mut new_state = self.clone();
                            new_state.name = candidates[0].clone();
                            new_state.minute += first;
                            result.push(Some(new_state));
                        }
                        false => {
                            let mut new_state = self.clone();
                            new_state.elephant = candidates[0].clone();
                            new_state.elephant_minute += second;
                            result.push(Some(new_state));
                        }
                    }
                    return result;
                }

                let combis = iproduct!(candidates.clone(), candidates.clone())
                    .filter(|&(a, b)| a != b)
                    .collect::<Vec<(&String, &String)>>();

                for (a, b) in combis {
                    let mut new_state = self.clone();
                    let (first, _) = weights[&(self.name.clone(), a.clone())];
                    let (second, _) = weights[&(self.elephant.clone(), b.clone())];
                    new_state.name = a.clone();
                    new_state.elephant = b.clone();
                    new_state.minute += first;
                    new_state.elephant_minute += second;
                    result.push(Some(new_state));
                }

                result
            }
            (true, false) => {
                for valve in can_be_opened.iter() {
                    if self.closed(valve.as_str()) {
                        let mut new_state = self.clone();
                        let (moves, _) = weights[&(self.name.clone(), valve.clone())];
                        new_state.name = valve.clone();
                        new_state.minute += moves;
                        result.push(Some(new_state));
                    }
                }
                result
            }
            (false, true) => {
                for valve in can_be_opened.iter() {
                    if self.closed(valve.as_str()) {
                        let mut new_state = self.clone();
                        let (moves, _) = weights[&(self.elephant.clone(), valve.clone())];
                        new_state.elephant = valve.clone();
                        new_state.elephant_minute += moves;
                        result.push(Some(new_state));
                    }
                }
                result
            }
            (false, false) => panic!("invalid state due to previous check"),
        }
    }
}

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
            visited: HashSet::from(["AA".to_string()]),
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

fn day16a(path: &str, minutes: u32) -> u32 {
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
            .filter(|(_, (moves, _))| state.minute + moves + 1 <= minutes)
            .map(|(n, (moves, rate))| {
                State {
                    name: n.clone(),
                    score: state.score + (rate * (minutes - state.minute - moves - 1)),
                    minute: state.minute + moves + 1,
                    visited: state.visited.clone(),
                }
                .visit(n.clone())
            })
            .filter(|s| s.minute <= minutes)
            .collect();

        for target in candidates.iter() {
            queue.push(target.clone());
            best = cmp::max(best, target.score);
        }
    }

    best
}

fn day16b(path: &str, minutes: u32) -> u32 {
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

    let mut next_queue = vec![ElephantState {
        name: start.name.clone(),
        elephant: start.name.clone(),
        minute: 1,
        elephant_minute: 1,
        score: 0,
        visited: HashSet::from([start.name.clone()]),
    }];

    let mut best = 0;
    for minute in 1..=minutes {
        let mut queue = next_queue.clone();
        next_queue.clear();

        while !queue.is_empty() {
            let mut state = queue.pop().unwrap();
            best = cmp::max(best, state.score);
             
            state.score(minutes, minute, &scores);
            for states in state.try_move(minute, &can_be_opened, &weights) {
                match states {
                    Some(st) => next_queue.push(st),
                    None => next_queue.push(state.clone()),
                }
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_pressure() {
        let actual = day16a("./data/day16.txt", 30);
        assert_eq!(actual, 1651);
    }

    #[test]
    #[ignore = "slow test"]
    fn find_most_pressure_part_a() {
        let actual = day16a("./data/day16final.txt", 30);
        assert_eq!(actual, 1792);
    }

    #[test]
    fn find_most_pressure_with_elephant() {
        let actual = day16b("./data/day16.txt", 26);
        assert_eq!(actual, 1707);
    }

    #[test]
    #[ignore = "slow test"]
    fn find_most_pressure_with_elephant_part_b() {
        let actual = day16b("./data/day16final.txt", 26);
        assert_eq!(actual, 2587);
    }
}
