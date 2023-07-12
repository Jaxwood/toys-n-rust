#![allow(dead_code)]
use std::{collections::HashSet, fs};

use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1, IResult};

type Kube = (i32, i32, i32);

fn parse_kube(input: &str) -> IResult<&str, Kube> {
    let (input, x) = separated_list1(tag(","), nom::character::complete::i32)(input)?;
    Ok((input, (x[0], x[1], x[2])))
}

fn parse(input: &str) -> IResult<&str, Vec<Kube>> {
    let (input, kubes) = separated_list1(newline, parse_kube)(input)?;
    Ok((input, kubes))
}

fn sides(kube: &Kube) -> Vec<HashSet<Kube>> {
    let &(x, y, z) = kube;

    vec![
        HashSet::from([(x, y, z), (x - 1, y, z), (x, y, z - 1), (x - 1, y, z - 1)]),
        HashSet::from([(x, y, z), (x, y - 1, z), (x, y - 1, z - 1), (x, y, z - 1)]),
        HashSet::from([
            (x, y - 1, z),
            (x - 1, y - 1, z),
            (x, y - 1, z - 1),
            (x - 1, y - 1, z - 1),
        ]),
        HashSet::from([
            (x - 1, y - 1, z),
            (x - 1, y - 1, z - 1),
            (x - 1, y, z - 1),
            (x - 1, y, z),
        ]),
        HashSet::from([(x, y, z), (x, y - 1, z), (x - 1, y, z), (x - 1, y - 1, z)]),
        HashSet::from([
            (x, y, z - 1),
            (x, y - 1, z - 1),
            (x - 1, y - 1, z - 1),
            (x - 1, y, z - 1),
        ]),
    ]
}

fn exposed_sides(kube: &Vec<HashSet<Kube>>, other: &Vec<HashSet<Kube>>) -> usize {
    kube.iter().filter(|&side| other.contains(side)).count()
}

fn day18a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, kubes) = parse(content.as_str()).expect("parsing failed");
    let kube_sides: Vec<_> = kubes.iter().map(|kube| sides(kube)).collect();

    kubes
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            6 - kubes
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != idx)
                .map(|(i, _)| exposed_sides(&kube_sides[idx], &kube_sides[i]))
                .sum::<usize>()
        })
        .sum()
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
    #[ignore = "slow test"]
    fn find_surface_area_part_a() {
        let actual = day18a("./data/day18final.txt");
        assert_eq!(actual, 4500);
    }

    #[test]
    fn find_exposed_sides() {
        let actual = (1, 1, 1);
        let other = (1, 1, 2);
        assert_eq!(1, exposed_sides(&sides(&actual), &sides(&other)));
    }
}
