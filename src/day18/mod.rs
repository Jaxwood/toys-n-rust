#![allow(dead_code)]
use std::io::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    fs::{self, File},
};

use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1, IResult};

type Cube = (i32, i32, i32);

fn parse_kube(input: &str) -> IResult<&str, Cube> {
    let (input, x) = separated_list1(tag(","), nom::character::complete::i32)(input)?;
    Ok((input, (x[0], x[1], x[2])))
}

fn parse(input: &str) -> IResult<&str, HashSet<Cube>> {
    let (input, cubes) = separated_list1(newline, parse_kube)(input)?;
    Ok((input, cubes.into_iter().collect::<HashSet<_>>()))
}

fn neighbours(&(x, y, z): &Cube) -> Vec<Cube> {
    vec![
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn surface_area(cubes: &HashSet<Cube>) -> usize {
    cubes
        .iter()
        .map(|k| {
            neighbours(k)
                .iter()
                .filter(|&p| cubes.get(p).is_none())
                .count()
        })
        .sum::<usize>()
}

fn universe(cubes: &HashSet<Cube>) -> (Cube, Cube) {
    let x_min = cubes.iter().min_by_key(|(x, _, _)| x).unwrap().0;
    let x_max = cubes.iter().max_by_key(|(x, _, _)| x).unwrap().0;
    let y_min = cubes.iter().min_by_key(|(_, y, _)| y).unwrap().1;
    let y_max = cubes.iter().max_by_key(|(_, y, _)| y).unwrap().1;
    let z_min = cubes.iter().min_by_key(|(_, _, z)| z).unwrap().2;
    let z_max = cubes.iter().max_by_key(|(_, _, z)| z).unwrap().2;

    (
        (x_min - 1, y_min - 1, z_min - 1),
        (x_max + 1, y_max + 1, z_max + 1),
    )
}

fn day18a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, cubes) = parse(content.as_str()).expect("parsing failed");
    surface_area(&cubes)
}

fn day18b(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, cubes) = parse(content.as_str()).expect("parsing failed");
    let ((x_min, y_min, z_min), (x_max, y_max, z_max)) = universe(&cubes);
    let mut air = HashSet::new();

    // steam outside of lava droplet
    let mut queue = VecDeque::from([(x_min, y_min, z_min)]);
    let mut steam = HashSet::new();
    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();

        for (x, y, z) in neighbours(&next) {
            let n = (x, y, z);
            if x < x_min || x > x_max || y < y_min || y > y_max || z < z_min || z > z_max {
                continue;
            }
            if steam.contains(&n) {
                continue;
            }
            if cubes.get(&n).is_none() {
                steam.insert(n);
                queue.push_back(n);
            }
        }
    }

    // find air bubbles in the lava droplet
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                let cube = (x, y, z);
                if cubes.get(&cube).is_none() && steam.get(&cube).is_none() {
                    air.insert(cube);
                }
            }
        }
    }

    // find cube surfaces that are next to air bubbles
    let result = cubes
        .iter()
        .map(|k| {
            neighbours(k)
                .iter()
                .filter(|&p| air.get(p).is_some())
                .count()
        })
        .sum::<usize>();

    surface_area(&cubes) - result
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

    #[test]
    fn find_surface_area_without_air_bubbles_part_b() {
        let actual = day18b("./data/day18final.txt");
        assert_eq!(actual, 2558);
    }
}
