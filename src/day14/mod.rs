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

fn drip(coord: (u32, u32), map: &mut HashMap<(u32, u32), Pixel>) -> bool {
    let coord = dripping(coord, map);
    match coord {
        Some((x, y)) => {
            map.insert((x, y), Pixel::Sand);
            true
        }
        None => false,
    }
}

fn is_out_of_bounds((_,y): (u32, u32), map: &mut HashMap<(u32, u32), Pixel>) -> bool {
    match map.keys().map(|(_, y)| y).max() {
        Some(max_y) => y > *max_y,
        None => false,
    }
}

fn dripping((x, y): (u32, u32), map: &mut HashMap<(u32, u32), Pixel>) -> Option<(u32, u32)> {
    if is_out_of_bounds((x,y), map) {
        return None;
    }

    let below = map.get(&(x, y + 1));
    let left = map.get(&(x - 1, y + 1));
    let right = map.get(&(x + 1, y + 1));

    match (left, below, right) {
        (None, None, None) => dripping((x, y + 1), map),
        (None, None, Some(_)) => dripping((x, y + 1), map),
        (None, Some(_), None) => dripping((x - 1, y + 1), map),
        (Some(_), None, None) => dripping((x, y + 1), map),
        (None, Some(_), Some(_)) => dripping((x - 1, y + 1), map),
        (Some(_), None, Some(_)) => dripping((x, y + 1), map),
        (Some(_), Some(_), None) => dripping((x + 1, y + 1), map),
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

    while drip((500, 0), &mut map) {}

    map.values().filter(|pixel| pixel.is_sand()).count()
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
    fn find_amount_of_rested_sand_part_a() {
        let actual = day14a("./data/day14final.txt");
        assert_eq!(actual, 799);
    }
}

