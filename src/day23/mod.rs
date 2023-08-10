#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

type Coord = (i32, i32);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pixel {
    Elf(VecDeque<Direction>),
    Open,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Move {
    from: Coord,
    to: Coord,
}

struct State {
    map: HashMap<Coord, Pixel>,
}

impl State {
    fn new(map: HashMap<Coord, Pixel>) -> Self {
        Self { map }
    }

    fn is_open(&self, coord: &Coord) -> bool {
        match self.map.get(coord) {
            Some(Pixel::Elf(_)) => false,
            _ => true,
        }
    }

    fn north_open(&self, coord: &Coord) -> bool {
        self.is_open(&(coord.0, coord.1 - 1))
            && self.is_open(&(coord.0 - 1, coord.1 - 1))
            && self.is_open(&(coord.0 + 1, coord.1 - 1))
    }

    fn south_open(&self, coord: &Coord) -> bool {
        self.is_open(&(coord.0, coord.1 + 1))
            && self.is_open(&(coord.0 - 1, coord.1 + 1))
            && self.is_open(&(coord.0 + 1, coord.1 + 1))
    }

    fn west_open(&self, coord: &Coord) -> bool {
        self.is_open(&(coord.0 - 1, coord.1))
            && self.is_open(&(coord.0 - 1, coord.1 - 1))
            && self.is_open(&(coord.0 - 1, coord.1 + 1))
    }

    fn east_open(&self, coord: &Coord) -> bool {
        self.is_open(&(coord.0 + 1, coord.1))
            && self.is_open(&(coord.0 + 1, coord.1 - 1))
            && self.is_open(&(coord.0 + 1, coord.1 + 1))
    }

    fn consider(&self) -> Vec<Move> {
        self.map
            .iter()
            .filter_map(|((x, y), pixel)| match pixel {
                Pixel::Open => None,
                Pixel::Elf(dir) => {
                    let result = dir
                        .iter()
                        .filter_map(|d| match d {
                            Direction::North => {
                                if self.north_open(&(*x, *y)) {
                                    Some(Move {
                                        from: (*x, *y),
                                        to: (*x, *y - 1),
                                    })
                                } else {
                                    None
                                }
                            }
                            Direction::East => {
                                if self.east_open(&(*x, *y)) {
                                    Some(Move {
                                        from: (*x, *y),
                                        to: (*x + 1, *y),
                                    })
                                } else {
                                    None
                                }
                            }
                            Direction::South => {
                                if self.south_open(&(*x, *y)) {
                                    Some(Move {
                                        from: (*x, *y),
                                        to: (*x, *y + 1),
                                    })
                                } else {
                                    None
                                }
                            }
                            Direction::West => {
                                if self.west_open(&(*x, *y)) {
                                    Some(Move {
                                        from: (*x, *y),
                                        to: (*x - 1, *y),
                                    })
                                } else {
                                    None
                                }
                            }
                        })
                        .collect::<Vec<_>>();

                    if result.len() == 4 || result.is_empty() {
                        return None;
                    } else {
                        return result.first().cloned();
                    }
                }
            })
            .collect::<Vec<_>>()
    }

    fn is_elf(&self, coord: &Coord) -> bool {
        match self.map.get(coord) {
            Some(Pixel::Elf(_)) => true,
            _ => false,
        }
    }

    fn try_move(&mut self, moves: Vec<Move>) {
        let duplicates = moves
            .iter()
            .filter(|m| moves.iter().filter(|n| n.to == m.to).count() > 1)
            .map(|m| m.to)
            .collect::<HashSet<_>>();

        let valid_moves = moves
            .iter()
            .filter(|m| !duplicates.contains(&m.to))
            .collect::<Vec<_>>();

        for Move { from, to } in valid_moves {
            match self.map.get(from) {
                Some(Pixel::Elf(dir)) => {
                    self.map.insert(*to, Pixel::Elf(dir.clone()));
                    self.map.insert(*from, Pixel::Open);
                }
                Some(Pixel::Open) => panic!("Invalid move"),
                None => panic!("Invalid move"),
            }
        }
    }

    fn update(&mut self) {
        for kv in self.map.iter_mut() {
            match kv.1 {
                Pixel::Elf(ref mut directions) => {
                    directions.rotate_left(1);
                }
                Pixel::Open => (),
            }
        }
    }

    fn empty_ground(&self) -> usize {
        let elves = self
            .map
            .iter()
            .filter(|(c, _)| self.is_elf(c))
            .collect::<Vec<_>>();
        let ((col_min, _), _) = elves.iter().min_by_key(|(&(x, _), _)| x).unwrap();
        let ((col_max, _), _) = elves.iter().max_by_key(|(&(x, _), _)| x).unwrap();
        let ((_, row_min), _) = elves.iter().min_by_key(|(&(_, y), _)| y).unwrap();
        let ((_, row_max), _) = elves.iter().max_by_key(|(&(_, y), _)| y).unwrap();

        let mut result = 0;
        for y in *row_min..=*row_max {
            for x in *col_min..=*col_max {
                result += match self.map.get(&(x, y)) {
                    Some(Pixel::Elf(_)) => 0,
                    _ => 1,
                };
            }
        }
        result
    }
}

fn parse_map(input: &str) -> IResult<&str, Vec<Pixel>> {
    let (input, pixels) = many1(alt((
        map(tag("#"), |_| {
            Pixel::Elf(VecDeque::from([
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ]))
        }),
        map(tag("."), |_| Pixel::Open),
    )))(input)?;

    Ok((input, pixels))
}

fn parse(input: &str) -> IResult<&str, HashMap<Coord, Pixel>> {
    let (input, map) = separated_list1(newline, parse_map)(input)?;

    let mut result = HashMap::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            result.insert((col as i32, row as i32), map[row][col].clone());
        }
    }

    Ok((input, result))
}

fn day23a(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let (_, map) = parse(&input).unwrap();
    let mut state = State::new(map.clone());
    for _ in 0..10 {
        let moves = state.consider();
        state.try_move(moves);
        state.update();
    }

    state.empty_ground()
}

fn day23b(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let (_, map) = parse(&input).unwrap();
    let mut state = State::new(map.clone());
    let mut round = 1;
    loop {
        let moves = state.consider();
        if moves.is_empty() {
            return round
        }
        state.try_move(moves);
        state.update();
        round += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_area() {
        let actual = day23a("./data/day23.txt");
        assert_eq!(actual, 110);
    }

    #[test]
    fn find_no_moves() {
        let actual = day23b("./data/day23.txt");
        assert_eq!(actual, 20);
    }

    #[test]
    fn find_area_part_a() {
        let actual = day23a("./data/day23final.txt");
        assert_eq!(actual, 4158);
    }

    #[test]
    fn find_area_part_b() {
        let actual = day23b("./data/day23final.txt");
        assert_eq!(actual, 1014);
    }
}
