#![allow(dead_code)]

use std::{cmp, fs, iter::once};

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1, sequence::tuple,
    IResult,
};

#[derive(Debug, Clone)]
struct State {
    minute: i32,
    robots: Vec<Robot>,
    ores: i32,
    clays: i32,
    obsidians: i32,
    geodes: i32,
}

impl Default for State {
    fn default() -> Self {
        State {
            minute: 0,
            robots: vec![Robot::Ore],
            ores: 0,
            clays: 0,
            obsidians: 0,
            geodes: 0,
        }
    }
}

impl State {
    fn is_done(&self) -> bool {
        return self.minute == 24;
    }

    fn harvest(&mut self) {
        self.minute += 1;

        for robot in &self.robots {
            match robot {
                Robot::Ore => self.ores += 1,
                Robot::Clay => self.clays += 1,
                Robot::Obsidian => self.obsidians += 1,
                Robot::Geode => self.geodes += 1,
            }
        }
    }

    fn count(&self, robot: &Robot) -> i32 {
        self.robots.iter().fold(0, |acc, r| {
            match (robot, r) {
                (Robot::Ore, Robot::Ore) => acc + 1,
                (Robot::Clay, Robot::Clay) => acc + 1,
                (Robot::Obsidian, Robot::Obsidian) => acc + 1,
                (Robot::Geode, Robot::Geode) => acc + 1,
                _ => acc,
            }
        })
    }

    fn try_buy(&self, blueprint: &Blueprint) -> Vec<Option<Robot>> {
        blueprint
            .costs
            .iter()
            .filter_map(|cost| match cost {
                Cost::Ore(ore) => {
                    if self.ores >= *ore && self.count(&Robot::Ore) < 4 {
                        Some(Robot::Ore)
                    } else {
                        None
                    }
                }
                Cost::Clay(ore) => {
                    if self.ores >= *ore && self.count(&Robot::Clay) < 5 {
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
            }
        }
        self.robots.push(robot.clone());
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

fn day19a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, blueprints) = parse(&content).unwrap();
    let mut quality_level = 0;

    for blueprint in blueprints.iter().skip(1) {
        let state = State::default();
        let mut queue = vec![state];
        let mut best = 0;

        while !queue.is_empty() {
            let mut state = queue.pop().unwrap();
            let robots = state.try_buy(blueprint);
            state.harvest();

            if state.is_done() {
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
        quality_level += best * blueprint.id;
    }

    quality_level
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
