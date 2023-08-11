#![allow(dead_code)]

use std::{collections::HashMap, fs};

use nom::{multi::{separated_list1, many1}, character::complete::newline, branch::alt, IResult, bytes::complete::tag, combinator::map};

type Coord = (usize, usize);

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Pixel {
    Rock,
    Blizzard(Direction),
    Open,
}

fn parse_map(input: &str) -> IResult<&str, Vec<Pixel>> {
    let (input, map) = many1(alt((
        map(tag("."), |_| Pixel::Open),
        map(tag("#"), |_| Pixel::Rock),
        map(tag("^"), |_| Pixel::Blizzard(Direction::Up)),
        map(tag("v"), |_| Pixel::Blizzard(Direction::Down)),
        map(tag("<"), |_| Pixel::Blizzard(Direction::Left)),
        map(tag(">"), |_| Pixel::Blizzard(Direction::Right)),
    )))(input)?;

    Ok((input, map))
}

fn parse(input: &str) -> IResult<&str, HashMap<Coord, Pixel>> {
    let (input, map) = separated_list1(newline, parse_map)(input)?;

    let mut result = HashMap::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            result.insert((col, row), map[row][col].clone());
        }
    }

    Ok((input, result))
}

fn day24a(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let (_, map) = parse(&input).unwrap();
    println!("{:?}", map);
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_shortest_path() {
        let actual = day24a("./data/day24.txt");
        assert_eq!(actual, 18);
    }
}
