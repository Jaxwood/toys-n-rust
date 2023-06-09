use std::collections::HashSet;
use std::fs;
use std::iter;

use nom::{
    character::complete::{alpha0, digit0, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone)]
enum Move {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, (dir, times)) = separated_pair(alpha0, space1, digit0)(input)?;
    match times.parse::<usize>() {
        Ok(amount) => Ok((
            input,
            match dir {
                "R" => Move::Right(amount),
                "L" => Move::Left(amount),
                "U" => Move::Up(amount),
                "D" => Move::Down(amount),
                _ => panic!("non supported case"),
            },
        )),
        _ => panic!("could not parse file"),
    }
}

fn parse(input: &str) -> Vec<Move> {
    let result = separated_list1(newline, parse_move)(input);
    match result {
        Ok((_, moves)) => moves,
        _ => panic!(""),
    }
}

fn move_head(m: &Move, (x, y): (i32, i32)) -> (i32, i32) {
    match m {
        Move::Right(_) => (x + 1, y),
        Move::Left(_) => (x - 1, y),
        Move::Up(_) => (x, y + 1),
        Move::Down(_) => (x, y - 1),
    }
}

fn move_tail((x, y): (i32, i32), (xx, yy): (i32, i32)) -> (i32, i32) {
    // move right
    if x > xx + 1 && y == yy {
        return (xx + 1, yy);
    }
    // move left
    if x < xx - 1 && y == yy {
        return (xx - 1, yy);
    }
    // move up
    if y > yy + 1 && x == xx {
        return (x, yy + 1);
    }
    // move down
    if y < yy - 1 && x == xx {
        return (xx, yy - 1);
    }
    if y != yy && x != xx {
        if x > xx && y > yy {
            return (xx + 1, yy + 1);
        }
        if x < xx && y > yy {
            return (xx - 1, yy + 1);
        }
        if x > xx && y < yy {
            return (xx + 1, yy - 1);
        }
        if x < xx && y < yy {
            return (xx - 1, yy - 1);
        }
    }
    (xx, yy)
}

fn get_times(m: &Move) -> &usize {
    match m {
        Move::Right(times) => times,
        Move::Left(times) => times,
        Move::Up(times) => times,
        Move::Down(times) => times,
    }
}

fn next_to(head: (i32, i32), (xx, yy): (i32, i32)) -> bool {
    let surrounding = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];

    let neighbours: Vec<(i32, i32)> = surrounding
        .iter()
        .map(|(nx, ny)| (xx + nx, yy + ny))
        .collect();

    return neighbours.iter().any(|&neighbour| head == neighbour);
}

fn day09a(path: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::from([tail]);

    let content = fs::read_to_string(path).expect("file not found");
    let moves = parse(content.as_str());

    for m in moves.iter() {
        let times = get_times(m);
        for _ in 0..(*times) {
            head = move_head(m, head);
            if !next_to(head, tail) {
                tail = move_tail(head, tail);
                visited.insert(tail);
            }
        }
    }
    visited.len()
}

fn get_change(m: Move) -> (i32, i32) {
    match m {
        Move::Right(_) => (1, 0),
        Move::Left(_) => (-1, 0),
        Move::Up(_) => (0, 1),
        Move::Down(_) => (0, -1),
    }
}

fn day09b(path: &str) -> usize {
    let rope: Vec<(i32, i32)> = iter::repeat((0, 0)).take(10).collect();
    let content = fs::read_to_string(path).expect("file not found");
    let moves = parse(content.as_str());
    moves
        .iter()
        .flat_map(|m| match m {
            Move::Right(time) => iter::repeat(Move::Right(1)).take(*time),
            Move::Left(time) => iter::repeat(Move::Left(1)).take(*time),
            Move::Up(time) => iter::repeat(Move::Up(1)).take(*time),
            Move::Down(time) => iter::repeat(Move::Down(1)).take(*time),
        })
        .fold(vec![rope], |mut acc, m| {
            let (x, y) = get_change(m);
            let cur = acc.last().unwrap();
            let result = cur
                .iter()
                .enumerate()
                .fold(vec![], |mut inner, (idx, tail)| {
                    if idx == 0 {
                        inner.push((x + tail.0, y + tail.1));
                    } else {
                        let head = inner.last().unwrap();
                        if !next_to(*head, *tail) {
                            inner.push(move_tail(*head, *tail));
                        } else {
                            inner.push(*tail);
                        }
                    }
                    inner
                });
            acc.push(result);
            acc
        })
        .iter()
        .fold(HashSet::new(), |mut acc, rs| {
            acc.insert(rs.iter().last().unwrap());
            acc
        }).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_visible_trees() {
        let result = day09a("./data/day09.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn find_visible_trees_parta() {
        let result = day09a("./data/day09final.txt");
        assert_eq!(result, 6087);
    }

    #[test]
    fn find_visible_trees_partb() {
        let result = day09b("./data/day09.txt");
        assert_eq!(result, 1);
    }

    #[test]
    fn find_visible_trees_final_partb() {
        let result = day09b("./data/day09final.txt");
        assert_eq!(result, 2493);
    }

    #[test]
    fn move_tail_right_test() {
        let actual = move_tail((2, 0), (0, 0));
        let expected = (1, 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn move_tail_left_test() {
        let actual = move_tail((2, 0), (4, 0));
        let expected = (3, 0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn move_tail_up_test() {
        let actual = move_tail((0, 2), (0, 0));
        let expected = (0, 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn move_tail_down_test() {
        let actual = move_tail((0, 2), (0, 4));
        let expected = (0, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn move_tail_up_right_test() {
        let actual = move_tail((1, 2), (0, 0));
        let expected = (1, 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn move_tail_up_left_test() {
        let actual = move_tail((-1, 2), (0, 0));
        let expected = (-1, 1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn move_tail_down_right_test() {
        let actual = move_tail((1, -2), (0, 0));
        let expected = (1, -1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn move_tail_down_left_test() {
        let actual = move_tail((-1, -2), (0, 0));
        let expected = (-1, -1);
        assert_eq!(expected, actual);
    }
}
