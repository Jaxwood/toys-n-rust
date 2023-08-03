#![allow(dead_code)]

use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    IResult, sequence::pair, bytes::complete::tag,
};

type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
enum Pixel {
    Open,
    Void,
    Wall,
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Forward(i64),
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_jungle(input: &str) -> IResult<&str, Vec<Pixel>> {
    let (input, pixels) = many1(alt((
        complete::char(' '),
        complete::char('#'),
        complete::char('.'),
    )))(input)?;
    Ok((
        input,
        pixels
            .iter()
            .map(|c| match c {
                '.' => Pixel::Open,
                '#' => Pixel::Wall,
                _ => Pixel::Void,
            })
            .collect::<Vec<_>>(),
    ))
}

fn parse_direction(input: &str) -> IResult<&str, Vec<Move>> {
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    let (input, directions) = many1(pair(complete::i64, alt((tag("R"), tag("L")))))(input)?;
    let (input, rest) = complete::i64(input)?;
    let (input, _) = newline(input)?;
    let directions = directions.iter().flat_map(|(steps, direction)| {
        vec![Move::Forward(*steps), match direction {
            &"R" => Move::Right,
            &"L" => Move::Left,
            _ => panic!("Unknown direction: {}", direction),
        }]
    })
    .chain(vec![Move::Forward(rest)])
    .collect::<Vec<_>>();
    Ok((input, directions))
}

fn parse(input: &str) -> IResult<&str, (Vec<Move>, HashMap<Coord, Pixel>)> {
    let (input, jungle) = separated_list1(newline, parse_jungle)(input)?;

    let jungle = jungle
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (y, row)| {
            row.iter().enumerate().for_each(|(x, pixel)| {
                acc.insert((x, y), pixel.clone());
            });
            acc
        });

    let (input, directions) = parse_direction(input)?;

    Ok((input, (directions, jungle)))
}

fn day22a(path: &str) -> i64 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, _map) = parse(&content).unwrap();

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_password() {
        let actual = day22a("./data/day22.txt");
        assert_eq!(actual, 6032);
    }

    #[test]
    #[ignore]
    fn find_password_part_a() {
        let actual = day22a("./data/day22final.txt");
        assert_eq!(actual, 0);
    }
}
