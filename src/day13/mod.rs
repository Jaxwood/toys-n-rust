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

#[derive(Debug, Clone, PartialEq)]
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

fn compare(left: Vec<Token>, right: Vec<Token>) -> bool {
    let left_pair = left.first();
    let right_pair = right.first();
    match (left_pair, right_pair) {
        (None, None) => true,
        (Some(Token::Number(l)), Some(Token::OpenBracket)) => {
            let mut rest = vec![Token::OpenBracket, Token::Number(*l), Token::CloseBracket];
            rest.extend(left.into_iter().skip(1));
            return compare(rest, right);
        }
        (Some(Token::OpenBracket), Some(Token::Number(r))) => {
            let mut rest = vec![Token::OpenBracket, Token::Number(*r), Token::CloseBracket];
            rest.extend(right.into_iter().skip(1));
            return compare(left, rest);
        }
        (Some(Token::OpenBracket), Some(Token::OpenBracket)) => compare(
            left.into_iter().skip(1).collect(),
            right.into_iter().skip(1).collect(),
        ),
        (Some(Token::CloseBracket), Some(Token::CloseBracket)) => compare(
            left.into_iter().skip(1).collect(),
            right.into_iter().skip(1).collect(),
        ),
        (Some(Token::Number(l)), Some(Token::Number(r))) => {
            if l < r {
                return true;
            } else if l > r {
                return false;
            } else {
                compare(
                    left.into_iter().skip(1).collect(),
                    right.into_iter().skip(1).collect(),
                )
            }
        }
        (Some(Token::CloseBracket), Some(_)) => compare(left.into_iter().skip(1).collect(), right),
        (Some(_), Some(Token::CloseBracket)) => compare(left, right.into_iter().skip(1).collect()),
        (None, Some(_)) => true,  // left ran out of items
        (Some(_), None) => false, // right ran out of items
        _ => panic!("unhandled case"),
    }
}

fn day13a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let packets = separated_list0(newline, parse)(content.as_str());
    match packets {
        Ok((_, ps)) => ps
            .into_iter()
            .enumerate()
            .filter_map(|(idx, packet)| {
                if compare(packet.left, packet.right) {
                    return Some(idx + 1);
                }
                return None;
            })
            .sum(),
        Err(_) => panic!("could not parse packets"),
    }
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
    fn find_pairs_in_right_order_part_a() {
        let actual = day13a("./data/day13final.txt");
        assert_eq!(actual, 0);
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

    #[test]
    fn test_comparison_1() {
        let actual = compare(tokenize("[1,1,3,1,1]"), tokenize("[1,1,5,1,1]"));
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comparison_2() {
        let actual = compare(tokenize("[[1],[2,3,4]]"), tokenize("[[1],4]"));
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comparison_3() {
        let actual = compare(tokenize("[9]"), tokenize("[[8,7,6]]"));
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comparison_4() {
        let actual = compare(tokenize("[[4,4],4,4]"), tokenize("[[4,4],4,4,4]"));
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comparison_5() {
        let actual = compare(tokenize("[7,7,7,7]"), tokenize("[7,7,7]"));
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comparison_6() {
        let actual = compare(tokenize("[]"), tokenize("[3]"));
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comparison_7() {
        let actual = compare(tokenize("[[[]]]"), tokenize("[[]]"));
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_comparison_8() {
        let actual = compare(
            tokenize("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            tokenize("[1,[2,[3,[4,[5,6,0]]]],8,9]"),
        );
        let expected = false;
        assert_eq!(actual, expected);
    }
}
