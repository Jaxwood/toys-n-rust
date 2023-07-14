#![allow(dead_code)]
use std::{collections::HashSet, fs};

use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1, IResult};

type Cube = (i32, i32, i32);

fn parse_kube(input: &str) -> IResult<&str, Cube> {
    let (input, x) = separated_list1(tag(","), nom::character::complete::i32)(input)?;
    Ok((input, (x[0], x[1], x[2])))
}

fn parse(input: &str) -> IResult<&str, HashSet<Cube>> {
    let (input, kubes) = separated_list1(newline, parse_kube)(input)?;
    Ok((input, kubes.into_iter().collect::<HashSet<_>>()))
}

fn points(&(x,y,z): &Cube) -> Vec<Cube> {

    vec![
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn day18a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, kubes) = parse(content.as_str()).expect("parsing failed");

    kubes.iter()
         .map(|k| {
             points(k)
                 .iter()
                 .filter(|&p| kubes.get(p).is_none())
                 .count()
         })
    .sum::<usize>()
}

fn day18b(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, kubes) = parse(content.as_str()).expect("parsing failed");

    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_surface_area() {
        let actual = day18a("./data/day18.txt");
        assert_eq!(actual, 64);
    }

    #[test]
    fn find_surface_area_part_a() {
        let actual = day18a("./data/day18final.txt");
        assert_eq!(actual, 4500);
    }

    #[test]
    fn find_surface_area_without_air_bubbles() {
        let actual = day18b("./data/day18.txt");
        assert_eq!(actual, 58);
    }
}
