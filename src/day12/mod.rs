#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use nom::{
    character::complete::{alpha1, newline},
    multi::separated_list1,
    IResult,
};

const START: u32 = 96;
const END: u32 = 123;

fn adjust_values(val: u32) -> u32 {
    // make E = 123
    if val == 69 {
        END
    // make S = 96
    } else if val == 83 {
        START
    } else {
        val
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, line) = alpha1(input)?;
    let heights = line
        .chars()
        .into_iter()
        .map(|x| x as u32)
        .map(adjust_values)
        .collect::<Vec<u32>>();
    Ok((input, heights))
}

fn parse(input: &str) -> HashMap<(i8, i8), u32> {
    let lines = separated_list1(newline, parse_line)(input);
    match lines {
        Ok((_, ys)) => ys
            .iter()
            .enumerate()
            .fold(HashMap::new(), |outer, (y, ls)| {
                ls.iter().enumerate().fold(outer, |mut inner, (x, &val)| {
                    inner.insert((x as i8, y as i8), val);
                    inner
                })
            }),
        _ => panic!("could not parse lines"),
    }
}

fn day12a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let height_map = parse(content.as_str());

    let start_location = height_map
        .iter()
        .find_map(
            |(coord, &height)| {
                if height == START {
                    Some(*coord)
                } else {
                    None
                }
            },
        )
        .expect("start location not found");

    let end_location = height_map
        .iter()
        .find_map(
            |(coord, &height)| {
                if height == END {
                    Some(*coord)
                } else {
                    None
                }
            },
        )
        .expect("start location not found");

    let mut candidates = VecDeque::from([start_location]);
    let neigh_bors = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut travel_cost: HashMap<(i8, i8), i32> = HashMap::new();
    for (coord, _) in height_map.iter() {
        travel_cost.insert(*coord, i32::MAX);
    }
    travel_cost.insert(start_location, 0);

    while candidates.len() > 0 {
        let (x, y) = candidates.pop_front().expect("no candidate found");
        let height = height_map.get(&(x, y)).expect("no height found");
        let cost = travel_cost.remove(&(x, y)).expect("no cost found");

        let paths = neigh_bors
            .iter()
            .map(|&(xx, yy)| (x + xx, y + yy))
            .filter(|coord| height_map.contains_key(coord))
            .filter(|coord| match travel_cost.get(coord) {
                Some(&val) => cost + 1 < val,
                None => false,
            })
            .filter(|coord| match height_map.get(coord) {
                Some(&candidate_height) => candidate_height <= height + 1,
                None => false,
            })
            .collect::<Vec<(i8, i8)>>();

        for path in paths {
            if path == end_location {
                return cost + 1;
            }
            travel_cost.insert(path, cost + 1);
            candidates.push_back(path);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_shortest_path() {
        let actual = day12a("./data/day12.txt");
        assert_eq!(actual, 31);
    }

    #[test]
    fn find_shortest_path_part_a() {
        let actual = day12a("./data/day12final.txt");
        assert_eq!(actual, 391);
    }
}
