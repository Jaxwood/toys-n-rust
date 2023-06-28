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
    left: Vec<Token>,
    right: Vec<Token>,
}

#[derive(Debug, PartialEq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Comma,
    Number(u32),
}

fn tokenize(input: &str) -> Vec<Token> {
    input
        .chars()
        .map(|c| match c {
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            ',' => Token::Comma,
            _ => Token::Number(c.to_digit(10).unwrap()),
        })
        .filter(|t| match t {
            Token::Comma => false,
            _ => true,
        })
        .collect()
}

fn parse(input: &str) -> IResult<&str, Packet> {
    let (input, first) = terminated(not_line_ending, newline)(input)?;
    let (input, second) = terminated(not_line_ending, newline)(input)?;
    Ok((
        input,
        Packet {
            left: tokenize(first),
            right: tokenize(second),
        },
    ))
}

fn day13a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let packets = separated_list0(newline, parse)(content.as_str());
    dbg!(packets);
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

    #[test]
    fn test_tokenizer() {
        let actual = tokenize("[[1,2],3,4]");
        let expected = vec![
            Token::OpenBracket,
            Token::OpenBracket,
            Token::Number(1),
            Token::Number(2),
            Token::CloseBracket,
            Token::Number(3),
            Token::Number(4),
            Token::CloseBracket,
        ];
        assert_eq!(actual, expected);
    }
}
