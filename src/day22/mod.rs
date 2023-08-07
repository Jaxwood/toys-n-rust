#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    sequence::pair,
    IResult,
};

type Coord = (usize, usize);

#[derive(Debug, PartialEq)]
struct Coord3D {
    x: usize,
    y: usize,
    z: usize,
    plane: CubeSide,
}

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
    South,
    East,
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
        if let Some(result) = cube.get(&(*x, *y)) {
            let others = cube
                .iter()
                .filter(|(_, d3)| d3.x == result.x && d3.y == result.y && d3.z == result.z)
                .filter(|(coord, _)| **coord != (*x, *y))
                .map(|(x, _)| x)
                .collect::<Vec<_>>();
            if others.len() == 1 {
                match self.jungle.get(others[0]) {
                    Some(Pixel::Open) => {
                        self.position = *others[0];
                        self.facing = get_direction(&self.facing, &result.plane);
                    }
                    _ => (),
                }
            } else {
                let plane = match (&cube.get(&(*x, *y)).unwrap().plane, &self.facing) {
                    (CubeSide::Top(_), Direction::North) => CubeSide::Back(0),
                    (CubeSide::Top(_), Direction::East) => CubeSide::Right(0),
                    (CubeSide::Top(_), Direction::South) => CubeSide::Front(0),
                    (CubeSide::Top(_), Direction::West) => CubeSide::Left(0),
                    (CubeSide::Bottom(_), Direction::North) => CubeSide::Left(0),
                    (CubeSide::Bottom(_), Direction::East) => CubeSide::Front(0),
                    (CubeSide::Bottom(_), Direction::South) => CubeSide::Right(0),
                    (CubeSide::Bottom(_), Direction::West) => CubeSide::Back(0),
                    (CubeSide::Left(_), Direction::North) => CubeSide::Top(0),
                    (CubeSide::Left(_), Direction::East) => CubeSide::Front(0),
                    (CubeSide::Left(_), Direction::South) => CubeSide::Bottom(0),
                    (CubeSide::Left(_), Direction::West) => CubeSide::Back(0),
                    (CubeSide::Right(_), Direction::North) => CubeSide::Bottom(0),
                    (CubeSide::Right(_), Direction::East) => CubeSide::Front(0),
                    (CubeSide::Right(_), Direction::South) => CubeSide::Top(0),
                    (CubeSide::Right(_), Direction::West) => CubeSide::Back(0),
                    (CubeSide::Front(_), Direction::North) => CubeSide::Top(0),
                    (CubeSide::Front(_), Direction::East) => CubeSide::Right(0),
                    (CubeSide::Front(_), Direction::South) => CubeSide::Bottom(0),
                    (CubeSide::Front(_), Direction::West) => CubeSide::Left(0),
                    (CubeSide::Back(_), Direction::North) => CubeSide::Bottom(0),
                    (CubeSide::Back(_), Direction::East) => CubeSide::Right(0),
                    (CubeSide::Back(_), Direction::South) => CubeSide::Top(0),
                    (CubeSide::Back(_), Direction::West) => CubeSide::Left(0),
                };
                let with_plane = cube
                    .iter()
                    .filter(|(_, d3)| {
                        d3.x == result.x
                            && d3.y == result.y
                            && d3.z == result.z
                            && is_same_plane(&plane, &d3.plane)
                    })
                    .filter(|(coord, _)| **coord != (*x, *y))
                    .map(|(x, _)| x)
                    .collect::<Vec<_>>();
                if with_plane.len() != 1 {
                    panic!("No unqiue cube found");
                }
                self.position = *with_plane[0];
                self.facing = get_direction(&self.facing, &result.plane);
            }
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
                        transform.insert(
                            (*x, *y),
                            Coord3D {
                                x: *x,
                                y: *y,
                                z: quadrant_size,
                                plane: side.clone(),
                            },
                        );
                    }
                }
                CubeSide::Front(_) => {
                    for (x, y) in pixels {
                        let new_x = *x;
                        let new_y = max_y;
                        let new_z = quadrant_size - ((y - 1) % quadrant_size);
                        transform.insert(
                            (*x, *y),
                            Coord3D {
                                x: new_x,
                                y: new_y,
                                z: new_z,
                                plane: side.clone(),
                            },
                        );
                    }
                }
                CubeSide::Back(_) => {
                    for (x, y) in pixels {
                        let new_x = *x;
                        let new_y = min_y;
                        let new_z = ((y - 1) % quadrant_size) + 1;
                        transform.insert(
                            (*x, *y),
                            Coord3D {
                                x: new_x,
                                y: new_y,
                                z: new_z,
                                plane: side.clone(),
                            },
                        );
                    }
                }
                CubeSide::Left(_) => {
                    for (x, y) in pixels {
                        let new_x = min_x;
                        let mut new_y = *y;
                        let mut new_z = ((x - 1) % quadrant_size) + 1;
                        if *y > max_y {
                            new_y = min_x + ((*x - 1) % quadrant_size);
                            new_z = quadrant_size - ((*y - 1) % quadrant_size);
                        }
                        transform.insert(
                            (*x, *y),
                            Coord3D {
                                x: new_x,
                                y: new_y,
                                z: new_z,
                                plane: side.clone(),
                            },
                        );
                    }
                }
                CubeSide::Right(_) => {
                    for (x, y) in pixels {
                        let new_x = max_x;
                        let mut new_y = max_y - (x - max_x - 1);
                        let mut new_z = quadrant_size - ((y - 1) % quadrant_size);
                        if *y < min_y {
                            new_y = min_x + ((*x - 1) % quadrant_size);
                            new_z = ((y - 1) % quadrant_size) + 1;
                        }
                        transform.insert(
                            (*x, *y),
                            Coord3D {
                                x: new_x,
                                y: new_y,
                                z: new_z,
                                plane: side.clone(),
                            },
                        );
                    }
                }
                CubeSide::Bottom(_) => {
                    for (x, y) in pixels {
                        let mut new_x = max_x - (*x - 1 % quadrant_size);
                        let mut new_y = *y;
                        let new_z = 1;
                        if *y > max_y {
                            new_x = min_x + ((*y - 1) % quadrant_size);
                            new_y = min_x + ((*x - 1) % quadrant_size);
                        }
                        transform.insert(
                            (*x, *y),
                            Coord3D {
                                x: new_x,
                                y: new_y,
                                z: new_z,
                                plane: side.clone(),
                            },
                        );
                    }
                }
            }
        }

        transform
    }
}

// this will depend on the initial layout of the 2d grid :<
// currently is it targeted the test input
// TODO: make this more generic
fn get_direction(facing: &Direction, plane: &CubeSide) -> Direction {
    match (plane, facing) {
        (CubeSide::Top(_), Direction::North) => Direction::North,
        (CubeSide::Top(_), Direction::South) => Direction::South,
        (CubeSide::Top(_), Direction::East) => Direction::South,
        (CubeSide::Top(_), Direction::West) => Direction::North,
        (CubeSide::Bottom(_), Direction::North) => Direction::North,
        (CubeSide::Bottom(_), Direction::South) => Direction::South,
        (CubeSide::Bottom(_), Direction::East) => Direction::North,
        (CubeSide::Bottom(_), Direction::West) => Direction::South,
        (CubeSide::Left(_), Direction::North) => Direction::East,
        (CubeSide::Left(_), Direction::South) => Direction::South,
        (CubeSide::Left(_), Direction::East) => Direction::East,
        (CubeSide::Left(_), Direction::West) => Direction::East,
        (CubeSide::Right(_), Direction::North) => Direction::North,
        (CubeSide::Right(_), Direction::South) => Direction::West,
        (CubeSide::Right(_), Direction::East) => Direction::West,
        (CubeSide::Right(_), Direction::West) => Direction::West,
        (CubeSide::Front(_), Direction::North) => Direction::North,
        (CubeSide::Front(_), Direction::South) => Direction::West,
        (CubeSide::Front(_), Direction::East) => Direction::West,
        (CubeSide::Front(_), Direction::West) => Direction::West,
        (CubeSide::Back(_), Direction::North) => Direction::East,
        (CubeSide::Back(_), Direction::South) => Direction::South,
        (CubeSide::Back(_), Direction::East) => Direction::East,
        (CubeSide::Back(_), Direction::West) => Direction::East,
    }
}

fn is_same_plane(plane: &CubeSide, other: &CubeSide) -> bool {
    match (plane, other) {
        (CubeSide::Top(_), CubeSide::Top(_)) => true,
        (CubeSide::Bottom(_), CubeSide::Bottom(_)) => true,
        (CubeSide::Left(_), CubeSide::Left(_)) => true,
        (CubeSide::Right(_), CubeSide::Right(_)) => true,
        (CubeSide::Front(_), CubeSide::Front(_)) => true,
        (CubeSide::Back(_), CubeSide::Back(_)) => true,
        _ => false,
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
    #[ignore]
    fn find_cube_password() {
        let actual = day22("./data/day22.txt", true);
        assert_eq!(actual, 5031);
    }

    #[test]
    fn find_password_part_a() {
        let actual = day22("./data/day22final.txt", false);
        assert_eq!(actual, 31568);
    }

    #[test]
    fn find_cube_password_partb() {
        let actual = day22("./data/day22final.txt", true);
        assert_eq!(actual, 36540);
    }
}
