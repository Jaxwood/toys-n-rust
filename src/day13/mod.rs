#![allow(dead_code)]

use std::{cell::RefCell, fs, rc::Rc};

use nom::{
    character::complete::{newline, not_line_ending},
    multi::separated_list0,
    sequence::terminated,
    IResult,
};

#[derive(Debug)]
struct Packet {
    left: Token,
    right: Token,
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(u32),
    List(Rc<RefCell<Vec<Token>>>),
}

#[derive(Debug, Clone, PartialEq)]
enum Literal {
    Number(u32),
    Open,
    Close,
}

impl Token {
    fn new() -> Token {
        Token::List(Rc::new(RefCell::new(vec![])))
    }

    fn add_token(self: &Self, token: Token) {
        match self {
            Token::Number(_) => panic!("cannot nest Token::List to Token::Number"),
            Token::List(inner) => {
                inner.borrow_mut().push(token);
            }
        }
    }
}

fn inner(candidate: Vec<Literal>) -> (Vec<Literal>, Vec<Literal>) {
    let mut brackets = 1;
    let result: Vec<_> = candidate
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(idx, next)| {
            if idx == 0 {
                return None;
            }
            if next == Literal::Open {
                brackets += 1;
            }
            if next == Literal::Close {
                brackets -= 1;
            }
            if brackets == 0 {
                None
            } else {
                Some(next)
            }
        })
        .collect();

    let rest: Vec<_> = candidate
        .clone()
        .into_iter()
        .skip(result.len() + 2)
        .collect();

    (result, rest)
}

fn visit(input: Vec<Literal>) -> usize {
    if input.len() == 0 {
        return 0;
    }
    let candidate = input.split_first();
    match candidate {
        Some((head, rest)) => match head {
            Literal::Number(num) => {
                println!("{}", num);
                visit(rest.to_vec())
            }
            Literal::Open => {
                let (i, r) = inner(input);
                visit(i);
                visit(r)
            }
            Literal::Close => visit(rest.to_vec()),
        },
        _ => panic!("empty vector"),
    }
}

fn to_tokens(input: Vec<Literal>, acc: &Token) {
    if input.len() == 0 {
        ()
    }
    let candidate = input.split_first();
    match candidate {
        Some((head, rest)) => match head {
            Literal::Number(num) => {
                let n = Token::Number(*num);
                acc.add_token(n);
                to_tokens(rest.to_vec(), acc)
            }
            Literal::Open => {
                let (i, r) = inner(input);
                let v = Token::new();
                to_tokens(i, &v);
                acc.add_token(v);
                to_tokens(r, acc);
            }
            Literal::Close => to_tokens(rest.to_vec(), acc),
        },
        _ => (),
    }
}

fn to_literals(input: &str) -> Vec<Literal> {
    input
        .chars()
        .map(|x| match x {
            '[' => Some(Literal::Open),
            ']' => Some(Literal::Close),
            _ => match x.to_digit(10) {
                Some(num) => Some(Literal::Number(num)),
                _ => None,
            },
        })
        .flatten()
        .collect()
}

fn parse(input: &str) -> IResult<&str, Packet> {
    let (input, first) = terminated(not_line_ending, newline)(input)?;
    let (input, second) = terminated(not_line_ending, newline)(input)?;
    let left = Token::new();
    let right = Token::new();
    to_tokens(to_literals(first), &left);
    to_tokens(to_literals(second), &right);
    Ok((input, Packet { left, right }))
}

fn day13a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, packets) = separated_list0(newline, parse)(content.as_str()).unwrap();
    for packet in packets {
        dbg!(packet.left);
        dbg!(packet.right);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_shortest_path() {
        let actual = day13a("./data/day13.txt");
        assert_eq!(actual, 31);
    }

    #[test]
    fn test_to_tokens() {
        let input = vec![
            Literal::Open,
            Literal::Open,
            Literal::Number(1),
            Literal::Number(2),
            Literal::Open,
            Literal::Number(3),
            Literal::Number(4),
            Literal::Close,
            Literal::Number(42),
            Literal::Close,
            Literal::Number(5),
            Literal::Number(6),
            Literal::Close,
        ];
        let actual = Token::new();
        to_tokens(input, &actual);
        let expected = Token::List(Rc::new(RefCell::new(vec![Token::List(Rc::new(
            RefCell::new(vec![
                Token::List(Rc::new(RefCell::new(vec![
                    Token::Number(1),
                    Token::Number(2),
                    Token::List(Rc::new(RefCell::new(vec![
                        Token::Number(3),
                        Token::Number(4),
                    ]))),
                    Token::Number(42),
                ]))),
                Token::Number(5),
                Token::Number(6),
            ]),
        ))])));
        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore]
    fn test_visit() {
        let actual = vec![
            Literal::Open,
            Literal::Open,
            Literal::Number(4),
            Literal::Number(4),
            Literal::Close,
            Literal::Number(4),
            Literal::Number(4),
            Literal::Close,
        ];
        let actual = visit(actual);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_split_literals() {
        let actual = vec![
            Literal::Open,
            Literal::Open,
            Literal::Number(1),
            Literal::Close,
            Literal::Close,
        ];
        let actual = inner(actual);
        let expected = (
            vec![Literal::Open, Literal::Number(1), Literal::Close],
            vec![],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_split_literals_2() {
        let actual = vec![
            Literal::Open,
            Literal::Open,
            Literal::Number(4),
            Literal::Number(4),
            Literal::Close,
            Literal::Number(4),
            Literal::Number(4),
            Literal::Close,
        ];
        let actual = inner(actual);
        let expected = (
            vec![
                Literal::Open,
                Literal::Number(4),
                Literal::Number(4),
                Literal::Close,
                Literal::Number(4),
                Literal::Number(4),
            ],
            vec![],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_split_literals_3() {
        let actual = vec![
            Literal::Open,
            Literal::Number(4),
            Literal::Number(4),
            Literal::Close,
            Literal::Number(4),
            Literal::Number(4),
        ];
        let actual = inner(actual);
        let expected = (
            vec![Literal::Number(4), Literal::Number(4)],
            vec![Literal::Number(4), Literal::Number(4)],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_add_token() {
        let actual = Token::new();
        actual.add_token(Token::Number(42));
        assert_eq!(
            actual,
            Token::List(Rc::new(RefCell::new(vec![Token::Number(42)])))
        );
    }
}
