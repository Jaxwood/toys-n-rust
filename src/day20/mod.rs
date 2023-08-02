#![allow(dead_code)]

use std::fs;

use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, numbers) = separated_list1(newline, complete::i64)(input)?;
    Ok((input, numbers))
}

fn day20a(path: &str) -> i64 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, numbers) = parse(&content).unwrap();

    let mut indexes = (0..numbers.len()).collect::<Vec<_>>();
    for (idx, &num) in numbers.iter().enumerate() {
        let pos = indexes.iter().position(|&i| i == idx).unwrap();
        indexes.remove(pos);
        let new_idx = (pos as i64 + num).rem_euclid(indexes.len() as i64) as usize;
        indexes.insert(new_idx, idx);
    }

    let zero_idx = numbers.iter().position(|&i| i == 0).unwrap();
    let zero = indexes.iter().position(|&i| i == zero_idx).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| numbers[indexes[(zero + i) % indexes.len()]])
        .sum()
}

fn day20b(path: &str) -> i64 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, numbers) = parse(&content).unwrap();
    let numbers = numbers.iter().map(|&i| i * 811589153).collect::<Vec<_>>();

    let mut indexes = (0..numbers.len()).collect::<Vec<_>>();
    for _ in 0..10 {
        for (idx, &num) in numbers.iter().enumerate() {
            let pos = indexes.iter().position(|&i| i == idx).unwrap();
            indexes.remove(pos);
            let new_idx = (pos as i64 + num).rem_euclid(indexes.len() as i64) as usize;
            indexes.insert(new_idx, idx);
        }
    }

    let zero_idx = numbers.iter().position(|&i| i == 0).unwrap();
    let zero = indexes.iter().position(|&i| i == zero_idx).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| numbers[indexes[(zero + i) % indexes.len()]])
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_groove_coordinates() {
        let actual = day20a("./data/day20.txt");
        assert_eq!(actual, 3);
    }

    #[test]
    fn find_groove_coordinates_part_a() {
        let actual = day20a("./data/day20final.txt");
        // not correct
        assert_eq!(actual, 2275);
    }

    #[test]
    fn find_groove_coordinates_with_decryption_key() {
        let actual = day20b("./data/day20.txt");
        assert_eq!(actual, 1623178306);
    }

    #[test]
    fn find_groove_coordinates_part_b() {
        let actual = day20b("./data/day20final.txt");
        // not correct
        assert_eq!(actual, 4090409331120);
    }
}
