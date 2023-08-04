#![allow(dead_code)]

use std::{collections::HashMap, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    sequence::pair,
    IResult,
};

type Coord = (usize, usize);

struct Person {
    facing: Direction,
    position: Coord,
    jungle: HashMap<Coord, Pixel>,
    is_cube: bool,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Person {
    fn turn(&mut self, towards: &Move) {
        match (&self.facing, towards) {
            (Direction::North, Move::Left) => self.facing = Direction::West,
            (Direction::North, Move::Right) => self.facing = Direction::East,
            (Direction::East, Move::Left) => self.facing = Direction::North,
            (Direction::East, Move::Right) => self.facing = Direction::South,
            (Direction::South, Move::Left) => self.facing = Direction::East,
            (Direction::South, Move::Right) => self.facing = Direction::West,
            (Direction::West, Move::Left) => self.facing = Direction::South,
            (Direction::West, Move::Right) => self.facing = Direction::North,
            (_, _) => panic!("Unknown turn: {:?} {:?}", self.facing, towards),
        }
    }

    fn wrap(&mut self, (x, y): &Coord) {
        let coord = match self.facing {
            Direction::North => self
                .jungle
                .iter()
                .filter(|((xx, _), _)| *xx == *x)
                .filter_map(|(coord, pixel)| match pixel {
                    Pixel::Open | Pixel::Wall => Some(coord),
                    _ => None,
                })
                .max_by_key(|(_, y)| y),
            Direction::East => self
                .jungle
                .iter()
                .filter(|((_, yy), _)| *yy == *y)
                .filter_map(|(coord, pixel)| match pixel {
                    Pixel::Open | Pixel::Wall => Some(coord),
                    _ => None,
                })
                .min_by_key(|(x, _)| x),
            Direction::South => self
                .jungle
                .iter()
                .filter(|((xx, _), _)| *xx == *x)
                .filter_map(|(coord, pixel)| match pixel {
                    Pixel::Open | Pixel::Wall => Some(coord),
                    _ => None,
                })
                .min_by_key(|(_, y)| y),
            Direction::West => self
                .jungle
                .iter()
                .filter(|((_, yy), _)| *yy == *y)
                .filter_map(|(coord, pixel)| match pixel {
                    Pixel::Open | Pixel::Wall => Some(coord),
                    _ => None,
                })
                .max_by_key(|(x, _)| x),
        };

        match self.jungle[coord.unwrap()] {
            Pixel::Open => self.position = *coord.unwrap(),
            _ => (),
        }
    }

    fn wrap_cube(&self, (x, y): &Coord) {
        todo!()
    }

    fn walk(&mut self) {
        let (x, y) = self.position;
        let next = match self.facing {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        };

        let next_pixel = self.jungle.get(&next);
        match next_pixel {
            Some(Pixel::Open) => self.position = next,
            Some(Pixel::Wall) => (),
            _ => {
                if self.is_cube {
                    self.wrap_cube(&next);
                } else {
                    self.wrap(&next);
                }
            }
        }
    }

    fn password(&self) -> usize {
        let (column, row) = self.position;
        match self.facing {
            Direction::East => row * 1000 + column * 4 + 0,
            Direction::South => row * 1000 + column * 4 + 1,
            Direction::West => row * 1000 + column * 4 + 2,
            Direction::North => row * 1000 + column * 4 + 3,
        }
    }

    fn cube_size(&self) -> usize {
        f64::sqrt(
            (self
                .jungle
                .iter()
                .filter(|(_, pixel)| match pixel {
                    Pixel::Void => false,
                    _ => true,
                })
                .count()
                / 6) as f64,
        ) as usize
    }
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
    let (input, route) = many1(pair(complete::i64, alt((tag("R"), tag("L")))))(input)?;
    let (input, rest) = complete::i64(input)?;
    let (input, _) = newline(input)?;
    let route = route
        .iter()
        .flat_map(|(steps, direction)| {
            vec![
                Move::Forward(*steps),
                match direction {
                    &"R" => Move::Right,
                    &"L" => Move::Left,
                    _ => panic!("Unknown direction: {}", direction),
                },
            ]
        })
        .chain(vec![Move::Forward(rest)])
        .collect::<Vec<_>>();
    Ok((input, route))
}

fn parse(input: &str) -> IResult<&str, (Vec<Move>, HashMap<Coord, Pixel>)> {
    let (input, jungle) = separated_list1(newline, parse_jungle)(input)?;

    let jungle = jungle
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (y, row)| {
            row.iter().enumerate().for_each(|(x, pixel)| {
                acc.insert((x + 1, y + 1), pixel.clone());
            });
            acc
        });

    let (input, route) = parse_direction(input)?;

    Ok((input, (route, jungle)))
}

fn day22(path: &str, is_cube: bool) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, (route, jungle)) = parse(&content).unwrap();
    let start = jungle
        .iter()
        .filter(|((_, y), _)| *y == 1)
        .filter_map(|(coord, pixel)| match pixel {
            Pixel::Open => Some(coord),
            _ => None,
        })
        .min_by_key(|(x, _)| x)
        .unwrap();

    let mut santa = Person {
        facing: Direction::East,
        position: start.clone(),
        jungle,
        is_cube,
    };

    route.iter().for_each(|direction| match direction {
        Move::Left | Move::Right => santa.turn(direction),
        Move::Forward(steps) => (0..*steps).for_each(|_| santa.walk()),
    });

    santa.password()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_password() {
        let actual = day22("./data/day22.txt", false);
        assert_eq!(actual, 6032);
    }

    #[test]
    fn find_cube_password() {
        let actual = day22("./data/day22.txt", true);
        assert_eq!(actual, 5031);
    }

    #[test]
    fn find_password_part_a() {
        let actual = day22("./data/day22final.txt", false);
        assert_eq!(actual, 31568);
    }
}
