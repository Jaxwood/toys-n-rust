#![allow(dead_code)]

use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    sequence::pair,
    IResult,
};

type Coord = (usize, usize);
type Coord3D = (usize, usize, usize);

struct Person {
    facing: Direction,
    position: Coord,
    jungle: HashMap<Coord, Pixel>,
    is_cube: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CubeSide {
    Top(usize),
    Bottom(usize),
    Left(usize),
    Right(usize),
    Front(usize),
    Back(usize),
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

    fn wrap_cube(&mut self, (x, y): &Coord, cube: &HashMap<Coord, Coord3D>) {
        //if let Some(result) = cube.get(&(11, 12)) {
        if let Some(result) = cube.get(&(12, 6)) {
            let others = cube
                .iter()
                .filter(|(_, (xx, yy, zz))| *xx == result.0 && *yy == result.1 && *zz == result.2)
                .map(|(x,_)| x)
                .collect::<Vec<_>>();
            println!("{:?} {:?}", (x,y), others);
        }
    }

    fn walk(&mut self, cube: &HashMap<Coord, Coord3D>) {
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
                    self.wrap_cube(&(x, y), cube);
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

    fn fold(&self) -> HashMap<Coord, Coord3D> {
        let mut cube = HashMap::new();
        let width = self.jungle.iter().map(|((x, _), _)| x).max().unwrap();
        let height = self.jungle.iter().map(|((_, y), _)| y).max().unwrap();
        let quadrant_size = self.cube_size();
        let quadrant_width = width / quadrant_size;

        let mut quadrants = (0..12).fold(vec![], |mut acc, _| {
            acc.push(vec![]);
            acc
        });

        for y in 0..*height {
            for x in 0..*width {
                let idx = ((x as f64).div_euclid(quadrant_size as f64)
                    + ((y as f64).div_euclid(quadrant_size as f64) * quadrant_width as f64))
                    as usize;
                let coord = (x + 1, y + 1);
                match self.jungle.get(&coord) {
                    Some(Pixel::Void) | None => (),
                    _ => {
                        quadrants[idx].push(coord);
                    }
                }
            }
        }

        // find top
        let sides = (0..6)
            .map(|x| (x, x + quadrant_width, x + quadrant_width + quadrant_width))
            .collect::<Vec<_>>();

        for (x, y, z) in sides {
            if !quadrants[x].is_empty() && !quadrants[y].is_empty() && !quadrants[z].is_empty() {
                cube.insert(CubeSide::Back(x), quadrants[x].clone());
                cube.insert(CubeSide::Top(y), quadrants[y].clone());
                cube.insert(CubeSide::Front(z), quadrants[z].clone());
                // left
                if !quadrants[x - 1].is_empty() {
                    cube.insert(CubeSide::Left(x - 1), quadrants[x - 1].clone());
                } else if !quadrants[y - 1].is_empty() {
                    cube.insert(CubeSide::Left(y - 1), quadrants[y - 1].clone());
                } else if !quadrants[z - 1].is_empty() {
                    cube.insert(CubeSide::Left(z - 1), quadrants[z - 1].clone());
                }
                // right
                if !quadrants[x + 1].is_empty() {
                    cube.insert(CubeSide::Right(x + 1), quadrants[x + 1].clone());
                } else if !quadrants[y + 1].is_empty() {
                    cube.insert(CubeSide::Right(y + 1), quadrants[y + 1].clone());
                } else if !quadrants[z + 1].is_empty() {
                    cube.insert(CubeSide::Right(z + 1), quadrants[z + 1].clone());
                }
            }
        }

        let idxs: HashSet<usize> = cube
            .keys()
            .map(|side| match side {
                CubeSide::Back(x) => *x,
                CubeSide::Top(x) => *x,
                CubeSide::Front(x) => *x,
                CubeSide::Left(x) => *x,
                CubeSide::Right(x) => *x,
                CubeSide::Bottom(x) => *x,
            })
            .collect();
        for bottom in 0..12 {
            if !idxs.contains(&bottom) && !quadrants[bottom].is_empty() {
                cube.insert(CubeSide::Bottom(bottom), quadrants[bottom].clone());
            }
        }

        let top_coords = cube
            .iter()
            .filter_map(|(side, coords)| match side {
                CubeSide::Top(_) => Some(coords.clone()),
                _ => None,
            })
            .flatten()
            .collect::<Vec<Coord>>();

        let mut transform = HashMap::new();
        let (min_x, max_x, min_y, max_y) = boundaries(&top_coords);

        for (side, pixels) in cube.iter() {
            match side {
                CubeSide::Top(_) => {
                    for (x, y) in pixels {
                        transform.insert((*x, *y), (*x, *y, quadrant_size));
                    }
                }
                CubeSide::Front(_) => {
                    for (x, y) in pixels {
                        transform.insert((*x, *y), (*x, max_y, quadrant_size - (*y - max_y - 1)));
                    }
                }
                CubeSide::Back(_) => {
                    for (x, y) in pixels {
                        transform.insert((*x, *y), (*x, min_y, quadrant_size - (min_y - *y - 1)));
                    }
                }
                CubeSide::Left(_) => {
                    for (x, y) in pixels {
                        let mut y_diff = *y;
                        if *y > max_y {
                            y_diff = *y - quadrant_size;
                        } else if *y < min_y {
                            y_diff = *y + quadrant_size;
                        }
                        transform.insert((*x, *y), (min_x, y_diff, quadrant_size - (min_x - *x - 1)));
                    }
                }
                CubeSide::Right(_) => {
                    for (x, y) in pixels {
                        let mut y_diff = *y;
                        if *y > max_y {
                            y_diff = *y - quadrant_size;
                        } else if *y < min_y {
                            y_diff = *y + quadrant_size;
                        }
                        transform.insert((*x, *y), (max_x, y_diff, quadrant_size - (*x - max_x - 1)));
                    }
                }
                CubeSide::Bottom(_) => {
                    let mut idx = 0;
                    for (x, y) in pixels {
                        idx = idx % quadrant_size;
                        let mut y_diff = *y;
                        if *y > max_y {
                            y_diff = *y - quadrant_size - quadrant_size;
                        } else if *y < min_y {
                            y_diff = *y + quadrant_size + quadrant_size;
                        }
                        transform.insert((*x, *y), (min_x + idx, y_diff, 1));
                        idx += 1;
                    }
                }
            }
        }

        for (_,(x,y,z)) in transform.iter() {
            println!("{},{},{}", x, y, z);
        }

        transform
    }
}

fn boundaries(coords: &Vec<Coord>) -> (usize, usize, usize, usize) {
    let min_x = coords.iter().map(|(x, _)| x).min().unwrap().clone();
    let max_x = coords.iter().map(|(x, _)| x).max().unwrap().clone();
    let min_y = coords.iter().map(|(_, y)| y).min().unwrap().clone();
    let max_y = coords.iter().map(|(_, y)| y).max().unwrap().clone();
    (min_x, max_x, min_y, max_y)
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

    let cube = santa.fold();

    route.iter().for_each(|direction| match direction {
        Move::Left | Move::Right => santa.turn(direction),
        Move::Forward(steps) => (0..*steps).for_each(|_| santa.walk(&cube)),
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
    fn find_next_move() {
        let actual = day22("./data/day22.txt", true);
        assert_eq!(actual, 5031);
    }

    #[test]
    fn find_password_part_a() {
        let actual = day22("./data/day22final.txt", false);
        assert_eq!(actual, 31568);
    }
}
