#![allow(dead_code)]
use std::fs;

use nom::{
    branch::{alt},
    bytes::complete::{tag},
    character::complete::{alpha0, newline, digit0, space1, not_line_ending},
    multi::{separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Node {}

#[derive(Debug)]
enum InstructionSet<'a> {
    CdCmd(&'a str),
    LsCmd,
    DirCmd(&'a str),
    Dir(&'a str, usize),
    File(&'a str, usize),
}

fn parse_cd_cmd(input: &str) -> IResult<&str, InstructionSet> {
    let (input, c) = preceded(tag("$ cd "), alt((tag("/"), tag(".."), alpha0)))(input)?;
    Ok((input, InstructionSet::CdCmd(c)))
}

fn parse_ls_cmd(input: &str) -> IResult<&str, InstructionSet> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, InstructionSet::LsCmd))
}

fn parse_dir_cmd(input: &str) -> IResult<&str, InstructionSet> {
    let (input, c) = preceded(tag("dir "), alpha0)(input)?;
    Ok((input, InstructionSet::DirCmd(c)))
}

fn parse_dir(input: &str) -> IResult<&str, InstructionSet> {
    let (input, (c,d)) = separated_pair(digit0, space1, alpha0)(input)?;
    match c.parse::<usize>() {
        Ok(size) => Ok((input, InstructionSet::Dir(d, size))),
        _ => panic!("could not parse dir")
    }
}

fn parse_file(input: &str) -> IResult<&str, InstructionSet> {
    let (input, (c,e)) = separated_pair(digit0, space1, not_line_ending)(input)?;
    match c.parse::<usize>() {
        Ok(size) => Ok((input, InstructionSet::File(e, size))),
        _ => panic!("could not parse file"),
    }
}

fn parse(input: &str) -> IResult<&str, Vec<InstructionSet>> {
    let (input, moves) = separated_list1(newline, alt((parse_cd_cmd, parse_ls_cmd, parse_dir_cmd, parse_file, parse_dir)))(input)?;
    Ok((input, moves))
}

fn day07(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let instructions = parse(content.as_str());
    dbg!(instructions);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_total_size() {
        let result = day07("./data/day07.txt");
        assert_eq!(result, 95437);
    }
}
