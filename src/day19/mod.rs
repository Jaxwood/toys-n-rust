#![allow(dead_code)]

use std::fs;

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1, sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum Robot {
    Ore(i32),
    Clay(i32),
    Obsidian(i32, i32),
    Geode(i32, i32),
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore: Robot,
    clay: Robot,
    obsidian: Robot,
    geode: Robot,
}

fn parse_id(input: &str) -> IResult<&str, i32> {
    let (input, (_, id, _)) =
        tuple((tag("Blueprint "), nom::character::complete::i32, tag(": ")))(input)?;
    Ok((input, id))
}

fn parse_ore(input: &str) -> IResult<&str, Robot> {
    let (input, (_, ore, _)) = tuple((
        tag("Each ore robot costs "),
        nom::character::complete::i32,
        tag(" ore. "),
    ))(input)?;
    Ok((input, Robot::Ore(ore)))
}

fn parse_clay(input: &str) -> IResult<&str, Robot> {
    let (input, (_, clay, _)) = tuple((
        tag("Each clay robot costs "),
        nom::character::complete::i32,
        tag(" ore. "),
    ))(input)?;
    Ok((input, Robot::Clay(clay)))
}

fn parse_obsidian(input: &str) -> IResult<&str, Robot> {
    let (input, (_, ore, _, clay, _)) = tuple((
        tag("Each obsidian robot costs "),
        nom::character::complete::i32,
        tag(" ore and "),
        nom::character::complete::i32,
        tag(" clay. "),
    ))(input)?;
    Ok((input, Robot::Obsidian(ore, clay)))
}

fn parse_geode(input: &str) -> IResult<&str, Robot> {
    let (input, (_, ore, _, obsidian, _)) = tuple((
        tag("Each geode robot costs "),
        nom::character::complete::i32,
        tag(" ore and "),
        nom::character::complete::i32,
        tag(" obsidian."),
    ))(input)?;
    Ok((input, Robot::Geode(ore, obsidian)))
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = parse_id(input)?;
    let (input, ore) = parse_ore(input)?;
    let (input, clay) = parse_clay(input)?;
    let (input, obsidian) = parse_obsidian(input)?;
    let (input, geode) = parse_geode(input)?;

    Ok((
        input,
        Blueprint {
            id,
            ore,
            clay,
            obsidian,
            geode,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Blueprint>> {
    let (input, blueprints) = separated_list1(newline, parse_blueprint)(input)?;
    Ok((input, blueprints))
}

fn day19a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, blueprints) = parse(&content).unwrap();
    for blueprint in blueprints {
        println!("{:?}", blueprint);
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_quality_level() {
        let actual = day19a("./data/day19.txt");
        assert_eq!(actual, 33);
    }
}
