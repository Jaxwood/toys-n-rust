#![allow(dead_code)]

use std::fs;

use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, numbers) = separated_list1(newline, complete::i32)(input)?;
    Ok((input, numbers))
}

fn mod_floor(a: i32, n: i32) -> i32 {
    return ((a % n) + n) % n;
}

fn new_index(len: i32, idx: i32, offset: i32) -> i32 {
    let new_idx = idx + offset;
    if new_idx <= 0 {
        return mod_floor(new_idx - 1, len);
    }
    if new_idx >= len {
        return (new_idx + 1) % len;
    }
    return new_idx;
}

fn update(acc: &mut Vec<i32>, from: usize, to: usize) {
    if from == to {
        return;
    }
    let num = acc.remove(from);
    acc.insert(to, num);
}

fn day20a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, numbers) = parse(&content).unwrap();
    let result = numbers.iter().fold(numbers.clone(), |mut acc, num| {
        let idx = acc.iter().position(|x| x == num).unwrap() as i32;
        let new_idx = new_index(acc.len() as i32, idx, *num);
        update(&mut acc, idx as usize, new_idx as usize);
        acc
    });
    let zero_idx = result.iter().position(|x| x == &0).unwrap();
    let one_thousand_idx = (zero_idx + 1000) % result.len();
    let two_thousand_idx = (zero_idx + 2000) % result.len();
    let three_thousand_idx = (zero_idx + 3000) % result.len();
    result[one_thousand_idx] + result[two_thousand_idx] + result[three_thousand_idx]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_new_index_a() {
        let actual = new_index(7, 0, 1);
        assert_eq!(actual, 1);
    }

    #[test]
    fn find_new_index_b() {
        let actual = new_index(7, 0, 2);
        assert_eq!(actual, 2);
    }

    #[test]
    fn find_new_index_c() {
        let actual = new_index(7, 1, -3);
        assert_eq!(actual, 4);
    }

    #[test]
    fn find_new_index_d() {
        let actual = new_index(7, 2, 3);
        assert_eq!(actual, 5);
    }

    #[test]
    fn find_new_index_e() {
        let actual = new_index(7, 2, -2);
        assert_eq!(actual, 6);
    }

    #[test]
    fn find_new_index_f() {
        let actual = new_index(7, 3, 0);
        assert_eq!(actual, 3);
    }

    #[test]
    fn find_new_index_g() {
        let actual = new_index(7, 5, 4);
        assert_eq!(actual, 3);
    }

    #[test]
    fn update_a() {
        let mut actual = vec![1, 2, -3, 3, -2, 0, 4];
        update(&mut actual, 0, 1);
        let expected = vec![2, 1, -3, 3, -2, 0, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_b() {
        let mut actual = vec![2, 1, -3, 3, -2, 0, 4];
        update(&mut actual, 0, 2);
        let expected = vec![1, -3, 2, 3, -2, 0, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_c() {
        let mut actual = vec![1, -3, 2, 3, -2, 0, 4];
        update(&mut actual, 1, 4);
        let expected = vec![1, 2, 3, -2, -3, 0, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_d() {
        let mut actual = vec![1, 2, 3, -2, -3, 0, 4];
        update(&mut actual, 2, 5);
        let expected = vec![1, 2, -2, -3, 0, 3, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_e() {
        let mut actual = vec![1, 2, -2, -3, 0, 3, 4];
        update(&mut actual, 2, 6);
        let expected = vec![1, 2, -3, 0, 3, 4, -2];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_f() {
        let mut actual = vec![1, 2, -3, 0, 3, 4, -2];
        update(&mut actual, 3, 3);
        let expected = vec![1, 2, -3, 0, 3, 4, -2];
        assert_eq!(actual, expected);
    }

    #[test]
    fn update_g() {
        let mut actual = vec![1, 2, -3, 0, 3, 4, -2];
        update(&mut actual, 5, 3);
        let expected = vec![1, 2, -3, 4, 0, 3, -2];
        assert_eq!(actual, expected);
    }

    #[test]
    fn find_groove_coordinates() {
        let actual = day20a("./data/day20.txt");
        assert_eq!(actual, 3);
    }

    #[test]
    fn find_groove_coordinates_part_a() {
        let actual = day20a("./data/day20final.txt");
        // not correct
        assert_eq!(actual, -14281);
    }
}
