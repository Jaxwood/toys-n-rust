#![allow(dead_code)]

use rayon::prelude::*;
use std::{cmp, collections::{HashSet, HashMap}, fs, iter::once};

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1, sequence::tuple,
    IResult,
};

#[derive(Debug, Clone)]
struct State {
    minute: i32,
    ore_robots: Vec<Robot>,
    clay_robots: Vec<Robot>,
    obsidian_robots: Vec<Robot>,
    geode_robots: Vec<Robot>,
    ores: i32,
    clays: i32,
    obsidians: i32,
    geodes: i32,
}

impl ToString for State {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.minute,
            self.ores,
            self.clays,
            self.obsidians,
            self.geodes,
            self.ore_robots.len(),
            self.clay_robots.len(),
            self.obsidian_robots.len(),
            self.geode_robots.len(),
        )
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            minute: 0,
            ore_robots: vec![Robot::Ore],
            clay_robots: vec![],
            obsidian_robots: vec![],
            geode_robots: vec![],
            ores: 0,
            clays: 0,
            obsidians: 0,
            geodes: 0,
        }
    }
}

impl State {
    fn is_done(&self, end: i32) -> bool {
        return self.minute == end;
    }

    fn harvest(&mut self) {
        self.minute += 1;

        self.ores += self.ore_robots.len() as i32;
        self.clays += self.clay_robots.len() as i32;
        self.obsidians += self.obsidian_robots.len() as i32;
        self.geodes += self.geode_robots.len() as i32;
    }

    fn try_buy(&self, blueprint: &Blueprint) -> Vec<Option<Robot>> {
        blueprint
            .costs
            .iter()
            .filter_map(|cost| match cost {
                Cost::Ore(ore) => {
                    if self.ores >= *ore && self.ore_robots.len() < 5 {
                        Some(Robot::Ore)
                    } else {
                        None
                    }
                }
                Cost::Clay(ore) => {
                    if self.ores >= *ore && self.clay_robots.len() < 10 {
                        Some(Robot::Clay)
                    } else {
                        None
                    }
                }
                Cost::Obsidian(ore, clay) => {
                    if self.ores >= *ore && self.clays >= *clay {
                        Some(Robot::Obsidian)
                    } else {
                        None
                    }
                }
                Cost::Geode(ore, obsidian) => {
                    if self.ores >= *ore && self.obsidians >= *obsidian {
                        Some(Robot::Geode)
                    } else {
                        None
                    }
                }
            })
            .map(|robot| Some(robot))
            .chain(once(None))
            .collect::<Vec<_>>()
    }

    fn ready(&mut self, robot: &Robot, blueprint: &Blueprint) {
        match robot {
            Robot::Ore => {
                blueprint
                    .costs
                    .iter()
                    .filter_map(|cost| match cost {
                        Cost::Ore(ore) => Some(ore),
                        _ => None,
                    })
                    .for_each(|ore| self.ores -= ore);
                    self.ore_robots.push(robot.clone());
            }
            Robot::Clay => {
                blueprint
                    .costs
                    .iter()
                    .filter_map(|cost| match cost {
                        Cost::Clay(ore) => Some(ore),
                        _ => None,
                    })
                    .for_each(|ore| self.ores -= ore);
                    self.clay_robots.push(robot.clone());
            }
            Robot::Obsidian => {
                blueprint
                    .costs
                    .iter()
                    .filter_map(|cost| match cost {
                        Cost::Obsidian(ore, clay) => Some((ore, clay)),
                        _ => None,
                    })
                    .for_each(|(ore, clay)| {
                        self.ores -= ore;
                        self.clays -= clay;
                    });
                    self.obsidian_robots.push(robot.clone());
            }
            Robot::Geode => {
                blueprint
                    .costs
                    .iter()
                    .filter_map(|cost| match cost {
                        Cost::Geode(ore, obsidian) => Some((ore, obsidian)),
                        _ => None,
                    })
                    .for_each(|(ore, obsidian)| {
                        self.ores -= ore;
                        self.obsidians -= obsidian;
                    });
                    self.geode_robots.push(robot.clone());
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
enum Cost {
    Ore(i32),
    Clay(i32),
    Obsidian(i32, i32),
    Geode(i32, i32),
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    costs: Vec<Cost>,
}

fn parse_id(input: &str) -> IResult<&str, i32> {
    let (input, (_, id, _)) =
        tuple((tag("Blueprint "), nom::character::complete::i32, tag(": ")))(input)?;
    Ok((input, id))
}

fn parse_ore(input: &str) -> IResult<&str, Cost> {
    let (input, (_, ore, _)) = tuple((
        tag("Each ore robot costs "),
        nom::character::complete::i32,
        tag(" ore. "),
    ))(input)?;
    Ok((input, Cost::Ore(ore)))
}

fn parse_clay(input: &str) -> IResult<&str, Cost> {
    let (input, (_, clay, _)) = tuple((
        tag("Each clay robot costs "),
        nom::character::complete::i32,
        tag(" ore. "),
    ))(input)?;
    Ok((input, Cost::Clay(clay)))
}

fn parse_obsidian(input: &str) -> IResult<&str, Cost> {
    let (input, (_, ore, _, clay, _)) = tuple((
        tag("Each obsidian robot costs "),
        nom::character::complete::i32,
        tag(" ore and "),
        nom::character::complete::i32,
        tag(" clay. "),
    ))(input)?;
    Ok((input, Cost::Obsidian(ore, clay)))
}

fn parse_geode(input: &str) -> IResult<&str, Cost> {
    let (input, (_, ore, _, obsidian, _)) = tuple((
        tag("Each geode robot costs "),
        nom::character::complete::i32,
        tag(" ore and "),
        nom::character::complete::i32,
        tag(" obsidian."),
    ))(input)?;
    Ok((input, Cost::Geode(ore, obsidian)))
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
            costs: vec![ore, clay, obsidian, geode],
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Blueprint>> {
    let (input, blueprints) = separated_list1(newline, parse_blueprint)(input)?;
    Ok((input, blueprints))
}

fn day19a(path: &str, minutes: i32) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, blueprints) = parse(&content).unwrap();

    blueprints.par_iter().map(|blueprint| {
        let state = State::default();
        let mut queue = vec![state];
        let mut best = 0;
        let mut visited: HashSet<String> = HashSet::new();

        while !queue.is_empty() {
            let mut state = queue.pop().unwrap();
            let key = state.to_string();
            if visited.contains(&key) {
                continue;
            } else {
                visited.insert(key);
            }

            let robots = state.try_buy(blueprint);
            state.harvest();

            if state.is_done(minutes) {
                best = cmp::max(best, state.geodes);
                continue;
            }

            for robot in robots.iter() {
                match robot {
                    None => queue.push(state.clone()),
                    Some(robot) => {
                        let mut new_state = state.clone();
                        new_state.ready(robot, blueprint);
                        queue.push(new_state);
                    }
                }
            }
        }
        best * blueprint.id
    }).sum()
}

fn day19b(path: &str, minutes: i32) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, blueprints) = parse(&content).unwrap();
    let first_three = blueprints.iter().take(3).collect::<Vec<_>>();

    first_three.par_iter().map(|blueprint| {
        let state = State::default();
        let mut queue = vec![state];
        let mut visited: HashSet<String> = HashSet::new();
        let mut scores: HashMap<i32, i32> = HashMap::new();

        while !queue.is_empty() {
            let mut state = queue.pop().unwrap();
            let key = state.to_string();
            if visited.contains(&key) {
                continue;
            } else {
                visited.insert(key);
            }

            let robots = state.try_buy(blueprint);
            state.harvest();

            if state.minute > 24 && scores.contains_key(&state.minute) && scores.get(&state.minute).unwrap() > &state.geodes {
                continue;
            } else {
                scores.insert(state.minute, state.geodes);
            }

            if state.is_done(minutes) {
                continue;
            }

            for robot in robots.iter() {
                match robot {
                    None => queue.push(state.clone()),
                    Some(robot) => {
                        let mut new_state = state.clone();
                        new_state.ready(robot, blueprint);
                        queue.push(new_state);
                    }
                }
            }
        }
        scores.get(&minutes).unwrap().clone()
    }).product()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn find_quality_level() {
        let actual = day19a("./data/day19.txt", 24);
        assert_eq!(actual, 33);
    }

    #[test]
    #[ignore]
    fn find_quality_level_first_three() {
        let actual = day19b("./data/day19.txt", 32);
        assert_eq!(actual, 56 * 62);
    }

    #[test]
    #[ignore]
    fn find_quality_level_part_a() {
        let actual = day19a("./data/day19final.txt", 24);
        assert_eq!(actual, 1023);
    }

    #[test]
    #[ignore]
    fn find_quality_level_part_b() {
        let actual = day19b("./data/day19final.txt", 32);
        assert_eq!(actual, 13520);
    }
}
