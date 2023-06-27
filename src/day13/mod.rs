#![allow(dead_code)]

use std::fs;

use nom::{
    character::complete::{newline, not_line_ending},
    multi::separated_list0,
    sequence::terminated,
    IResult,
};

#[derive(Debug)]
struct Packet {
    left: String,
    right: String,
}

fn parse(input: &str) -> IResult<&str, Packet> {
    let (input, first) = terminated(not_line_ending, newline)(input)?;
    let (input, second) = terminated(not_line_ending, newline)(input)?;
    Ok((
        input,
        Packet {
            left: first.to_string(),
            right: second.to_string(),
        },
    ))
}

fn day13a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let packets = separated_list0(newline, parse)(content.as_str());
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pairs_in_right_order() {
        let actual = day13a("./data/day13.txt");
        assert_eq!(actual, 13);
    }
}
