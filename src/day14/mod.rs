#![allow(dead_code)]

use std::{cmp, collections::HashMap, fmt::Display, fs};

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

enum Pixel {
    Rock,
    Air,
    Sand,
}

impl Pixel {
    fn is_sand(&self) -> bool {
        match self {
            Pixel::Sand => true,
            _ => false,
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::Rock => write!(f, "#"),
            Pixel::Air => write!(f, "."),
            Pixel::Sand => write!(f, "o"),
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            separated_pair(
                nom::character::complete::u32,
                tag(","),
                nom::character::complete::u32,
            ),
        ),
    )(input)
}

fn print(map: &HashMap<(u32, u32), Pixel>) {
    for y in 0..12 {
        for x in 490..510 {
            match map.get(&(x, y)) {
                Some(pixel) => print!("{}", pixel),
                None => print!("{}", Pixel::Air),
            }
        }
        println!()
    }
}

fn drip(
    coord: (u32, u32),
    map: &mut HashMap<(u32, u32), Pixel>,
    max_depth: u32,
    partb: bool,
) -> bool {
    let coord = dripping(coord, map, max_depth, partb);
    if coord == Some((500, 0)) {
        return false;
    }
    match coord {
        Some((x, y)) => {
            map.insert((x, y), Pixel::Sand);
            true
        }
        None => false,
    }
}

fn is_out_of_bounds((_, y): (u32, u32), max_depth: u32) -> bool {
    y > max_depth
}

fn dripping(
    (x, y): (u32, u32),
    map: &mut HashMap<(u32, u32), Pixel>,
    max_depth: u32,
    partb: bool,
) -> Option<(u32, u32)> {
    if is_out_of_bounds((x, y), max_depth) {
        return None;
    }

    let below = map.get(&(x, y + 1)).or_else(|| match partb {
        true => {
            if y + 1 == max_depth {
                Some(&Pixel::Rock)
            } else {
                None
            }
        }
        false => None,
    });
    let left = map.get(&(x - 1, y + 1)).or_else(|| match partb {
        true => {
            if y + 1 == max_depth {
                Some(&Pixel::Rock)
            } else {
                None
            }
        }
        false => None,
    });
    let right = map.get(&(x + 1, y + 1)).or_else(|| match partb {
        true => {
            if y + 1 == max_depth {
                Some(&Pixel::Rock)
            } else {
                None
            }
        }
        false => None,
    });

    match (left, below, right) {
        (None, None, None) => dripping((x, y + 1), map, max_depth, partb),
        (None, None, Some(_)) => dripping((x, y + 1), map, max_depth, partb),
        (None, Some(_), None) => dripping((x - 1, y + 1), map, max_depth, partb),
        (Some(_), None, None) => dripping((x, y + 1), map, max_depth, partb),
        (None, Some(_), Some(_)) => dripping((x - 1, y + 1), map, max_depth, partb),
        (Some(_), None, Some(_)) => dripping((x, y + 1), map, max_depth, partb),
        (Some(_), Some(_), None) => dripping((x + 1, y + 1), map, max_depth, partb),
        (Some(_), Some(_), Some(_)) => Some((x, y)),
    }
}

fn day14a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, coords) = parse(content.as_str()).expect("parsing failed");

    let mut map = coords
        .iter()
        .map(|coord| {
            coord.iter().zip(coord.iter().skip(1)).fold(
                HashMap::new(),
                |mut acc, ((x, y), (xx, yy))| {
                    for y in cmp::min(*y, *yy)..=cmp::max(*y, *yy) {
                        for x in cmp::min(*x, *xx)..=cmp::max(*x, *xx) {
                            acc.insert((x, y), Pixel::Rock);
                        }
                    }
                    acc
                },
            )
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();
    while drip((500, 0), &mut map, max_y, false) {}

    map.values().filter(|pixel| pixel.is_sand()).count()
}

fn day14b(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, coords) = parse(content.as_str()).expect("parsing failed");

    let mut map = coords
        .iter()
        .map(|coord| {
            coord.iter().zip(coord.iter().skip(1)).fold(
                HashMap::new(),
                |mut acc, ((x, y), (xx, yy))| {
                    for y in cmp::min(*y, *yy)..=cmp::max(*y, *yy) {
                        for x in cmp::min(*x, *xx)..=cmp::max(*x, *xx) {
                            acc.insert((x, y), Pixel::Rock);
                        }
                    }
                    acc
                },
            )
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    let y_max = *map.keys().map(|(_, y)| y).max().unwrap();
    while drip((500, 0), &mut map, y_max + 2, true) {}

    map.values().filter(|pixel| pixel.is_sand()).count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_amount_of_rested_sand() {
        let actual = day14a("./data/day14.txt");
        assert_eq!(actual, 24);
    }

    #[test]
    fn find_amount_of_rested_sand_with_bottom() {
        let actual = day14b("./data/day14.txt");
        assert_eq!(actual, 93);
    }

    #[test]
    fn find_amount_of_rested_sand_part_a() {
        let actual = day14a("./data/day14final.txt");
        assert_eq!(actual, 799);
    }

    #[test]
    fn find_amount_of_rested_sand_part_b() {
        let actual = day14b("./data/day14final.txt");
        assert_eq!(actual, 29076);
    }
}
