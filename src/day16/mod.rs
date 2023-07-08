#![allow(dead_code)]

use std::{
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
    current: String,
    score: u32,
    minute: u32,
    opened: HashSet<String>,
    previous: Option<String>,
}

impl State {
    fn new(current: String, minute: u32) -> Self {
        Self {
            current,
            score: 0,
            minute,
            opened: HashSet::new(),
            previous: None,
        }
    }

    fn move_to(&mut self, name: &String) {
        self.minute += 1;
        self.previous = Some(self.current.clone());
        self.current = name.clone();
        // println!("You move to valve {}", name);
    }

    fn open(&mut self, rate: u32) -> bool {
        self.minute += 1;
        self.score += (30 - self.minute) * rate;
        self.previous = None;
        // println!("You open valve {}", self.current);
        self.opened.insert(self.current.clone())
    }

    fn is_open(&self, rate: u32) -> bool {
        if rate == 0 {
            return true;
        }
        self.opened.contains(self.current.as_str())
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

fn day16a(path: &str) -> usize {
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

    let mut queue = Vec::from([State::new(start.name.clone(), 0)]);
    let mut best: HashSet<u32> = HashSet::new();

    while !queue.is_empty() {
        queue.sort_by(|a, b| a.score.cmp(&b.score));
        let state = queue.pop().unwrap();

        // only run for 30 minutes
        if state.minute >= 30 {
            continue;
        }

        // has all valves been opened?
        if state.opened.is_superset(&can_be_opened) {
            best.insert(state.score);
            continue;
        }

        // if valve is closed, open it and continue if it has a positive rate
        let rate = *scores.get(state.current.as_str()).unwrap();
        if !state.is_open(rate) {
            let mut new_state = state.clone();
            new_state.open(rate);
            queue.push(new_state);
        }

        // for each move to it and continue
        let others = connections.get(state.current.as_str()).unwrap();
        for valve in others {
            // move to next valve
            match state.previous {
                Some(ref previous) if previous == valve => continue,
                _ => {
                    let mut new_state = state.clone();
                    new_state.move_to(valve);
                    queue.push(new_state);
                }
            }
        }
    }

    match best.iter().max() {
        Some(max) => *max as usize,
        None => 0,
    }
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
    #[ignore]
    fn find_most_pressure_part_a() {
        let actual = day16a("./data/day16final.txt");
        assert_eq!(actual, 1651);
    }

    #[test]
    fn can_move_and_open_valve() {
        let mut actual = State::new("AA".to_string(), 0);

        // first minute
        actual.move_to(&"BB".to_string());
        assert_eq!(actual.previous, Some("AA".to_string()));

        // second minute
        actual.open(13);

        assert_eq!(actual.previous, None);
        assert_eq!(actual.minute, 2);
        assert_eq!(actual.score, 28 * 13);
        assert_eq!(actual.is_open(13), true);
    }
}
