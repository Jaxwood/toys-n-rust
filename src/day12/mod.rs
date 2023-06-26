#![allow(dead_code)]

use std::{collections::HashMap, fs};

use nom::{
    character::complete::{alpha1, newline},
    multi::separated_list1,
    IResult,
};

fn adjust_values(val: u32) -> u32 {
    // make S = 96
    if val == 69 {
        96
    // make E = 123
    } else if val == 83 {
        123
    } else {
        val
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, line) = alpha1(input)?;
    let heights = line
        .chars()
        .into_iter()
        .map(|x| x as u32)
        .map(adjust_values)
        .collect::<Vec<u32>>();
    Ok((input, heights))
}

fn parse(input: &str) -> HashMap<(usize, usize), u32> {
    let lines = separated_list1(newline, parse_line)(input);
    match lines {
        Ok((_, ys)) => ys.iter().enumerate().fold(HashMap::new(), |acc, (y, ls)| {
            ls.iter().enumerate().fold(acc, |mut acc2, (x, &val)| {
                acc2.insert((x, y), val);
                acc2
            })
        }),
        _ => panic!("could not parse lines"),
    }
}

fn day12a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    dbg!(parse(content.as_str()));
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_shortest_path() {
        let actual = day12a("./data/day12.txt");
        assert_eq!(actual, 31);
    }
}
