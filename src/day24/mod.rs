#![allow(dead_code)]

use std::{
    collections::{HashSet, VecDeque},
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    storm: Vec<Blizzard>,
    position: Coord,
    height: i32,
    width: i32,
    start: Coord,
    end: Coord,
    minutes: i32,
}

impl From<&Blizzard> for Coord {
    fn from(value: &Blizzard) -> Self {
        match value {
            Blizzard::Up(coord) => *coord,
            Blizzard::Down(coord) => *coord,
            Blizzard::Left(coord) => *coord,
            Blizzard::Right(coord) => *coord,
        }
    }
}

type Coord = (i32, i32);

enum Pixel {
    Storm(Blizzard),
    Rock,
    Open,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Blizzard {
    Up(Coord),
    Down(Coord),
    Left(Coord),
    Right(Coord),
}

fn parse_map(input: &str) -> IResult<&str, Vec<Pixel>> {
    let (input, map) = many1(alt((
        map(tag("."), |_| Pixel::Open),
        map(tag("#"), |_| Pixel::Rock),
        map(tag("^"), |_| Pixel::Storm(Blizzard::Up((0, 0)))),
        map(tag("v"), |_| Pixel::Storm(Blizzard::Down((0, 0)))),
        map(tag("<"), |_| Pixel::Storm(Blizzard::Left((0, 0)))),
        map(tag(">"), |_| Pixel::Storm(Blizzard::Right((0, 0)))),
    )))(input)?;

    Ok((input, map))
}

fn parse(input: &str) -> IResult<&str, (i32, i32, Vec<Blizzard>)> {
    let (input, mut map) = separated_list1(newline, parse_map)(input)?;

    let width = map[0].len() - 1;
    let height = map.len() - 1;

    let is_blizzard = |pixel: &Pixel| match pixel {
        Pixel::Storm(_) => true,
        _ => false,
    };

    let result = map
        .iter_mut()
        .enumerate()
        .flat_map(|(idx, row)| {
            row.iter_mut()
                .enumerate()
                .filter(|k| is_blizzard(k.1))
                .map(|(idx2, pixel)| match pixel {
                    Pixel::Storm(Blizzard::Up(_)) => Blizzard::Up((idx2 as i32, idx as i32)),
                    Pixel::Storm(Blizzard::Down(_)) => Blizzard::Down((idx2 as i32, idx as i32)),
                    Pixel::Storm(Blizzard::Left(_)) => Blizzard::Left((idx2 as i32, idx as i32)),
                    Pixel::Storm(Blizzard::Right(_)) => Blizzard::Right((idx2 as i32, idx as i32)),
                    _ => panic!("Not a blizzard!"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok((input, (width as i32, height as i32, result)))
}

impl State {
    fn tick(&mut self) {
        self.minutes += 1;
        self.storm.iter_mut().for_each(|blizzard| match blizzard {
            Blizzard::Up((column, row)) => {
                if *row == 1 {
                    *blizzard = Blizzard::Up((*column, self.height - 1));
                } else {
                    *blizzard = Blizzard::Up((*column, *row - 1));
                }
            }
            Blizzard::Down((column, row)) => {
                if *row == self.height - 1 {
                    *blizzard = Blizzard::Down((*column, 1));
                } else {
                    *blizzard = Blizzard::Down((*column, *row + 1));
                }
            }
            Blizzard::Left((column, row)) => {
                if *column == 1 {
                    *blizzard = Blizzard::Left((self.width - 1, *row));
                } else {
                    *blizzard = Blizzard::Left((*column - 1, *row));
                }
            }
            Blizzard::Right((column, row)) => {
                if *column == self.width - 1 {
                    *blizzard = Blizzard::Right((1, *row));
                } else {
                    *blizzard = Blizzard::Right((*column + 1, *row));
                }
            }
        });
    }

    fn hash(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        for row in 1..self.height {
            for column in 1..self.width {
                let pixels = self
                    .storm
                    .iter()
                    .filter(|&blizzard| Coord::from(blizzard) == (column, row))
                    .collect::<Vec<_>>();
                if (column, row) == self.position {
                    result.push(String::from("E"));
                } else if pixels.is_empty() {
                    result.push(String::from("."));
                } else if pixels.len() > 1 {
                    result.push(pixels.len().to_string());
                } else {
                    match pixels.first() {
                        Some(blizzard) => match blizzard {
                            Blizzard::Up(_) => result.push(String::from("^")),
                            Blizzard::Down(_) => result.push(String::from("v")),
                            Blizzard::Left(_) => result.push(String::from("<")),
                            Blizzard::Right(_) => result.push(String::from(">")),
                        },
                        None => panic!("No blizzard!"),
                    }
                }
            }
        }
        result.join("")
    }

    fn find_open(&self) -> HashSet<Coord> {
        let blizzard = self
            .storm
            .iter()
            .map(|bliz| Coord::from(bliz))
            .collect::<HashSet<_>>();

        let all = (1..self.height)
            .flat_map(|row| (1..self.width).map(move |column| (column, row)))
            .chain(vec![self.end].into_iter())
            .collect::<HashSet<_>>();

        all.difference(&blizzard).cloned().collect::<HashSet<_>>()
    }

    fn done(&self) -> bool {
        self.position == self.end
    }

    fn add(&self, (x, y): Coord) -> Coord {
        (self.position.0 + x, self.position.1 + y)
    }

    fn in_range(&self, (x, y): Coord) -> bool {
        (x > 0 && y > 0 && x < self.width && y < self.height) || (self.end == (x, y))
    }
}

fn forecast(state: &State) {
    let hash = state.hash();

    println!("{}:", state.minutes);
    for row in 0..state.height - 1 {
        for column in 0..state.width - 1 {
            let ch = hash
                .chars()
                .nth((row * (state.width - 1) + column) as usize);
            match ch {
                Some(c) => print!("{}", c),
                None => panic!("No char!"),
            }
        }
        println!();
    }
}

fn day24a(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let (_, (width, height, storm)) = parse(&input).unwrap();

    let state = State {
        storm: storm.clone(),
        position: (1, 0),
        width,
        height,
        start: (1, 0),
        end: (width - 1, height),
        minutes: 0,
    };

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(state.clone());

    while !queue.is_empty() {
        let mut next = queue.pop_front().unwrap();
        let hash = &next.hash();
        if seen.contains(hash) {
            continue;
        }
        seen.insert(hash.to_string());
        next.tick();

        let open = next.find_open();
        let moves = vec![(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)]
            .iter()
            .map(|&coord| next.add(coord))
            .filter(|&coord| next.in_range(coord))
            .collect::<Vec<_>>();

        for coord in moves {
            if open.contains(&coord) {
                let mut new_state = next.clone();
                new_state.position = coord;

                if new_state.done() {
                    return new_state.minutes as usize;
                }

                queue.push_back(new_state);
            }
        }
    }

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

    #[test]
    fn find_shortest_path_part_a() {
        let actual = day24a("./data/day24final.txt");
        assert_eq!(actual, 0);
    }
}
