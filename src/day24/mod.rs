#![allow(dead_code)]

use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

struct State {
    map: HashMap<Coord, Vec<Pixel>>,
    height: usize,
    width: usize,
    start: Coord,
    end: Coord,
}

type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

fn parse(input: &str) -> IResult<&str, HashMap<Coord, Vec<Pixel>>> {
    let (input, map) = separated_list1(newline, parse_map)(input)?;

    let mut result = HashMap::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            result.insert((col, row), vec![map[row][col].clone()]);
        }
    }

    Ok((input, result))
}

impl State {

    fn set_start_end(&mut self) {
        let start = self
            .map
            .iter()
            .filter(|(k, _)| k.1 == 0)
            .filter(|(_, v)| **v == vec![Pixel::Open])
            .map(|(k, _)| k)
            .next()
            .unwrap();
        let row_max = self.map.keys().max_by_key(|(_, y)| y).unwrap().1;
        let end = self
            .map
            .iter()
            .filter(|(k, _)| k.1 == row_max)
            .filter(|(_, v)| **v == vec![Pixel::Open])
            .map(|(k, _)| k)
            .next()
            .unwrap();

        self.start = *start;
        self.end = *end;
    }

    fn set_width_height(&mut self) {
        let col_max = self.map.keys().max_by_key(|(x, _)| x).unwrap().0 - 1;
        let row_max = self.map.keys().max_by_key(|(_, y)| y).unwrap().1 - 1;
        self.width = col_max;
        self.height = row_max;
    }

    fn tick(&mut self) {
        for ((x, y), pixels) in self.map.iter_mut() {
            for pixel in pixels {
                match pixel {
                    Pixel::Blizzard(dir) => match dir {
                        Direction::Up => todo!(),
                        Direction::Down => todo!(),
                        Direction::Left => todo!(),
                        Direction::Right => todo!(),
                    },
                    _ => (),
                }
            }
        }
    }
}

fn day24a(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let (_, map) = parse(&input).unwrap();
    let mut state = State {
        map: map.clone(),
        width: 0,
        height: 0,
        start: (0, 0),
        end: (0, 0),
    };

    state.set_width_height();
    state.set_start_end();

    state.tick();

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

