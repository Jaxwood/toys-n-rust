#![allow(dead_code)]

use core::fmt;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    fs, cmp,
};

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug)]
struct Reading {
    sensor: Coord,
    beacon: Coord,
    distance: i32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Reading {
    fn new(sensor: Coord, beacon: Coord) -> Reading {
        let distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
        Reading {
            sensor,
            beacon,
            distance,
        }
    }

    fn within_range(&self, y: i32) -> bool {
        self.sensor.y - self.distance as i32 <= y && y <= self.sensor.y + self.distance as i32
    }

    fn calculate_distance_to(&self, (x, y): (i32, i32)) -> i32 {
        (self.sensor.x - x).abs() + (self.sensor.y - y).abs()
    }

    fn border_coords(&self, offset: i32) -> Vec<(i32, i32)> {
        let mut coords = Vec::new();
        for y in self.sensor.y - self.distance - offset..=self.sensor.y + self.distance + offset {
            for x in self.sensor.x - self.distance - offset..=self.sensor.x + self.distance + offset
            {
                if self.calculate_distance_to((x, y)) == self.distance + offset {
                    coords.push((x, y));
                }
            }
        }
        coords
    }

    fn coords_on_y_axis(&self, y: i32) -> Vec<(i32, i32)> {
        let min = self.sensor.x - self.distance;
        let max = self.sensor.x + self.distance;
        (min..=max)
            .map(|x| (x, y))
            .filter(|&coord| Coord::from(coord) != self.beacon)
            .filter(|&coord| self.calculate_distance_to(coord) <= self.distance)
            .collect()
    }
}

impl Display for Reading {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "(sensor: {}, beacon: {}) distance {}",
            self.sensor, self.beacon, self.distance
        )
    }
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    separated_pair(
        nom::character::complete::i32,
        tag(", y="),
        nom::character::complete::i32,
    )(input)
    .map(|(input, (x, y))| (input, Coord { x, y }))
}

fn parse_reading(input: &str) -> IResult<&str, Reading> {
    tuple((
        tag("Sensor at x="),
        parse_coord,
        tag(": closest beacon is at x="),
        parse_coord,
    ))(input)
    .map(|(input, (_, sensor, _, beacon))| (input, Reading::new(sensor, beacon)))
}

fn parse(input: &str) -> IResult<&str, Vec<Reading>> {
    separated_list1(newline, parse_reading)(input).map(|(input, readings)| (input, readings))
}

fn day15a(path: &str, y: i32) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, readings) = parse(content.as_str()).expect("parsing failed");
    readings
        .iter()
        .filter(|reading| reading.within_range(y))
        .flat_map(|reading| reading.coords_on_y_axis(y))
        .collect::<HashSet<(i32, i32)>>()
        .iter()
        .count()
}

fn print(candidates: &HashSet<(i32, i32)>) {
    for y in -15..30 {
        let mut line = String::new();
        for x in -10..35 {
            if candidates.contains(&(x, y)) {
                line.push('#');
            } else if x == 14 && y == 11 {
                line.push('o');
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
    }
}

fn day15b(path: &str, max: i32) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, readings) = parse(content.as_str()).expect("parsing failed");

    let readings: Vec<_> = readings
        .iter()
        .filter(|reading| reading.beacon.x <= max && reading.beacon.y <= max)
        .collect();
    let candidates: HashSet<_> = readings
        .iter()
        .flat_map(|reading| reading.border_coords(1))
        .filter(|(x, y)| {
            !readings
                .iter()
                .any(|r| r.calculate_distance_to((*x, *y)) <= r.distance)
        })
        .map(|(x, y)| {
            let borders = readings.iter().fold(0, |acc, r| {
                if r.calculate_distance_to((x, y)) == r.distance + 1 {
                    acc + 1
                } else {
                    acc
                }
            });
            (x, y, borders)
        })
        .collect();
    let mut best = 0;
    let mut result = (0, 0);
    for (x, y, cnt) in candidates {
        if cnt > best {
            best = cmp::max(best, cnt);
            result = (x, y);
        }
    }
    4000000 * result.0 as usize + result.1 as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_beacon_on_y_axis() {
        let actual = day15a("./data/day15.txt", 10);
        assert_eq!(actual, 26);
    }

    #[test]
    fn find_beacon() {
        let actual = day15b("./data/day15.txt", 20);
        assert_eq!(actual, 56000011);
    }

    #[test]
    fn find_beacon_part_b() {
        let actual = day15b("./data/day15final.txt", 4000000);
        assert_eq!(actual, 56000011);
    }

    #[test]
    #[ignore]
    fn find_beacon_on_y_axis_part_a() {
        let actual = day15a("./data/day15final.txt", 2000000);
        assert_eq!(actual, 4717631);
    }
}
